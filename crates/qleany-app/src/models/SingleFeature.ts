import {useEffect, useState} from 'react';
import {listen} from "@tauri-apps/api/event";
import {error, info} from '@tauri-apps/plugin-log';
import {
    FeatureDto,
    getFeature,
    updateFeature
} from "@/controller/feature-controller.ts";
import {useDebouncedCallback} from "@mantine/hooks";

export interface SingleFeatureModelProps {
    featureId: number | null;
    onFeatureChanged?: (feature: FeatureDto | null) => void;
}

export function useSingleFeatureModel(
    {
        featureId,
        onFeatureChanged
    }
    : SingleFeatureModelProps) {
    const [feature, setFeature] = useState<FeatureDto | null>(null);
    const [loading, setLoading] = useState<boolean>(false);

    // Function to fetch feature data from the backend
    async function fetchFeatureData() {
        if (!featureId) {
            setFeature(null);
            if (onFeatureChanged) {
                onFeatureChanged(null);
            }
            return null;
        }

        setLoading(true);
        try {
            const featureData = await getFeature(featureId);
            
            setFeature(featureData);
            if (onFeatureChanged) {
                onFeatureChanged(featureData);
            }
            
            return featureData;
        } catch (err) {
            await error(`Failed to fetch feature data: ${err}`);
            throw err;
        } finally {
            setLoading(false);
        }
    }

    // Function to update feature data
    async function updateFeatureData(updatedFeature: FeatureDto) {
        try {
            setLoading(true);
            const result = await updateFeature(updatedFeature);
            
            setFeature(result);
            if (onFeatureChanged) {
                onFeatureChanged(result);
            }
            
            await info("Feature updated successfully");
            return result;
        } catch (err) {
            await error(`Failed to update feature: ${err}`);
            throw err;
        } finally {
            setLoading(false);
        }
    }

    const featureUpdaterHandler = useDebouncedCallback(async (event) => {
            const payload = event.payload as { ids: number[] };

            if (!featureId) {
                return;
            }

            if (!payload.ids.includes(featureId)) {
                return; // Ignore updates for other features
            }

            info(`Feature updated event received for feature ${featureId}`);
            await fetchFeatureData().catch((err) => error(`Error fetching updated feature: ${err}`));
        }
        , 1000);

    // Setup event listeners
    useEffect(() => {
        fetchFeatureData().catch((err) => error(`Error in initial feature fetch: ${err}`));

        // Listen for feature updates
        const unlisten_direct_access_feature_updated = listen('direct_access_feature_updated', featureUpdaterHandler);

        // Listen for feature removal events
        const unlisten_direct_access_feature_removed = listen('direct_access_feature_removed', async (event) => {
            const payload = event.payload as { ids: number[] };
            
            if (featureId && payload.ids.includes(featureId)) {
                info(`Feature removed event received for current feature ${featureId}`);
                setFeature(null);
                if (onFeatureChanged) {
                    onFeatureChanged(null);
                }
            }
        });

        const unlisten_direct_access_all_reset = listen('direct_access_all_reset', () => {
            info(`Direct access all reset event received`);
            fetchFeatureData().then((dto) => info(`Feature data reset successfully: ${JSON.stringify(dto)}`)
            ).catch((err) => error(`Error resetting feature data: ${err}`));
        });

        return () => {
            unlisten_direct_access_feature_updated.then(f => f());
            unlisten_direct_access_feature_removed.then(f => f());
            unlisten_direct_access_all_reset.then(f => f());
        };
    }, [featureId]);

    return {
        feature,
        loading,
        updateFeatureData,
        fetchFeatureData
    };
}