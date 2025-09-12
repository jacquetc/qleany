import { useEffect, useState, useRef } from 'react';
import { Alert, Stack, TextInput, Title } from '@mantine/core';
import { error, info } from '@tauri-apps/plugin-log';
import { useFeatureContext } from '@/contexts/FeatureContext';
import ErrorBoundary from '@/components/ErrorBoundary';

const FeatureDetails = () => {
    const {
        features,
        selectedFeatureId,
        isLoadingFeatures,
        featureError,
        updateFeature
    } = useFeatureContext();

    // Find the selected feature from the features array
    const feature = selectedFeatureId 
        ? features.find(f => f.id === selectedFeatureId) || null 
        : null;

    const [formData, setFormData] = useState<{
        name: string;
    }>({
        name: '',
    });

    // Refs for debouncing
    const saveTimeoutRef = useRef<number | null>(null);
    const isLoadingDataRef = useRef(false);

    // Update form data when feature changes
    useEffect(() => {
        if (feature) {
            // Flag that we're loading data from external source
            isLoadingDataRef.current = true;
            
            setFormData({
                name: feature.name,
            });
            
            // Reset flag after a brief timeout to allow state update to complete
            setTimeout(() => {
                isLoadingDataRef.current = false;
            }, 0);
        }
    }, [feature]);

    // Add auto-save effect with debouncing
    useEffect(() => {
        if (isLoadingFeatures) return;
        if (!feature) return; // Skip if feature data hasn't loaded yet
        if (isLoadingDataRef.current) return; // Skip if formData change is from external data loading

        // Check if formData actually differs from the current feature
        const hasChanges = (
            formData.name !== feature.name
        );

        if (!hasChanges) return; // Skip if no actual changes

        if (saveTimeoutRef.current) {
            window.clearTimeout(saveTimeoutRef.current);
        }

        saveTimeoutRef.current = window.setTimeout(async () => {
            try {
                // Update the feature with the form data
                const updatedFeature = {
                    ...feature,
                    name: formData.name,
                };

                // Call the context's update function
                updateFeature(updatedFeature);
                info("Feature updated successfully");
            } catch (err) {
                error(`Failed to update feature: ${err}`);
            }
        }, 500);

        return () => {
            if (saveTimeoutRef.current) {
                window.clearTimeout(saveTimeoutRef.current);
            }
        };
    }, [formData, feature, isLoadingFeatures, updateFeature]);


    // Custom fallback component for error state
    const errorFallback = (
        <Alert color="yellow" title="Feature details could not be loaded">
            There was an issue loading the feature details. Please try again later.
        </Alert>
    );

    // Loading state
    if (isLoadingFeatures) {
        return (
            <Alert color="blue" title="Loading feature details">
                Please wait...
            </Alert>
        );
    }

    // Error state
    if (featureError) {
        return (
            <Alert color="red" title="Error loading feature details">
                {featureError instanceof Error ? featureError.message : 'An unknown error occurred'}
            </Alert>
        );
    }

    // No feature selected state
    if (!selectedFeatureId || !feature) {
        return (
            <Alert color="gray" title="No feature selected">
                Please select a feature to view its details.
            </Alert>
        );
    }

    return (
        <ErrorBoundary fallback={errorFallback}>
            <Title order={2}>"{formData.name}" details</Title>
            <Stack>
                <TextInput
                    id="featureName"
                    label="Name"
                    value={formData.name}
                    onChange={(e) => {
                        const newName = e.target.value;
                        setFormData({...formData, name: newName});
                    }}
                    disabled={isLoadingFeatures}
                />
            </Stack>
        </ErrorBoundary>
    );
};

export default FeatureDetails;
