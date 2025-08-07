import { useEffect, useState } from 'react';
import { Alert, Button, Checkbox, Stack, Tabs, TextInput, Title } from '@mantine/core';
import { error, info } from '@tauri-apps/plugin-log';
import { useFeatureContext } from '@/contexts/FeatureContext';
import { UseCaseDTO, UseCaseRelationshipField } from '@/services/use-case-service';
import DtoSelector from './DtoSelector.tsx';
import DtoDetails from './DtoDetails.tsx';
import ErrorBoundary from '@/components/ErrorBoundary';

const UseCaseDetails = () => {
    const {
        selectedUseCaseId,
        useCases,
        isLoadingUseCases,
        useCaseError,
        updateUseCase
    } = useFeatureContext();

    // Find the selected use case from the useCases array
    const useCase = selectedUseCaseId 
        ? useCases.find(u => u.id === selectedUseCaseId) || null 
        : null;

    const [formData, setFormData] = useState<{
        name: string;
        validator: boolean;
        undoable: boolean;
        dto_in: number | null;
        dto_out: number | null;
    }>({
        name: '',
        validator: false,
        undoable: false,
        dto_in: null,
        dto_out: null,
    });

    const [entities, setEntities] = useState<any[]>([]); // TODO: Replace with proper entity type
    const [selectedEntities, setSelectedEntities] = useState<number[]>([]);
    const [loading, setLoading] = useState(false);
    const [activeTab, setActiveTab] = useState<string | null>("details");

    // Update form data when use case changes
    useEffect(() => {
        if (useCase) {
            setFormData({
                name: useCase.name,
                validator: useCase.validator,
                undoable: useCase.undoable,
                dto_in: useCase.dto_in,
                dto_out: useCase.dto_out,
            });
            setSelectedEntities(useCase.entities);
        }
    }, [useCase]);

    // TODO: Fetch entities for the entity dropdown
    // This would need to be moved to a custom hook or context

    const handleSubmit = async (e: React.FormEvent) => {
        e.preventDefault();

        if (!useCase) return;

        try {
            setLoading(true);
            
            // Update the use case with the form data
            const updatedUseCase: UseCaseDTO = {
                ...useCase,
                name: formData.name,
                validator: formData.validator,
                undoable: formData.undoable,
                dto_in: formData.dto_in,
                dto_out: formData.dto_out,
                entities: selectedEntities, // Include the selected entities
            };

            // Call the context's update function
            updateUseCase(updatedUseCase);
            
            info("Use Case updated successfully");
        } catch (err) {
            error(`Failed to update use case: ${err}`);
        } finally {
            setLoading(false);
        }
    };

    const handleEntityChange = (entityId: number, checked: boolean) => {
        if (checked) {
            setSelectedEntities(prev => [...prev, entityId]);
        } else {
            setSelectedEntities(prev => prev.filter(id => id !== entityId));
        }
    };

    // Custom fallback component for error state
    const errorFallback = (
        <Alert color="yellow" title="Use case details could not be loaded">
            There was an issue loading the use case details. Please try again later.
        </Alert>
    );

    // Loading state
    if (isLoadingUseCases) {
        return (
            <Alert color="blue" title="Loading use case details">
                Please wait...
            </Alert>
        );
    }

    // Error state
    if (useCaseError) {
        return (
            <Alert color="red" title="Error loading use case details">
                {useCaseError instanceof Error ? useCaseError.message : 'An unknown error occurred'}
            </Alert>
        );
    }

    // No use case selected state
    if (!selectedUseCaseId || !useCase) {
        return (
            <Alert color="gray" title="No use case selected">
                Please select a use case to view its details.
            </Alert>
        );
    }

    return (
        <ErrorBoundary fallback={errorFallback}>
            <Tabs value={activeTab} onChange={setActiveTab}>
                <Tabs.List>
                    <Tabs.Tab value="details">Use Case Details</Tabs.Tab>
                    {formData.dto_in !== null && <Tabs.Tab value="dto_in">DTO In</Tabs.Tab>}
                    {formData.dto_out !== null && <Tabs.Tab value="dto_out">DTO Out</Tabs.Tab>}
                </Tabs.List>

                <Tabs.Panel value="details">
                    <Title order={2}>"{formData.name}" details</Title>
                    <form onSubmit={handleSubmit}>
                        <Stack>
                            <TextInput
                                id="useCaseName"
                                label="Name"
                                value={formData.name}
                                onChange={(e) => setFormData({...formData, name: e.target.value})}
                            />

                            <Checkbox
                                id="useCaseValidator"
                                label="Validator"
                                checked={formData.validator}
                                onChange={(e) => setFormData({...formData, validator: e.target.checked})}
                            />

                            <Checkbox
                                id="useCaseUndoable"
                                label="Undoable"
                                checked={formData.undoable}
                                onChange={(e) => setFormData({...formData, undoable: e.target.checked})}
                            />

                            <Title order={4}>Entities</Title>
                            <Stack>
                                {entities.map(entity => (
                                    <Checkbox
                                        key={entity.id}
                                        id={`entity-${entity.id}`}
                                        label={entity.name}
                                        checked={selectedEntities.includes(entity.id)}
                                        onChange={(e) => handleEntityChange(entity.id, e.target.checked)}
                                    />
                                ))}
                            </Stack>

                            <DtoSelector
                                label="DTO In"
                                value={formData.dto_in}
                                useCaseId={selectedUseCaseId}
                                isDtoOut={false}
                                onChange={(dtoId) => setFormData({...formData, dto_in: dtoId})}
                            />

                            <DtoSelector
                                label="DTO Out"
                                value={formData.dto_out}
                                useCaseId={selectedUseCaseId}
                                isDtoOut={true}
                                onChange={(dtoId) => setFormData({...formData, dto_out: dtoId})}
                            />

                            <Button type="submit" loading={loading}>Save Changes</Button>
                        </Stack>
                    </form>
                </Tabs.Panel>

                {formData.dto_in !== null && (
                    <Tabs.Panel value="dto_in">
                        <DtoDetails selectedDto={formData.dto_in}/>
                    </Tabs.Panel>
                )}

                {formData.dto_out !== null && (
                    <Tabs.Panel value="dto_out">
                        <DtoDetails selectedDto={formData.dto_out}/>
                    </Tabs.Panel>
                )}
            </Tabs>
        </ErrorBoundary>
    );
};

export default UseCaseDetails;
