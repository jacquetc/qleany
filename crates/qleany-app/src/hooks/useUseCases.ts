import {useCallback, useEffect} from 'react';
import {UseCaseDTO, useCaseService} from '../services/use-case-service';
import {FeatureRelationshipField, featureService} from '../services/feature-service';
import {EntityEventPayload, directAccessEventService} from '../services/direct-access-event-service.ts';
import {error, info} from '@tauri-apps/plugin-log';

import {useMutation, useQuery, useQueryClient} from '@tanstack/react-query';

/**
 * Custom hook for fetching and managing use cases data for a feature
 *
 * This hook uses React Query to fetch and cache use cases data,
 * and subscribes to Tauri events to keep the data in sync.
 *
 * @param featureId The ID of the feature
 */
export function useUseCases(featureId: number | null) {
    const queryClient = useQueryClient();

    // Query for fetching use cases
    const useCasesQuery = useQuery({
        queryKey: ['useCases', featureId],
        queryFn: async () => {
            if (!featureId) return [];

            try {
                // Get use case IDs from feature relationship
                const useCaseIds = await featureService.getFeatureRelationship(featureId, FeatureRelationshipField.UseCases);

                // Get use cases using the IDs
                const useCases = await useCaseService.getUseCaseMulti(useCaseIds);

                // Filter out null use cases
                const filteredUseCases = useCases.filter((useCase): useCase is UseCaseDTO => useCase !== null);

                return filteredUseCases;
            } catch (err) {
                error(`Error fetching use cases: ${err}`);
                throw err;
            }
        },
        enabled: !!featureId,
        staleTime: 1000 * 60 * 5, // 5 minutes
        retry: 1
    });

    // Mutation for creating a new use case
    const createUseCaseMutation = useMutation({
        mutationFn: async () => {
            if (!featureId) {
                throw new Error("No feature selected");
            }

            // Create use case with default values
            const dto = {
                name: 'New Use Case',
                validator: false,
                entities: [],
                undoable: false,
                dto_in: null,
                dto_out: null
            };

            try {
                // Create the use case
                const newUseCase = await useCaseService.createUseCase(dto);

                // Get existing use cases for the feature
                const featureUseCases = await featureService.getFeatureRelationship(featureId, FeatureRelationshipField.UseCases);

                // Add the new use case to the feature relationship
                await featureService.setFeatureRelationship({
                    id: featureId,
                    field: FeatureRelationshipField.UseCases,
                    right_ids: [...featureUseCases, newUseCase.id],
                });

                return newUseCase;
            } catch (err) {
                error(`Error creating use case: ${err}`);
                throw err;
            }
        },
        onSuccess: () => {
            // Invalidate queries to refetch data
            queryClient.invalidateQueries({queryKey: ['useCases']});
            info("Use case created successfully");
        }
    });

    // Mutation for updating a use case
    const updateUseCaseMutation = useMutation({
        mutationFn: async (useCase: UseCaseDTO) => {
            try {
                return await useCaseService.updateUseCase(useCase);
            } catch (err) {
                error(`Error updating use case: ${err}`);
                throw err;
            }
        },
        onSuccess: () => {
            queryClient.invalidateQueries({queryKey: ['useCases']});
            info("Use case updated successfully");
        }
    });

    // Mutation for reordering use cases
    const reorderUseCasesMutation = useMutation({
        mutationFn: async (reorderedIds: number[]) => {
            if (!featureId) {
                throw new Error("No feature selected");
            }

            try {
                await featureService.setFeatureRelationship({
                    id: featureId,
                    field: FeatureRelationshipField.UseCases,
                    right_ids: reorderedIds,
                });
            } catch (err) {
                error(`Error reordering use cases: ${err}`);
                throw err;
            }
        },
        onSuccess: () => {
            queryClient.invalidateQueries({queryKey: ['useCases']});
            info("Use cases reordered successfully");
        }
    });

    // Set up event listeners for Tauri events
    useEffect(() => {
        if (!featureId) return;

        // Handler for use case created events
        const handleUseCaseCreated = (payload: EntityEventPayload) => {
            info(`Use case created event received: ${payload.ids}`);
            queryClient.invalidateQueries({queryKey: ['useCases', featureId]});
        };

        // Handler for use case updated events
        const handleUseCaseUpdated = (payload: EntityEventPayload) => {
            info(`Use case updated event received: ${payload.ids}`);
            queryClient.invalidateQueries({queryKey: ['useCases', featureId]});
        };

        // Handler for use case removed events
        const handleUseCaseRemoved = (payload: EntityEventPayload) => {
            info(`Use case removed event received: ${payload.ids}`);
            queryClient.invalidateQueries({queryKey: ['useCases', featureId]});
        };

        // Handler for feature updated events
        const handleFeatureUpdated = (payload: EntityEventPayload) => {
            if (featureId && payload.ids.includes(featureId)) {
                info(`Feature updated event received for feature ${featureId}`);
                queryClient.invalidateQueries({queryKey: ['useCases', featureId]});
            }
        };

        // Handler for reset events
        const handleReset = () => {
            info(`All reset event received`);
            queryClient.invalidateQueries({queryKey: ['useCases']});
        };

        // Subscribe to use case events
        const unsubscribe = directAccessEventService.subscribeToUseCaseEvents({
            onCreated: handleUseCaseCreated,
            onUpdated: handleUseCaseUpdated,
            onRemoved: handleUseCaseRemoved,
            onReset: handleReset
        });

        // Also subscribe to feature updates
        const unsubscribeFeature = directAccessEventService.subscribeToFeatureEvents({
            onUpdated: handleFeatureUpdated
        });

        // Cleanup function
        return () => {
            unsubscribe().catch(err => {
                error(`Error unsubscribing from use case events: ${err}`);
            });
            unsubscribeFeature().catch(err => {
                error(`Error unsubscribing from feature events: ${err}`);
            });
        };
    }, [featureId, queryClient]);

    // Function to create a new use case
    const createUseCase = useCallback(() => {
        createUseCaseMutation.mutate();
    }, [createUseCaseMutation]);

    // Function to update a use case
    const updateUseCase = useCallback((useCase: UseCaseDTO) => {
        updateUseCaseMutation.mutate(useCase);
    }, [updateUseCaseMutation]);

    // Function to reorder use cases
    const reorderUseCases = useCallback((reorderedIds: number[]) => {
        reorderUseCasesMutation.mutate(reorderedIds);
    }, [reorderUseCasesMutation]);

    return {
        useCases: useCasesQuery.data || [],
        isLoading: useCasesQuery.isLoading,
        error: useCasesQuery.error,
        createUseCase,
        updateUseCase,
        reorderUseCases,
        refetch: useCasesQuery.refetch
    };
}