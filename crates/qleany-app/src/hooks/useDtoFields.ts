import {useCallback, useEffect} from 'react';
import {DtoFieldDTO, dtoFieldService} from '../services/dto-field-service';
import {DtoRelationshipField, dtoService} from '../services/dto-service';
import {EntityEventPayload, directAccessEventService} from '../services/direct-access-event-service.ts';
import {error, info} from '@tauri-apps/plugin-log';

import {useMutation, useQuery, useQueryClient} from '@tanstack/react-query';

/**
 * Custom hook for fetching and managing DTO fields data for a DTO
 *
 * This hook uses React Query to fetch and cache DTO fields data,
 * and subscribes to Tauri events to keep the data in sync.
 *
 * @param dtoId The ID of the DTO
 */
export function useDtoFields(dtoId: number | null) {
    const queryClient = useQueryClient();

    // Query for fetching DTO fields
    const dtoFieldsQuery = useQuery({
        queryKey: ['dtoFields', dtoId],
        queryFn: async () => {
            if (!dtoId) return [];

            try {
                // Get DTO field IDs from DTO relationship
                const dtoFieldIds = await dtoService.getDtoRelationship(dtoId, DtoRelationshipField.Fields);

                // Get DTO fields using the IDs
                const dtoFields = await dtoFieldService.getDtoFieldMulti(dtoFieldIds);

                // Filter out null DTO fields
                const filteredDtoFields = dtoFields.filter((dtoField): dtoField is DtoFieldDTO => dtoField !== null);

                return filteredDtoFields;
            } catch (err) {
                error(`Error fetching DTO fields: ${err}`);
                throw err;
            }
        },
        enabled: !!dtoId,
        staleTime: 1000 * 60 * 5, // 5 minutes
        retry: 1
    });

    // Mutation for creating a new DTO field
    const createDtoFieldMutation = useMutation({
        mutationFn: async (fieldData: {
            name: string;
            fieldType: string;
            isNullable: boolean;
            isList: boolean;
        }) => {
            if (!dtoId) {
                throw new Error("No DTO selected");
            }

            // Create DTO field with provided values or defaults
            const dto = {
                name: fieldData.name || 'New Field',
                field_type: fieldData.fieldType || 'String',
                is_nullable: fieldData.isNullable || false,
                is_list: fieldData.isList || false
            };

            try {
                // Create the DTO field
                const newDtoField = await dtoFieldService.createDtoField(dto);

                // Get existing DTO fields for the DTO
                const dtoFields = await dtoService.getDtoRelationship(dtoId, DtoRelationshipField.Fields);

                // Add the new DTO field to the DTO relationship
                await dtoService.setDtoRelationship({
                    id: dtoId,
                    field: DtoRelationshipField.Fields,
                    right_ids: [...dtoFields, newDtoField.id],
                });

                return newDtoField;
            } catch (err) {
                error(`Error creating DTO field: ${err}`);
                throw err;
            }
        },
        onSuccess: () => {
            // Invalidate queries to refetch data
            queryClient.invalidateQueries({queryKey: ['dtoFields']});
            info("DTO field created successfully");
        }
    });

    // Mutation for updating a DTO field
    const updateDtoFieldMutation = useMutation({
        mutationFn: async (dtoField: DtoFieldDTO) => {
            try {
                return await dtoFieldService.updateDtoField(dtoField);
            } catch (err) {
                error(`Error updating DTO field: ${err}`);
                throw err;
            }
        },
        onSuccess: () => {
            queryClient.invalidateQueries({queryKey: ['dtoFields']});
            info("DTO field updated successfully");
        }
    });

    // Mutation for removing a DTO field
    const removeDtoFieldMutation = useMutation({
        mutationFn: async (dtoFieldId: number) => {
            if (!dtoId) {
                throw new Error("No DTO selected");
            }

            try {
                // Get existing DTO fields for the DTO
                const dtoFields = await dtoService.getDtoRelationship(dtoId, DtoRelationshipField.Fields);

                // Remove the DTO field from the DTO relationship
                await dtoService.setDtoRelationship({
                    id: dtoId,
                    field: DtoRelationshipField.Fields,
                    right_ids: dtoFields.filter(id => id !== dtoFieldId),
                });

                // Remove the DTO field
                await dtoFieldService.removeDtoField(dtoFieldId);
            } catch (err) {
                error(`Error removing DTO field: ${err}`);
                throw err;
            }
        },
        onSuccess: () => {
            queryClient.invalidateQueries({queryKey: ['dtoFields']});
            info("DTO field removed successfully");
        }
    });

    // Set up event listeners for Tauri events
    useEffect(() => {
        if (!dtoId) return;

        // Handler for DTO field created events
        const handleDtoFieldCreated = (payload: EntityEventPayload) => {
            info(`DTO field created event received: ${payload.ids}`);
            queryClient.invalidateQueries({queryKey: ['dtoFields', dtoId]});
        };

        // Handler for DTO field updated events
        const handleDtoFieldUpdated = (payload: EntityEventPayload) => {
            info(`DTO field updated event received: ${payload.ids}`);
            queryClient.invalidateQueries({queryKey: ['dtoFields', dtoId]});
        };

        // Handler for DTO field removed events
        const handleDtoFieldRemoved = (payload: EntityEventPayload) => {
            info(`DTO field removed event received: ${payload.ids}`);
            queryClient.invalidateQueries({queryKey: ['dtoFields', dtoId]});
        };

        // Handler for DTO updated events
        const handleDtoUpdated = (payload: EntityEventPayload) => {
            if (dtoId && payload.ids.includes(dtoId)) {
                info(`DTO updated event received for DTO ${dtoId}`);
                queryClient.invalidateQueries({queryKey: ['dtoFields', dtoId]});
            }
        };

        // Handler for reset events
        const handleReset = () => {
            info(`All reset event received`);
            queryClient.invalidateQueries({queryKey: ['dtoFields']});
        };

        // Subscribe to events
        const unsubscribe = directAccessEventService.subscribeToDtoFieldEvents({
            onCreated: handleDtoFieldCreated,
            onUpdated: handleDtoFieldUpdated,
            onRemoved: handleDtoFieldRemoved,
            onReset: handleReset
        });

        // Also subscribe to DTO updates
        const unsubscribeDto = directAccessEventService.subscribeToDtoEvents({
            onUpdated: handleDtoUpdated
        });

        // Cleanup function
        return () => {
            unsubscribe().catch(err => {
                error(`Error unsubscribing from DTO field events: ${err}`);
            });
            unsubscribeDto().catch(err => {
                error(`Error unsubscribing from DTO events: ${err}`);
            });
        };
    }, [dtoId, queryClient]);

    // Function to create a new DTO field
    const createDtoField = useCallback((fieldData: {
        name: string;
        fieldType: string;
        isNullable: boolean;
        isList: boolean;
    } = {
        name: 'New Field',
        fieldType: 'String',
        isNullable: false,
        isList: false
    }) => {
        createDtoFieldMutation.mutate(fieldData);
    }, [createDtoFieldMutation]);

    // Function to update a DTO field
    const updateDtoField = useCallback((dtoField: DtoFieldDTO) => {
        updateDtoFieldMutation.mutate(dtoField);
    }, [updateDtoFieldMutation]);

    // Function to remove a DTO field
    const removeDtoField = useCallback((dtoFieldId: number) => {
        removeDtoFieldMutation.mutate(dtoFieldId);
    }, [removeDtoFieldMutation]);

    return {
        dtoFields: dtoFieldsQuery.data || [],
        isLoading: dtoFieldsQuery.isLoading,
        error: dtoFieldsQuery.error,
        createDtoField,
        updateDtoField,
        removeDtoField,
        refetch: dtoFieldsQuery.refetch
    };
}