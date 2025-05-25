import {useEffect, useState} from 'react';
import {Button, Stack, TextInput, Title} from '@mantine/core';
import {useSingleFeatureModel} from "#models/SingleFeature.ts";

interface FeatureDetailsProps {
    selectedFeature: number | null;
}

const FeatureDetails = ({selectedFeature}: FeatureDetailsProps) => {
    const {feature, updateFeatureData} = useSingleFeatureModel({
        featureId: selectedFeature
    });

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

                // Call the model's update function
                await updateFeatureData(updatedFeature);
            } catch (err) {
                // Error handling is done in the model
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
