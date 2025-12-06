import { useEffect, useRef, useState } from 'react';
import { Alert, Divider, Stack, TextInput, Title } from '@mantine/core';
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

    // Refs for debouncing
    const saveTimeoutRef = useRef<number | null>(null);
    const isLoadingDataRef = useRef(false);

    // Update form data when DTO changes
    useEffect(() => {
        if (dto) {
            // Flag that we're loading data from external source
            isLoadingDataRef.current = true;

            setFormData({
                name: dto.name,
            });

            // Reset flag after a brief timeout to allow state update to complete
            setTimeout(() => {
                isLoadingDataRef.current = false;
            }, 0);
        }
    }, [dto]);

    // Add auto-save effect with debouncing
    useEffect(() => {
        if (isLoadingDtos) return;
        if (!dto) return; // Skip if dto data hasn't loaded yet
        if (isLoadingDataRef.current) return; // Skip if formData change is from external data loading

        // Check if formData actually differs from the current dto
        const hasChanges = (
            formData.name !== dto.name
        );

        if (!hasChanges) return; // Skip if no actual changes

        if (saveTimeoutRef.current) {
            window.clearTimeout(saveTimeoutRef.current);
        }

        saveTimeoutRef.current = window.setTimeout(async () => {
            try {
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
            }
        }, 500);

        return () => {
            if (saveTimeoutRef.current) {
                window.clearTimeout(saveTimeoutRef.current);
            }
        };
    }, [formData, dto, isLoadingDtos, updateDto]);

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
            <Stack>
                <TextInput
                    id="dtoName"
                    label="Name"
                    value={formData.name}
                    onChange={(e) => setFormData({...formData, name: e.target.value})}
                    disabled={isLoadingDtos}
                />
            </Stack>

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
