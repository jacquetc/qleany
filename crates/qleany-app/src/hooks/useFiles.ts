import {useCallback, useEffect, useRef} from 'react';
import {FileDTO, fileService} from '../services/file-service';
import {RootRelationshipField, rootService} from '../services/root-service';
import {directAccessEventService, EntityEventPayload} from '../services/direct-access-event-service.ts';
import {error, info} from '@tauri-apps/plugin-log';

import {useMutation, useQuery, useQueryClient} from '@tanstack/react-query';
import {undoRedoService} from "#services/undo-redo-service.ts";

/**
 * Custom hook for fetching and managing files data
 *
 * This hook uses React Query to fetch and cache files data,
 * and subscribes to Tauri events to keep the data in sync.
 *
 * @param rootId The ID of the root entity
 * @param groupFilter Optional filter for files by group
 */
export function useFiles(rootId: number | null, groupFilter?: string) {

    const queryClient = useQueryClient();

    // Query for fetching files
    const filesQuery = useQuery({
        queryKey: ['files', rootId, groupFilter],
        queryFn: async () => {
            if (!rootId) return [];

            try {
                // Get file IDs from root relationship
                const fileIds = await rootService.getRootRelationship(rootId, RootRelationshipField.Files);

                // Get files using the IDs
                const files = await fileService.getFileMulti(fileIds);

                // Filter out null files and apply group filter
                let filteredFiles = files.filter((file): file is FileDTO => file !== null);
                info(`useFiles: Retrieved ${filteredFiles.length} files for rootId=${rootId}`);

                if (groupFilter) {
                    const beforeFilter = filteredFiles.length;
                    filteredFiles = filteredFiles.filter(file => file.group === groupFilter);
                    info(`useFiles: Filtered by group '${groupFilter}': ${beforeFilter} -> ${filteredFiles.length} files`);
                }

                return filteredFiles;
            } catch (err) {
                error(`Error fetching files: ${err}`);
                throw err;
            }
        },
        enabled: !!rootId,
        staleTime: 1000 * 60 * 5, // 5 minutes
        retry: 1
    });

    // Mutation for creating a new file
    const createFileMutation = useMutation({
        mutationFn: async () => {
            if (!rootId) {
                throw new Error("No root selected");
            }

            // Create file with default values
            const dto = {
                name: 'New File',
                relative_path: '',
                group: groupFilter || "New Group",
            };

            try {
                // Create the file
                const newFile = await fileService.createFile(dto);

                // Get existing files for the root
                const rootFiles = await rootService.getRootRelationship(rootId, RootRelationshipField.Files);

                // Add the new file to the root relationship
                await rootService.setRootRelationship({
                    id: rootId,
                    field: RootRelationshipField.Files,
                    right_ids: [...rootFiles, newFile.id],
                });

                return newFile;
            } catch (err) {
                error(`Error creating file: ${err}`);
                throw err;
            }
        },
        onSuccess: () => {
            // Invalidate queries to refetch data
            queryClient.invalidateQueries({queryKey: ['files']});
            info("File created successfully");
        }
    });

    // Debounce ref for query invalidation
    const invalidationTimeoutRef = useRef<NodeJS.Timeout | null>(null);

    // Debounced invalidation function
    const debouncedInvalidateQueries = useCallback(() => {
        if (invalidationTimeoutRef.current) {
            clearTimeout(invalidationTimeoutRef.current);
        }
        
        invalidationTimeoutRef.current = setTimeout(() => {
            queryClient.invalidateQueries({queryKey: ['files', rootId]});
            invalidationTimeoutRef.current = null;
        }, 100); // 100ms debounce
    }, [queryClient, rootId]);

    // Set up event listeners for Tauri events
    useEffect(() => {
        if (!rootId) return;

        // Handler for file created events
        const handleFileCreated = (payload: EntityEventPayload) => {
            info(`File created event received: ${payload.ids}`);
            debouncedInvalidateQueries();
        };

        // Handler for file updated events
        const handleFileUpdated = (payload: EntityEventPayload) => {
            info(`File updated event received: ${payload.ids}`);
            debouncedInvalidateQueries();
        };

        // Handler for file removed events
        const handleFileRemoved = (payload: EntityEventPayload) => {
            info(`File removed event received: ${payload.ids}`);
            debouncedInvalidateQueries();
        };

        // Handler for reset events
        const handleReset = () => {
            info(`All reset event received`);
            queryClient.invalidateQueries({queryKey: ['files']});
        };

        // Subscribe to events
        const unsubscribe = directAccessEventService.subscribeToFileEvents({
            onCreated: handleFileCreated,
            onUpdated: handleFileUpdated,
            onRemoved: handleFileRemoved,
            onReset: handleReset
        });

        const unsubscribeUndoRedo = undoRedoService.subscribeToUndoRedoEvents({
            onUndone: () => queryClient.invalidateQueries({queryKey: ['files', rootId]}),
            onRedone: () => queryClient.invalidateQueries({queryKey: ['files', rootId]}),
        });

        // Cleanup function
        return () => {
            // Clear any pending timeout
            if (invalidationTimeoutRef.current) {
                clearTimeout(invalidationTimeoutRef.current);
                invalidationTimeoutRef.current = null;
            }
            
            unsubscribe().catch(err => {
                error(`Error unsubscribing from events: ${err}`);
            });
            unsubscribeUndoRedo().catch(err => {
                error(`Error unsubscribing from undo redo events: ${err}`);
            });
        };
    }, [rootId, queryClient]);

    // Function to create a new file
    const createFile = useCallback(() => {
        createFileMutation.mutate();
    }, [createFileMutation]);

    return {
        files: filesQuery.data || [],
        isLoading: filesQuery.isLoading,
        queryError: filesQuery.error,
        createFile,
        refetch: filesQuery.refetch
    };
}