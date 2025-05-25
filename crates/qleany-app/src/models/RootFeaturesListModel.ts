import {useEffect, useState} from 'react';
import {listen} from "@tauri-apps/api/event";
import {error, info} from '@tauri-apps/plugin-log';
import {createFeature, FeatureDto, getFeatureMulti} from "#controller/feature-controller.ts";
import {
    getRootMulti,
    getRootRelationship,
    RootRelationshipField,
    setRootRelationship
} from "@/controller/root-controller.ts";

export interface FeatureListModelCallbacks {
    onFeaturesChanged: (features: FeatureDto[]) => void;
}

export function useFeatureListModel(callbacks: FeatureListModelCallbacks) {
    const [features, setFeatures] = useState<FeatureDto[]>([]);
    const [rootId, setRootId] = useState<number>(1);

    // Function to get the root ID
    async function getRootIdFromBackend() {
        const roots = await getRootMulti([]);
        if (roots.length > 0 && roots[0] !== null) {
            setRootId(roots[0]!.id);
            return roots[0]!.id;
        }
        return 1; // Fallback to default
    }

    // Function to fetch feature data from the backend
    async function fetchFeatureData() {
        const currentRootId = await getRootIdFromBackend();
        const featureIds = await getRootRelationship(currentRootId, RootRelationshipField.Features);
        const features = await getFeatureMulti(featureIds);
        const filteredFeatures = features.filter((feature) => feature !== null) as FeatureDto[];

        setFeatures(filteredFeatures);
        callbacks.onFeaturesChanged(filteredFeatures);

        return filteredFeatures;
    }

    async function createNewFeature() {
        try {
            // Create feature
            const dto = {
                name: 'New Feature',
                use_cases: [],
            };

            const newFeature = await createFeature(dto);

            // Update root relationship
            const currentRootId = await getRootIdFromBackend();
            const rootFeatures = await getRootRelationship(currentRootId, RootRelationshipField.Features) || [];

            await setRootRelationship({
                id: currentRootId,
                field: RootRelationshipField.Features,
                right_ids: [...rootFeatures, newFeature.id],
            });

            // Update features state
            const updatedFeatures = [...features, newFeature];
            setFeatures(updatedFeatures);
            callbacks.onFeaturesChanged(updatedFeatures);

            await info("Feature created successfully");
            return newFeature;
        } catch (err) {
            await error(`Failed to create feature: ${err}`);
            throw err;
        }
    }

    // Function to handle reordering of features
    async function handleReorder(reorderedIds: number[]): Promise<void> {
        try {
            // Update the root relationship with the new order
            await setRootRelationship({
                id: rootId,
                field: RootRelationshipField.Features,
                right_ids: reorderedIds,
            });

            info("Feature order updated successfully");
            await fetchFeatureData();
        } catch (err) {
            error(`Failed to update feature order: ${err}`);
            throw err;
        }
    }

    // Setup event listeners
    useEffect(() => {
        fetchFeatureData().catch((err) => error(err));

        // mounting the event listeners
        const unlisten_direct_access_feature_created = listen('direct_access_feature_created', (event) => {
            const payload = event.payload as { ids: string[] };
            info(`Feature created event received: ${payload.ids}`);

            fetchFeatureData().catch((err) => error(err));
        });

        const unlisten_direct_access_feature_updated = listen('direct_access_feature_updated', async (event) => {
                const payload = event.payload as { ids: number[] };
                info(`Feature updated event received: ${payload.ids}`);
                const updatedFeatures = await getFeatureMulti(payload.ids);

                for (const updatedFeature of updatedFeatures) {

                    if (!updatedFeature) {
                        info(`Feature not found in the current state.`);
                        continue;
                    }
                    const index = features.findIndex((feature) => feature.id === updatedFeature.id);
                    if (index !== -1) {
                        const updatedFeaturesList = [...features];
                        updatedFeaturesList[index] = updatedFeature;
                        setFeatures(updatedFeaturesList);
                        callbacks.onFeaturesChanged(updatedFeaturesList);
                    } else {
                        info(`Feature not found in the current state.`);
                    }
                }
            }
        );

        const unlisten_direct_access_all_reset = listen('direct_access_all_reset', () => {
            info(`Direct access all reset event received`);
            fetchFeatureData().catch((err) => error(err));
        });

        return () => {
            unlisten_direct_access_feature_created.then(f => f());
            unlisten_direct_access_feature_updated.then(f => f());
            unlisten_direct_access_all_reset.then(f => f());
        };
    }, [features]);

    return {
        features,
        createNewFeature,
        handleReorder,
        fetchFeatureData
    };
}