import { useEffect, useState } from 'react';
import { Alert, Button, Stack, TextInput, Title } from '@mantine/core';
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

    // Update form data when feature changes
    useEffect(() => {
        if (feature) {
            setFormData({
                name: feature.name,
            });
        }
    }, [feature]);

    const handleSubmit = async (e: React.FormEvent) => {
        e.preventDefault();

        if (feature) {
            try {
                // Update the feature with the form data
                const updatedFeature = {
                    ...feature,
                    name: formData.name,
                };

                // Call the context's update function
                updateFeature(updatedFeature);
            } catch (err) {
                // Error handling is done in the hook
            }
        }
    };

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
            <form onSubmit={handleSubmit}>
                <Stack>
                    <TextInput
                        id="featureName"
                        label="Name"
                        value={formData.name}
                        onChange={(e) => setFormData({...formData, name: e.target.value})}
                    />
                    <Button type="submit">Save Changes</Button>
                </Stack>
            </form>
        </ErrorBoundary>
    );
};

export default FeatureDetails;
