import {useCallback, useEffect} from 'react';
import {
    RelationshipDTO,
    CreateRelationshipDTO,
    RelationshipType,
    Strength,
    Direction,
    Cardinality,
    Order,
    relationshipService
} from '../services/relationship-service';
import {EntityDTO, EntityRelationshipField, entityService} from '../services/entity-service';
import {EntityEventPayload, directAccessEventService} from '../services/direct-access-event-service.ts';
import {RootRelationshipField, rootService} from '../services/root-service';
import {error, info} from '@tauri-apps/plugin-log';

import {useMutation, useQuery, useQueryClient} from '@tanstack/react-query';

/**
 * Custom hook for fetching and managing relationships data for a specific entity
 *
 * This hook uses React Query to fetch and cache relationships data,
 * and subscribes to Tauri events to keep the data in sync.
 *
 * @param entityId The ID of the entity
 */
export function useRelationships(entityId: number | null) {
    const queryClient = useQueryClient();

    // Query for fetching relationships
    const relationshipsQuery = useQuery({
        queryKey: ['relationships', entityId],
        queryFn: async () => {
            try {
                if (entityId) {
                    // Get relationship IDs for a specific entity
                    const relationshipIds = await entityService.getEntityRelationship(entityId, EntityRelationshipField.Relationships);

                    // Get relationships using the IDs
                    const relationships = await relationshipService.getRelationshipMulti(relationshipIds);

                    // Filter out null relationships
                    return relationships.filter((relationship): relationship is RelationshipDTO => relationship !== null);
                } else {
                    // When entityId is null, we need to fetch all relationships from all entities
                    // First, get all entities
                    const allEntitiesQuery = await queryClient.fetchQuery({
                        queryKey: ['entities'],
                        queryFn: async () => {
                            // This assumes there's a way to get all entities
                            const rootId = 1; // Assuming root ID is 1, adjust as needed
                            const entityIds = await rootService.getRootRelationship(rootId, RootRelationshipField.Entities);
                            const entities = await entityService.getEntityMulti(entityIds);
                            return entities.filter((entity): entity is EntityDTO => entity !== null);
                        }
                    });

                    // Now collect all relationship IDs from all entities
                    const allRelationshipIdsSet = new Set<number>();

                    // For each entity, get its relationships
                    for (const entity of allEntitiesQuery) {
                        if (entity && entity.id) {
                            try {
                                const relationshipIds = await entityService.getEntityRelationship(entity.id, EntityRelationshipField.Relationships);
                                relationshipIds.forEach(id => allRelationshipIdsSet.add(id));
                            } catch (err) {
                                error(`Error fetching relationships for entity ${entity.id}: ${err}`);
                                // Continue with other entities even if one fails
                            }
                        }
                    }

                    // Convert Set to Array
                    const allRelationshipIds = Array.from(allRelationshipIdsSet);

                    // Get all relationships using the collected IDs
                    if (allRelationshipIds.length > 0) {
                        const relationships = await relationshipService.getRelationshipMulti(allRelationshipIds);
                        return relationships.filter((relationship): relationship is RelationshipDTO => relationship !== null);
                    }

                    return [];
                }
            } catch (err) {
                error(`Error fetching relationships: ${err}`);
                throw err;
            }
        },
        // Always enabled, even when entityId is null
        enabled: true,
        staleTime: 1000 * 60 * 5, // 5 minutes
        retry: 1
    });

    // Mutation for creating a new relationship
    const createRelationshipMutation = useMutation({
        mutationFn: async (relationshipData: {
            fieldName: string;
            relationshipType: RelationshipType;
            rightEntityId: number;
            strength: Strength;
            direction: Direction;
            cardinality: Cardinality;
            order: Order | null;
        } = {
            fieldName: 'New Relationship',
            relationshipType: RelationshipType.OneToMany,
            rightEntityId: 0,
            strength: Strength.Strong,
            direction: Direction.Forward,
            cardinality: Cardinality.ZeroOrMore,
            order: null
        }) => {
            if (!entityId) {
                throw new Error("No entity selected");
            }

            // Create relationship with provided values or defaults
            const dto: CreateRelationshipDTO = {
                field_name: relationshipData.fieldName || 'New Relationship',
                relationship_type: relationshipData.relationshipType || RelationshipType.OneToMany,
                left_entity: entityId,
                right_entity: relationshipData.rightEntityId,
                strength: relationshipData.strength || Strength.Strong,
                direction: relationshipData.direction || Direction.Forward,
                cardinality: relationshipData.cardinality || Cardinality.ZeroOrMore,
                order: relationshipData.order
            };

            try {
                // Create the relationship
                const newRelationship = await relationshipService.createRelationship(dto);

                // Get existing relationships for the entity
                const entityRelationships = await entityService.getEntityRelationship(entityId, EntityRelationshipField.Relationships);

                // Add the new relationship to the entity relationship
                await entityService.setEntityRelationship({
                    id: entityId,
                    field: EntityRelationshipField.Relationships,
                    right_ids: [...entityRelationships, newRelationship.id],
                });

                return newRelationship;
            } catch (err) {
                error(`Error creating relationship: ${err}`);
                throw err;
            }
        },
        onSuccess: () => {
            // Invalidate queries to refetch data
            queryClient.invalidateQueries({queryKey: ['relationships']});
            info("Relationship created successfully");
        }
    });

    // Mutation for updating a relationship
    const updateRelationshipMutation = useMutation({
        mutationFn: async (relationship: RelationshipDTO) => {
            try {
                return await relationshipService.updateRelationship(relationship);
            } catch (err) {
                error(`Error updating relationship: ${err}`);
                throw err;
            }
        },
        onSuccess: () => {
            queryClient.invalidateQueries({queryKey: ['relationships']});
            info("Relationship updated successfully");
        }
    });

    // Mutation for removing a relationship
    const removeRelationshipMutation = useMutation({
        mutationFn: async (relationshipId: number) => {
            if (!entityId) {
                throw new Error("No entity selected");
            }

            try {
                // Get existing relationships for the entity
                const entityRelationships = await entityService.getEntityRelationship(entityId, EntityRelationshipField.Relationships);

                // Remove the relationship from the entity relationship
                await entityService.setEntityRelationship({
                    id: entityId,
                    field: EntityRelationshipField.Relationships,
                    right_ids: entityRelationships.filter(id => id !== relationshipId),
                });

                // Remove the relationship
                await relationshipService.removeRelationship(relationshipId);
            } catch (err) {
                error(`Error removing relationship: ${err}`);
                throw err;
            }
        },
        onSuccess: () => {
            queryClient.invalidateQueries({queryKey: ['relationships']});
            info("Relationship removed successfully");
        }
    });

    // Set up event listeners for Tauri events
    useEffect(() => {
        if (!entityId) return;

        // Handler for relationship created events
        const handleRelationshipCreated = (payload: EntityEventPayload) => {
            info(`Relationship created event received: ${payload.ids}`);
            queryClient.invalidateQueries({queryKey: ['relationships', entityId]});
        };

        // Handler for relationship updated events
        const handleRelationshipUpdated = (payload: EntityEventPayload) => {
            info(`Relationship updated event received: ${payload.ids}`);
            queryClient.invalidateQueries({queryKey: ['relationships', entityId]});
        };

        // Handler for relationship removed events
        const handleRelationshipRemoved = (payload: EntityEventPayload) => {
            info(`Relationship removed event received: ${payload.ids}`);
            queryClient.invalidateQueries({queryKey: ['relationships', entityId]});
        };

        // Handler for entity updated events
        const handleEntityUpdated = (payload: EntityEventPayload) => {
            if (entityId && payload.ids.includes(entityId)) {
                info(`Entity updated event received for entity ${entityId}`);
                queryClient.invalidateQueries({queryKey: ['relationships', entityId]});
            }
        };

        // Handler for reset events
        const handleReset = () => {
            info(`All reset event received`);
            queryClient.invalidateQueries({queryKey: ['relationships']});
        };

        // Subscribe to relationship events
        const unsubscribe = directAccessEventService.subscribeToRelationshipEvents({
            onCreated: handleRelationshipCreated,
            onUpdated: handleRelationshipUpdated,
            onRemoved: handleRelationshipRemoved,
            onReset: handleReset
        });

        // Also subscribe to entity updates
        const unsubscribeEntity = directAccessEventService.subscribeToEntityEvents({
            onUpdated: handleEntityUpdated
        });

        // Cleanup function
        return () => {
            unsubscribe().catch(err => {
                error(`Error unsubscribing from relationship events: ${err}`);
            });
            unsubscribeEntity().catch(err => {
                error(`Error unsubscribing from entity events: ${err}`);
            });
        };
    }, [entityId, queryClient]);

    // Function to create a new relationship
    const createRelationship = useCallback((relationshipData?: {
        fieldName: string;
        relationshipType: RelationshipType;
        rightEntityId: number;
        strength: Strength;
        direction: Direction;
        cardinality: Cardinality;
        order: Order | null;
    }) => {
        createRelationshipMutation.mutate(relationshipData);
    }, [createRelationshipMutation]);

    // Function to update a relationship
    const updateRelationship = useCallback((relationship: RelationshipDTO) => {
        updateRelationshipMutation.mutate(relationship);
    }, [updateRelationshipMutation]);

    // Function to remove a relationship
    const removeRelationship = useCallback((relationshipId: number) => {
        removeRelationshipMutation.mutate(relationshipId);
    }, [removeRelationshipMutation]);

    return {
        relationships: relationshipsQuery.data || [],
        isLoading: relationshipsQuery.isLoading,
        error: relationshipsQuery.error,
        createRelationship,
        updateRelationship,
        removeRelationship,
        refetch: relationshipsQuery.refetch
    };
}