import {useEffect, useState} from 'react';
import {FeatureDto, updateFeature} from "../controller/feature_controller";
import {Button, Stack, TextInput, Title} from '@mantine/core';
import {error, info} from '@tauri-apps/plugin-log';

interface FeatureDetailsProps {
    selectedFeature: number | null;
    features: FeatureDto[];
    onFeatureUpdated: () => void;
}

const FeatureDetails = ({selectedFeature, features, onFeatureUpdated}: FeatureDetailsProps) => {
    const [formData, setFormData] = useState<{
        name: string;
    }>({
        name: '',
    });

    // Update form data when selected feature changes
    useEffect(() => {
        if (selectedFeature) {
            const featureData = features.find(feature => feature.id === selectedFeature);
            if (featureData) {
                setFormData({
                    name: featureData.name,
                });
            }
        }
    }, [selectedFeature, features]);

    const handleSubmit = async (e: React.FormEvent) => {
        e.preventDefault();

        // Find the selected feature in the data
        const selectedFeatureData = features.find(feature => feature.id === selectedFeature);

        if (selectedFeatureData) {
            try {
                // Update the feature with the form data
                const updatedFeature = {
                    ...selectedFeatureData,
                    name: formData.name,
                };

                // Call the API to update the feature
                await updateFeature(updatedFeature);

                // Notify parent component to refresh data
                onFeatureUpdated();

                await info("Feature updated successfully");
            } catch (err) {
                await error(`Failed to update feature: ${err}`);
            }
        }
    };

    const renderContent = () => {
        if (!selectedFeature) {
            return null;
        }

        return (
            <>
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
            </>
        );
    };

    return (
        <>
            {renderContent()}
        </>
    );
};

export default FeatureDetails;