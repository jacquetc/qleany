import {useCallback, useEffect} from 'react';
import {CreateEntityDTO, EntityDTO, entityService} from '../services/entity-service';
import {RootRelationshipField, rootService} from '../services/root-service';
import {EntityEventPayload, directAccessEventService} from '../services/direct-access-event-service.ts';
import {error, info} from '@tauri-apps/plugin-log';

import {useMutation, useQuery, useQueryClient} from '@tanstack/react-query';
import {undoRedoService} from "#services/undo-redo-service.ts";

/**
 * Custom hook for fetching and managing entities data
 *
 * This hook uses React Query to fetch and cache entities data,
 * and subscribes to Tauri events to keep the data in sync.
 *
 * @param rootId The ID of the root entity
 */
export function useEntities(rootId: number | null) {
    const queryClient = useQueryClient();

    // Query for fetching entities
    const entitiesQuery = useQuery({
        queryKey: ['entities', rootId],
        queryFn: async () => {
            if (!rootId) return [];

            try {
                // Get entity IDs from root relationship
                const entityIds = await rootService.getRootRelationship(rootId, RootRelationshipField.Entities);

                // Get entities using the IDs
                const entities = await entityService.getEntityMulti(entityIds);

                // Filter out null entities
                const filteredEntities = entities.filter((entity): entity is EntityDTO => entity !== null);

                return filteredEntities;
            } catch (err) {
                error(`Error fetching entities: ${err}`);
                throw err;
            }
        },
        enabled: !!rootId,
        staleTime: 1000 * 60 * 5, // 5 minutes
        retry: 1
    });

    // Mutation for creating a new entity
    const createEntityMutation = useMutation({
        mutationFn: async () => {
            if (!rootId) {
                throw new Error("No root selected");
            }

            // Create entity with default values
            const dto: CreateEntityDTO = {
                name: 'New Entity',
                only_for_heritage: false,
                parent: null,
                allow_direct_access: true,
                fields: [],
                relationships: []
            };

            try {
                // Create the entity
                const newEntity = await entityService.createEntity(dto);

                // Get existing entities for the root
                const rootEntities = await rootService.getRootRelationship(rootId, RootRelationshipField.Entities);

                // Add the new entity to the root relationship
                await rootService.setRootRelationship({
                    id: rootId,
                    field: RootRelationshipField.Entities,
                    right_ids: [...rootEntities, newEntity.id],
                });

                return newEntity;
            } catch (err) {
                error(`Error creating entity: ${err}`);
                throw err;
            }
        },
        onSuccess: () => {
            // Invalidate queries to refetch data
            queryClient.invalidateQueries({queryKey: ['entities']});
            info("Entity created successfully");
        }
    });

    // Mutation for updating an entity
    const updateEntityMutation = useMutation({
        mutationFn: async (entity: EntityDTO) => {
            try {
                return await entityService.updateEntity(entity);
            } catch (err) {
                error(`Error updating entity: ${err}`);
                throw err;
            }
        },
        onSuccess: () => {
            queryClient.invalidateQueries({queryKey: ['entities']});
            info("Entity updated successfully");
        }
    });

    // Mutation for removing an entity
    const removeEntityMutation = useMutation({
        mutationFn: async (entityId: number) => {
            if (!rootId) {
                throw new Error("No root selected");
            }

            try {
                // Get existing entities for the root
                const rootEntities = await rootService.getRootRelationship(rootId, RootRelationshipField.Entities);

                // Remove the entity from the root relationship
                await rootService.setRootRelationship({
                    id: rootId,
                    field: RootRelationshipField.Entities,
                    right_ids: rootEntities.filter(id => id !== entityId),
                });

                // Remove the entity
                await entityService.removeEntity(entityId);
            } catch (err) {
                error(`Error removing entity: ${err}`);
                throw err;
            }
        },
        onSuccess: () => {
            queryClient.invalidateQueries({queryKey: ['entities']});
            info("Entity removed successfully");
        }
    });

    // Set up event listeners for Tauri events
    useEffect(() => {
        if (!rootId) return;

        // Handler for entity created events
        const handleEntityCreated = (payload: EntityEventPayload) => {
            info(`Entity created event received: ${payload.ids}`);
            queryClient.invalidateQueries({queryKey: ['entities', rootId]});
        };

        // Handler for entity updated events
        const handleEntityUpdated = (payload: EntityEventPayload) => {
            info(`Entity updated event received: ${payload.ids}`);
            queryClient.invalidateQueries({queryKey: ['entities', rootId]});
        };

        // Handler for entity removed events
        const handleEntityRemoved = (payload: EntityEventPayload) => {
            info(`Entity removed event received: ${payload.ids}`);
            queryClient.invalidateQueries({queryKey: ['entities', rootId]});
        };

        // Handler for root updated events
        const handleRootUpdated = (payload: EntityEventPayload) => {
            if (rootId && payload.ids.includes(rootId)) {
                info(`Root updated event received for root ${rootId}`);
                queryClient.invalidateQueries({queryKey: ['entities', rootId]});
            }
        };

        // Handler for reset events
        const handleReset = () => {
            info(`All reset event received`);
            queryClient.invalidateQueries({queryKey: ['entities']});
        };

        // Subscribe to entity events
        const unsubscribe = directAccessEventService.subscribeToEntityEvents({
            onCreated: handleEntityCreated,
            onUpdated: handleEntityUpdated,
            onRemoved: handleEntityRemoved,
            onReset: handleReset
        });

        // Also subscribe to root updates
        const unsubscribeRoot = directAccessEventService.subscribeToRootEvents({
            onUpdated: handleRootUpdated
        });

        const unsubscribeUndoRedo = undoRedoService.subscribeToUndoRedoEvents({
            onUndone: () => queryClient.invalidateQueries({queryKey: ['entities', rootId]}),
            onRedone: () => queryClient.invalidateQueries({queryKey: ['entities', rootId]}),
        });

        // Cleanup function
        return () => {
            unsubscribe().catch(err => {
                error(`Error unsubscribing from entity events: ${err}`);
            });
            unsubscribeRoot().catch(err => {
                error(`Error unsubscribing from root events: ${err}`);
            });
            unsubscribeUndoRedo().catch(err => {
                error(`Error unsubscribing from undo redo events: ${err}`);
            });
        };
    }, [rootId, queryClient]);

    // Function to create a new entity
    const createEntity = useCallback(() => {
        createEntityMutation.mutate();
    }, [createEntityMutation]);

    // Function to update an entity
    const updateEntity = useCallback((entity: EntityDTO) => {
        updateEntityMutation.mutate(entity);
    }, [updateEntityMutation]);

    // Function to remove an entity
    const removeEntity = useCallback((entityId: number) => {
        removeEntityMutation.mutate(entityId);
    }, [removeEntityMutation]);

    return {
        entities: entitiesQuery.data || [],
        isLoading: entitiesQuery.isLoading,
        error: entitiesQuery.error,
        createEntity,
        updateEntity,
        removeEntity,
        refetch: entitiesQuery.refetch
    };
}