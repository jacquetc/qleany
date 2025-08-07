import {useCallback, useEffect} from 'react';
import {FileDTO, fileService} from '../services/file-service';
import {RootRelationshipField, rootService} from '../services/root-service';
import {EntityEventPayload, directAccessEventService} from '../services/direct-access-event-service.ts';
import {error, info} from '@tauri-apps/plugin-log';

import {useMutation, useQuery, useQueryClient} from '@tanstack/react-query';

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

                if (groupFilter) {
                    filteredFiles = filteredFiles.filter(file => file.group === groupFilter);
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

    // Set up event listeners for Tauri events
    useEffect(() => {
        if (!rootId) return;

        // Handler for file created events
        const handleFileCreated = (payload: EntityEventPayload) => {
            info(`File created event received: ${payload.ids}`);
            queryClient.invalidateQueries({queryKey: ['files', rootId]});
        };

        // Handler for file updated events
        const handleFileUpdated = (payload: EntityEventPayload) => {
            info(`File updated event received: ${payload.ids}`);
            queryClient.invalidateQueries({queryKey: ['files', rootId]});
        };

        // Handler for file removed events
        const handleFileRemoved = (payload: EntityEventPayload) => {
            info(`File removed event received: ${payload.ids}`);
            queryClient.invalidateQueries({queryKey: ['files', rootId]});
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

        // Cleanup function
        return () => {
            unsubscribe().catch(err => {
                error(`Error unsubscribing from events: ${err}`);
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