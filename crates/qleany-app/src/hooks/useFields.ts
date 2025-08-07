import {useCallback, useEffect} from 'react';
import {FieldDTO, FieldType, CreateFieldDTO, fieldService} from '../services/field-service';
import {EntityDTO, EntityRelationshipField, entityService} from '../services/entity-service';
import {EntityEventPayload, directAccessEventService} from '../services/direct-access-event-service.ts';
import {RootRelationshipField, rootService} from '../services/root-service';
import {error, info} from '@tauri-apps/plugin-log';

import {useMutation, useQuery, useQueryClient} from '@tanstack/react-query';

/**
 * Custom hook for fetching and managing fields data for a specific entity
 *
 * This hook uses React Query to fetch and cache fields data,
 * and subscribes to Tauri events to keep the data in sync.
 *
 * @param entityId The ID of the entity
 */
export function useFields(entityId: number | null) {
    const queryClient = useQueryClient();

    // Query for fetching fields
    const fieldsQuery = useQuery({
        queryKey: ['fields', entityId],
        queryFn: async () => {
            try {
                if (entityId) {
                    // Get field IDs for a specific entity
                    const fieldIds = await entityService.getEntityRelationship(entityId, EntityRelationshipField.Fields);

                    // Get fields using the IDs
                    const fields = await fieldService.getFieldMulti(fieldIds);

                    // Filter out null fields
                    return fields.filter((field): field is FieldDTO => field !== null);
                } else {
                    // When entityId is null, we need to fetch all fields from all entities
                    // First, get all entities
                    const allEntitiesQuery = await queryClient.fetchQuery({
                        queryKey: ['entities'],
                        queryFn: async () => {
                            // This assumes there's a way to get all entities
                            // You might need to adjust this based on your actual API
                            const rootId = 1; // Assuming root ID is 1, adjust as needed
                            const entityIds = await rootService.getRootRelationship(rootId, RootRelationshipField.Entities);
                            const entities = await entityService.getEntityMulti(entityIds);
                            return entities.filter((entity): entity is EntityDTO => entity !== null);
                        }
                    });

                    // Now collect all field IDs from all entities
                    const allFieldIdsSet = new Set<number>();

                    // For each entity, get its fields
                    for (const entity of allEntitiesQuery) {
                        if (entity && entity.id) {
                            try {
                                const fieldIds = await entityService.getEntityRelationship(entity.id, EntityRelationshipField.Fields);
                                fieldIds.forEach(id => allFieldIdsSet.add(id));
                            } catch (err) {
                                error(`Error fetching fields for entity ${entity.id}: ${err}`);
                                // Continue with other entities even if one fails
                            }
                        }
                    }

                    // Convert Set to Array
                    const allFieldIds = Array.from(allFieldIdsSet);

                    // Get all fields using the collected IDs
                    if (allFieldIds.length > 0) {
                        const fields = await fieldService.getFieldMulti(allFieldIds);
                        return fields.filter((field): field is FieldDTO => field !== null);
                    }

                    return [];
                }
            } catch (err) {
                error(`Error fetching fields: ${err}`);
                throw err;
            }
        },
        // Always enabled, even when entityId is null
        enabled: true,
        staleTime: 1000 * 60 * 5, // 5 minutes
        retry: 1
    });

    // Mutation for creating a new field
    const createFieldMutation = useMutation({
        mutationFn: async (fieldData: {
            name: string;
            fieldType: FieldType;
            entity?: number | null;
            isNullable: boolean;
            isPrimaryKey: boolean;
            isList: boolean;
            single: boolean;
            strong: boolean;
            ordered: boolean;
            listModel: boolean;
            listModelDisplayedField?: string | null;
        } = {
            name: 'New Field',
            fieldType: FieldType.String,
            isNullable: false,
            isPrimaryKey: false,
            isList: false,
            single: true,
            strong: true,
            ordered: false,
            listModel: false,
            listModelDisplayedField: null
        }) => {
            if (!entityId) {
                throw new Error("No entity selected");
            }

            // Create field with provided values or defaults
            const dto: CreateFieldDTO = {
                name: fieldData.name || 'New Field',
                field_type: fieldData.fieldType || FieldType.String,
                entity: fieldData.entity !== undefined ? fieldData.entity : null,
                is_nullable: fieldData.isNullable || false,
                is_primary_key: fieldData.isPrimaryKey || false,
                is_list: fieldData.isList || false,
                single: fieldData.single !== undefined ? fieldData.single : true,
                strong: fieldData.strong !== undefined ? fieldData.strong : true,
                ordered: fieldData.ordered || false,
                list_model: fieldData.listModel || false,
                list_model_displayed_field: fieldData.listModelDisplayedField || null
            };

            try {
                // Create the field
                const newField = await fieldService.createField(dto);

                // Get existing fields for the entity
                const entityFields = await entityService.getEntityRelationship(entityId, EntityRelationshipField.Fields);

                // Add the new field to the entity relationship
                await entityService.setEntityRelationship({
                    id: entityId,
                    field: EntityRelationshipField.Fields,
                    right_ids: [...entityFields, newField.id],
                });

                return newField;
            } catch (err) {
                error(`Error creating field: ${err}`);
                throw err;
            }
        },
        onSuccess: () => {
            // Invalidate queries to refetch data
            queryClient.invalidateQueries({queryKey: ['fields']});
            info("Field created successfully");
        }
    });

    // Mutation for updating a field
    const updateFieldMutation = useMutation({
        mutationFn: async (field: FieldDTO) => {
            try {
                return await fieldService.updateField(field);
            } catch (err) {
                error(`Error updating field: ${err}`);
                throw err;
            }
        },
        onSuccess: () => {
            queryClient.invalidateQueries({queryKey: ['fields']});
            info("Field updated successfully");
        }
    });

    // Mutation for removing a field
    const removeFieldMutation = useMutation({
        mutationFn: async (fieldId: number) => {
            if (!entityId) {
                throw new Error("No entity selected");
            }

            try {
                // Get existing fields for the entity
                const entityFields = await entityService.getEntityRelationship(entityId, EntityRelationshipField.Fields);

                // Remove the field from the entity relationship
                await entityService.setEntityRelationship({
                    id: entityId,
                    field: EntityRelationshipField.Fields,
                    right_ids: entityFields.filter(id => id !== fieldId),
                });

                // Remove the field
                await fieldService.removeField(fieldId);
            } catch (err) {
                error(`Error removing field: ${err}`);
                throw err;
            }
        },
        onSuccess: () => {
            queryClient.invalidateQueries({queryKey: ['fields']});
            info("Field removed successfully");
        }
    });

    // Set up event listeners for Tauri events
    useEffect(() => {
        if (!entityId) return;

        // Handler for field created events
        const handleFieldCreated = (payload: EntityEventPayload) => {
            info(`Field created event received: ${payload.ids}`);
            queryClient.invalidateQueries({queryKey: ['fields', entityId]});
        };

        // Handler for field updated events
        const handleFieldUpdated = (payload: EntityEventPayload) => {
            info(`Field updated event received: ${payload.ids}`);
            queryClient.invalidateQueries({queryKey: ['fields', entityId]});
        };

        // Handler for field removed events
        const handleFieldRemoved = (payload: EntityEventPayload) => {
            info(`Field removed event received: ${payload.ids}`);
            queryClient.invalidateQueries({queryKey: ['fields', entityId]});
        };

        // Handler for entity updated events
        const handleEntityUpdated = (payload: EntityEventPayload) => {
            if (entityId && payload.ids.includes(entityId)) {
                info(`Entity updated event received for entity ${entityId}`);
                queryClient.invalidateQueries({queryKey: ['fields', entityId]});
            }
        };

        // Handler for reset events
        const handleReset = () => {
            info(`All reset event received`);
            queryClient.invalidateQueries({queryKey: ['fields']});
        };

        // Subscribe to field events
        const unsubscribe = directAccessEventService.subscribeToFieldEvents({
            onCreated: handleFieldCreated,
            onUpdated: handleFieldUpdated,
            onRemoved: handleFieldRemoved,
            onReset: handleReset
        });

        // Also subscribe to entity updates
        const unsubscribeEntity = directAccessEventService.subscribeToEntityEvents({
            onUpdated: handleEntityUpdated
        });

        // Cleanup function
        return () => {
            unsubscribe().catch(err => {
                error(`Error unsubscribing from field events: ${err}`);
            });
            unsubscribeEntity().catch(err => {
                error(`Error unsubscribing from entity events: ${err}`);
            });
        };
    }, [entityId, queryClient]);

    // Function to create a new field
    const createField = useCallback((fieldData?: {
        name: string;
        fieldType: FieldType;
        entity?: number | null;
        isNullable: boolean;
        isPrimaryKey: boolean;
        isList: boolean;
        single: boolean;
        strong: boolean;
        ordered: boolean;
        listModel: boolean;
        listModelDisplayedField?: string | null;
    }) => {
        createFieldMutation.mutate(fieldData);
    }, [createFieldMutation]);

    // Function to update a field
    const updateField = useCallback((field: FieldDTO) => {
        updateFieldMutation.mutate(field);
    }, [updateFieldMutation]);

    // Function to remove a field
    const removeField = useCallback((fieldId: number) => {
        removeFieldMutation.mutate(fieldId);
    }, [removeFieldMutation]);

    return {
        fields: fieldsQuery.data || [],
        isLoading: fieldsQuery.isLoading,
        error: fieldsQuery.error,
        createField,
        updateField,
        removeField,
        refetch: fieldsQuery.refetch
    };
}