import {useCallback, useEffect} from 'react';
import {FeatureDTO, featureService} from '../services/feature-service';
import {RootRelationshipField, rootService} from '../services/root-service';
import {EntityEventPayload, directAccessEventService} from '../services/direct-access-event-service.ts';
import {error, info} from '@tauri-apps/plugin-log';

import {useMutation, useQuery, useQueryClient} from '@tanstack/react-query';

/**
 * Custom hook for fetching and managing features data
 *
 * This hook uses React Query to fetch and cache features data,
 * and subscribes to Tauri events to keep the data in sync.
 *
 * @param rootId The ID of the root entity
 */
export function useFeatures(rootId: number | null) {
    const queryClient = useQueryClient();

    // Query for fetching features
    const featuresQuery = useQuery({
        queryKey: ['features', rootId],
        queryFn: async () => {
            if (!rootId) return [];

            try {
                // Get feature IDs from root relationship
                const featureIds = await rootService.getRootRelationship(rootId, RootRelationshipField.Features);

                // Get features using the IDs
                const features = await featureService.getFeatureMulti(featureIds);

                // Filter out null features
                const filteredFeatures = features.filter((feature): feature is FeatureDTO => feature !== null);

                return filteredFeatures;
            } catch (err) {
                error(`Error fetching features: ${err}`);
                throw err;
            }
        },
        enabled: !!rootId,
        staleTime: 1000 * 60 * 5, // 5 minutes
        retry: 1
    });

    // Mutation for creating a new feature
    const createFeatureMutation = useMutation({
        mutationFn: async () => {
            if (!rootId) {
                throw new Error("No root selected");
            }

            // Create feature with default values
            const dto = {
                name: 'New Feature',
                use_cases: [],
            };

            try {
                // Create the feature
                const newFeature = await featureService.createFeature(dto);

                // Get existing features for the root
                const rootFeatures = await rootService.getRootRelationship(rootId, RootRelationshipField.Features);

                // Add the new feature to the root relationship
                await rootService.setRootRelationship({
                    id: rootId,
                    field: RootRelationshipField.Features,
                    right_ids: [...rootFeatures, newFeature.id],
                });

                return newFeature;
            } catch (err) {
                error(`Error creating feature: ${err}`);
                throw err;
            }
        },
        onSuccess: () => {
            // Invalidate queries to refetch data
            queryClient.invalidateQueries({queryKey: ['features']});
            info("Feature created successfully");
        }
    });

    // Mutation for updating a feature
    const updateFeatureMutation = useMutation({
        mutationFn: async (feature: FeatureDTO) => {
            try {
                return await featureService.updateFeature(feature);
            } catch (err) {
                error(`Error updating feature: ${err}`);
                throw err;
            }
        },
        onSuccess: () => {
            queryClient.invalidateQueries({queryKey: ['features']});
            info("Feature updated successfully");
        }
    });

    // Mutation for reordering features
    const reorderFeaturesMutation = useMutation({
        mutationFn: async (reorderedIds: number[]) => {
            if (!rootId) {
                throw new Error("No root selected");
            }

            try {
                await rootService.setRootRelationship({
                    id: rootId,
                    field: RootRelationshipField.Features,
                    right_ids: reorderedIds,
                });
            } catch (err) {
                error(`Error reordering features: ${err}`);
                throw err;
            }
        },
        onSuccess: () => {
            queryClient.invalidateQueries({queryKey: ['features']});
            info("Features reordered successfully");
        }
    });

    // Set up event listeners for Tauri events
    useEffect(() => {
        if (!rootId) return;

        // Handler for feature created events
        const handleFeatureCreated = (payload: EntityEventPayload) => {
            info(`Feature created event received: ${payload.ids}`);
            queryClient.invalidateQueries({queryKey: ['features', rootId]});
        };

        // Handler for feature updated events
        const handleFeatureUpdated = (payload: EntityEventPayload) => {
            info(`Feature updated event received: ${payload.ids}`);
            queryClient.invalidateQueries({queryKey: ['features', rootId]});
        };

        // Handler for feature removed events
        const handleFeatureRemoved = (payload: EntityEventPayload) => {
            info(`Feature removed event received: ${payload.ids}`);
            queryClient.invalidateQueries({queryKey: ['features', rootId]});
        };

        // Handler for root updated events
        const handleRootUpdated = (payload: EntityEventPayload) => {
            if (rootId && payload.ids.includes(rootId)) {
                info(`Root updated event received for root ${rootId}`);
                queryClient.invalidateQueries({queryKey: ['features', rootId]});
            }
        };

        // Handler for reset events
        const handleReset = () => {
            info(`All reset event received`);
            queryClient.invalidateQueries({queryKey: ['features']});
        };

        // Subscribe to feature events
        const unsubscribe = directAccessEventService.subscribeToFeatureEvents({
            onCreated: handleFeatureCreated,
            onUpdated: handleFeatureUpdated,
            onRemoved: handleFeatureRemoved,
            onReset: handleReset
        });

        // Also subscribe to root updates
        const unsubscribeRoot = directAccessEventService.subscribeToRootEvents({
            onUpdated: handleRootUpdated
        });

        // Cleanup function
        return () => {
            unsubscribe().catch(err => {
                error(`Error unsubscribing from feature events: ${err}`);
            });
            unsubscribeRoot().catch(err => {
                error(`Error unsubscribing from root events: ${err}`);
            });
        };
    }, [rootId, queryClient]);

    // Function to create a new feature
    const createFeature = useCallback(() => {
        createFeatureMutation.mutate({});
    }, [createFeatureMutation]);

    // Function to update a feature
    const updateFeature = useCallback((feature: FeatureDTO) => {
        updateFeatureMutation.mutate(feature);
    }, [updateFeatureMutation]);

    // Function to reorder features
    const reorderFeatures = useCallback((reorderedIds: number[]) => {
        reorderFeaturesMutation.mutate(reorderedIds);
    }, [reorderFeaturesMutation]);

    return {
        features: featuresQuery.data || [],
        isLoading: featuresQuery.isLoading,
        error: featuresQuery.error,
        createFeature,
        updateFeature,
        reorderFeatures,
        refetch: featuresQuery.refetch
    };
}