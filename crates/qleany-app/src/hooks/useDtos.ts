import {useCallback, useEffect} from 'react';
import {DtoDTO, dtoService} from '../services/dto-service';
import {EntityEventPayload, directAccessEventService} from '../services/direct-access-event-service.ts';
import {error, info} from '@tauri-apps/plugin-log';

import {useMutation, useQuery, useQueryClient} from '@tanstack/react-query';
import {undoRedoService} from "#services/undo-redo-service.ts";

/**
 * Custom hook for fetching and managing DTOs data
 *
 * This hook uses React Query to fetch and cache DTOs data,
 * and subscribes to Tauri events to keep the data in sync.
 */
export function useDtos() {
    const queryClient = useQueryClient();

    // Query for fetching all DTOs
    const dtosQuery = useQuery({
        queryKey: ['dtos'],
        queryFn: async () => {
            try {
                // For DTOs, we need to get all of them since they're not tied to a specific entity
                // This is a simplified approach - in a real app, you might need pagination or filtering
                const dtoIds = await dtoService.getDtoMulti([]);

                // Filter out null DTOs
                const filteredDtos = dtoIds.filter((dto): dto is DtoDTO => dto !== null);

                return filteredDtos;
            } catch (err) {
                error(`Error fetching DTOs: ${err}`);
                throw err;
            }
        },
        staleTime: 1000 * 60 * 5, // 5 minutes
        retry: 1
    });

    // Mutation for creating a new DTO
    const createDtoMutation = useMutation({
        mutationFn: async (name: string) => {
            // Create DTO with default values
            const dto = {
                name: name || 'New DTO',
                fields: [],
            };

            try {
                // Create the DTO
                return await dtoService.createDto(dto);
            } catch (err) {
                error(`Error creating DTO: ${err}`);
                throw err;
            }
        },
        onSuccess: () => {
            // Invalidate queries to refetch data
            queryClient.invalidateQueries({queryKey: ['dtos']});
            info("DTO created successfully");
        }
    });

    // Mutation for updating a DTO
    const updateDtoMutation = useMutation({
        mutationFn: async (dto: DtoDTO) => {
            try {
                return await dtoService.updateDto(dto);
            } catch (err) {
                error(`Error updating DTO: ${err}`);
                throw err;
            }
        },
        onSuccess: () => {
            queryClient.invalidateQueries({queryKey: ['dtos']});
            info("DTO updated successfully");
        }
    });

    // Mutation for removing a DTO
    const removeDtoMutation = useMutation({
        mutationFn: async (dtoId: number) => {
            try {
                await dtoService.removeDto(dtoId);
            } catch (err) {
                error(`Error removing DTO: ${err}`);
                throw err;
            }
        },
        onSuccess: () => {
            queryClient.invalidateQueries({queryKey: ['dtos']});
            info("DTO removed successfully");
        }
    });

    // Set up event listeners for Tauri events
    useEffect(() => {
        // Handler for DTO created events
        const handleDtoCreated = (payload: EntityEventPayload) => {
            info(`DTO created event received: ${payload.ids}`);
            queryClient.invalidateQueries({queryKey: ['dtos']});
        };

        // Handler for DTO updated events
        const handleDtoUpdated = (payload: EntityEventPayload) => {
            info(`DTO updated event received: ${payload.ids}`);
            queryClient.invalidateQueries({queryKey: ['dtos']});
        };

        // Handler for DTO removed events
        const handleDtoRemoved = (payload: EntityEventPayload) => {
            info(`DTO removed event received: ${payload.ids}`);
            queryClient.invalidateQueries({queryKey: ['dtos']});
        };

        // Handler for reset events
        const handleReset = () => {
            info(`All reset event received`);
            queryClient.invalidateQueries({queryKey: ['dtos']});
        };

        // Subscribe to events
        const unsubscribe = directAccessEventService.subscribeToDtoEvents({
            onCreated: handleDtoCreated,
            onUpdated: handleDtoUpdated,
            onRemoved: handleDtoRemoved,
            onReset: handleReset
        });

        const unsubscribeUndoRedo = undoRedoService.subscribeToUndoRedoEvents({
            onUndone: () => queryClient.invalidateQueries({queryKey: ['dtos']}),
            onRedone: () => queryClient.invalidateQueries({queryKey: ['dtos']}),
        });

        // Cleanup function
        return () => {
            unsubscribe().catch(err => {
                error(`Error unsubscribing from events: ${err}`);
            });
            unsubscribeUndoRedo().catch(err => {
                error(`Error unsubscribing from undo redo events: ${err}`);
            });
        };
    }, [queryClient]);

    // Function to create a new DTO
    const createDto = useCallback((name: string = 'New DTO') => {
        createDtoMutation.mutate(name);
    }, [createDtoMutation]);

    // Function to update a DTO
    const updateDto = useCallback((dto: DtoDTO) => {
        updateDtoMutation.mutate(dto);
    }, [updateDtoMutation]);

    // Function to remove a DTO
    const removeDto = useCallback((dtoId: number) => {
        removeDtoMutation.mutate(dtoId);
    }, [removeDtoMutation]);

    // Function to get a single DTO by ID
    const getDto = useCallback(async (dtoId: number) => {
        try {
            return await dtoService.getDto(dtoId);
        } catch (err) {
            error(`Error getting DTO: ${err}`);
            throw err;
        }
    }, []);

    return {
        dtos: dtosQuery.data || [],
        isLoading: dtosQuery.isLoading,
        error: dtosQuery.error,
        createDto,
        updateDto,
        removeDto,
        getDto,
        refetch: dtosQuery.refetch
    };
}