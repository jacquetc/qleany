import {useCallback, useEffect} from 'react';
import {GlobalDTO, CreateGlobalDTO, globalService} from '../services/global-service';
import {RootRelationshipField, rootService} from '../services/root-service';
import {EntityEventPayload, directAccessEventService} from '../services/direct-access-event-service.ts';
import {error, info} from '@tauri-apps/plugin-log';

import {useMutation, useQuery, useQueryClient} from '@tanstack/react-query';

/**
 * Custom hook for fetching and managing global configuration data
 *
 * This hook uses React Query to fetch and cache global data,
 * and provides functions for creating, updating, and removing global configurations.
 *
 * @param rootId The ID of the root entity
 */
export function useGlobal(rootId: number | null) {
    const queryClient = useQueryClient();

    // Query for fetching global configuration
    const globalQuery = useQuery({
        queryKey: ['global', rootId],
        queryFn: async () => {
            if (!rootId) return null;

            try {
                // Get global ID from root relationship
                const globalIds = await rootService.getRootRelationship(rootId, RootRelationshipField.Global);

                if (globalIds.length === 0) {
                    return null;
                }

                // Get global configuration using the ID
                const globalData = await globalService.getGlobal(globalIds[0]);
                return globalData;
            } catch (err) {
                error(`Error fetching global configuration: ${err}`);
                throw err;
            }
        },
        enabled: !!rootId,
        staleTime: 1000 * 60 * 5, // 5 minutes
        retry: 1
    });

    // Mutation for creating a new global configuration
    const createGlobalMutation = useMutation({
        mutationFn: async (dto: CreateGlobalDTO) => {
            if (!rootId) {
                throw new Error("No root selected");
            }

            try {
                // Create the global configuration
                const newGlobal = await globalService.createGlobal(dto);

                // Set the global relationship on the root
                await rootService.setRootRelationship({
                    id: rootId,
                    field: RootRelationshipField.Global,
                    right_ids: [newGlobal.id],
                });

                return newGlobal;
            } catch (err) {
                error(`Error creating global configuration: ${err}`);
                throw err;
            }
        },
        onSuccess: () => {
            // Invalidate queries to refetch data
            queryClient.invalidateQueries({queryKey: ['global']});
            info("Global configuration created successfully");
        }
    });

    // Mutation for updating a global configuration
    const updateGlobalMutation = useMutation({
        mutationFn: async (global: GlobalDTO) => {
            try {
                return await globalService.updateGlobal(global);
            } catch (err) {
                error(`Error updating global configuration: ${err}`);
                throw err;
            }
        },
        onSuccess: () => {
            queryClient.invalidateQueries({queryKey: ['global']});
            info("Global configuration updated successfully");
        }
    });

    // Mutation for removing a global configuration
    const removeGlobalMutation = useMutation({
        mutationFn: async (globalId: number) => {
            if (!rootId) {
                throw new Error("No root selected");
            }

            try {
                // Clear the global relationship on the root
                await rootService.setRootRelationship({
                    id: rootId,
                    field: RootRelationshipField.Global,
                    right_ids: [],
                });

                // Remove the global configuration
                await globalService.removeGlobal(globalId);
            } catch (err) {
                error(`Error removing global configuration: ${err}`);
                throw err;
            }
        },
        onSuccess: () => {
            queryClient.invalidateQueries({queryKey: ['global']});
            info("Global configuration removed successfully");
        }
    });

    // Set up event listeners for Tauri events
    useEffect(() => {
        if (!rootId) return;

        // Handler for global created events
        const handleGlobalCreated = (payload: EntityEventPayload) => {
            info(`Global created event received: ${payload.ids}`);
            queryClient.invalidateQueries({queryKey: ['global', rootId]});
        };

        // Handler for global updated events
        const handleGlobalUpdated = (payload: EntityEventPayload) => {
            info(`Global updated event received: ${payload.ids}`);
            queryClient.invalidateQueries({queryKey: ['global', rootId]});
        };

        // Handler for global removed events
        const handleGlobalRemoved = (payload: EntityEventPayload) => {
            info(`Global removed event received: ${payload.ids}`);
            queryClient.invalidateQueries({queryKey: ['global', rootId]});
        };

        // Handler for reset events
        const handleReset = () => {
            info(`All reset event received`);
            queryClient.invalidateQueries({queryKey: ['global']});
        };

        // Subscribe to global events
        const unsubscribe = directAccessEventService.subscribeToGlobalEvents({
            onCreated: handleGlobalCreated,
            onUpdated: handleGlobalUpdated,
            onRemoved: handleGlobalRemoved,
            onReset: handleReset
        });

        // Cleanup function
        return () => {
            unsubscribe().catch(err => {
                error(`Error unsubscribing from global events: ${err}`);
            });
        };
    }, [rootId, queryClient]);

    // Function to create a new global configuration
    const createGlobal = useCallback((dto: CreateGlobalDTO) => {
        createGlobalMutation.mutate(dto);
    }, [createGlobalMutation]);

    // Function to update a global configuration
    const updateGlobal = useCallback((global: GlobalDTO) => {
        updateGlobalMutation.mutate(global);
    }, [updateGlobalMutation]);

    // Function to remove a global configuration
    const removeGlobal = useCallback((globalId: number) => {
        removeGlobalMutation.mutate(globalId);
    }, [removeGlobalMutation]);

    return {
        global: globalQuery.data,
        isLoading: globalQuery.isLoading,
        error: globalQuery.error,
        createGlobal,
        updateGlobal,
        removeGlobal,
        refetch: globalQuery.refetch
    };
}