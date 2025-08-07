import { useEffect, useState } from 'react';
import { Alert, Button, Divider, Stack, TextInput, Title } from '@mantine/core';
import { error, info } from '@tauri-apps/plugin-log';
import { useFeatureContext } from '@/contexts/FeatureContext';
import { DtoDTO } from '@/services/dto-service';
import DtoFieldsList from './DtoFieldsList.tsx';
import DtoFieldDetails from './DtoFieldDetails.tsx';
import ErrorBoundary from '@/components/ErrorBoundary';

interface DtoDetailsProps {
    selectedDto: number | null; // Keep this prop for now until we refactor all components
}

const DtoDetails = ({ selectedDto }: DtoDetailsProps) => {
    const {
        dtos,
        isLoadingDtos,
        dtoError,
        updateDto
    } = useFeatureContext();

    // Find the selected DTO from the dtos array
    const dto = selectedDto 
        ? dtos.find(d => d.id === selectedDto) || null 
        : null;

    const [formData, setFormData] = useState<{
        name: string;
    }>({
        name: '',
    });

    const [selectedDtoField, setSelectedDtoField] = useState<number | null>(null);
    const [loading, setLoading] = useState(false);

    // Update form data when DTO changes
    useEffect(() => {
        if (dto) {
            setFormData({
                name: dto.name,
            });
        }
    }, [dto]);

    const handleSubmit = async (e: React.FormEvent) => {
        e.preventDefault();

        if (!dto) return;

        try {
            setLoading(true);
            
            // Update the DTO with the form data
            const updatedDto: DtoDTO = {
                ...dto,
                name: formData.name,
            };

            // Call the context's update function
            updateDto(updatedDto);
            
            info("DTO updated successfully");
        } catch (err) {
            error(`Failed to update DTO: ${err}`);
        } finally {
            setLoading(false);
        }
    };

    // Custom fallback component for error state
    const errorFallback = (
        <Alert color="yellow" title="DTO details could not be loaded">
            There was an issue loading the DTO details. Please try again later.
        </Alert>
    );

    // Loading state
    if (isLoadingDtos) {
        return (
            <Alert color="blue" title="Loading DTO details">
                Please wait...
            </Alert>
        );
    }

    // Error state
    if (dtoError) {
        return (
            <Alert color="red" title="Error loading DTO details">
                {dtoError instanceof Error ? dtoError.message : 'An unknown error occurred'}
            </Alert>
        );
    }

    // No DTO selected state
    if (!selectedDto || !dto) {
        return (
            <Alert color="gray" title="No DTO selected">
                Please select a DTO to view its details.
            </Alert>
        );
    }

    return (
        <ErrorBoundary fallback={errorFallback}>
            <Title order={2}>"{formData.name}" details</Title>
            <form onSubmit={handleSubmit}>
                <Stack>
                    <TextInput
                        id="dtoName"
                        label="Name"
                        value={formData.name}
                        onChange={(e) => setFormData({...formData, name: e.target.value})}
                    />

                    <Button type="submit" loading={loading}>Save Changes</Button>
                </Stack>
            </form>

            <Divider my="md"/>

            <Stack>
                <DtoFieldsList
                    selectedDto={selectedDto}
                    onSelectDtoField={setSelectedDtoField}
                />
            </Stack>

            {selectedDtoField && (
                <>
                    <Divider my="md"/>
                    <Stack>
                        <DtoFieldDetails selectedDtoField={selectedDtoField} dtoId={selectedDto}/>
                    </Stack>
                </>
            )}
        </ErrorBoundary>
    );
};

export default DtoDetails;
