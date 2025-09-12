import { useEffect, useState, useRef } from 'react';
import { Alert, Checkbox, Stack, Tabs, TextInput, Title } from '@mantine/core';
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

    // Refs for debouncing
    const saveTimeoutRef = useRef<number | null>(null);
    const isLoadingDataRef = useRef(false);

    // Update form data when use case changes
    useEffect(() => {
        if (useCase) {
            // Flag that we're loading data from external source
            isLoadingDataRef.current = true;
            
            setFormData({
                name: useCase.name,
                validator: useCase.validator,
                undoable: useCase.undoable,
                dto_in: useCase.dto_in,
                dto_out: useCase.dto_out,
            });
            setSelectedEntities(useCase.entities);
            
            // Reset flag after a brief timeout to allow state update to complete
            setTimeout(() => {
                isLoadingDataRef.current = false;
            }, 0);
        }
    }, [useCase]);

    // Add auto-save effect with debouncing
    useEffect(() => {
        if (isLoadingUseCases) return;
        if (!useCase) return; // Skip if use case data hasn't loaded yet
        if (isLoadingDataRef.current) return; // Skip if formData change is from external data loading

        // Check if formData actually differs from the current use case
        const hasChanges = (
            formData.name !== useCase.name ||
            formData.validator !== useCase.validator ||
            formData.undoable !== useCase.undoable ||
            formData.dto_in !== useCase.dto_in ||
            formData.dto_out !== useCase.dto_out ||
            JSON.stringify(selectedEntities) !== JSON.stringify(useCase.entities)
        );

        if (!hasChanges) return; // Skip if no actual changes

        if (saveTimeoutRef.current) {
            window.clearTimeout(saveTimeoutRef.current);
        }

        saveTimeoutRef.current = window.setTimeout(async () => {
            setLoading(true);
            try {
                // Update the use case with the form data
                const updatedUseCase: UseCaseDTO = {
                    ...useCase,
                    name: formData.name,
                    validator: formData.validator,
                    undoable: formData.undoable,
                    dto_in: formData.dto_in,
                    dto_out: formData.dto_out,
                    entities: selectedEntities,
                };

                // Call the context's update function
                updateUseCase(updatedUseCase);
                
                info("Use Case updated successfully");
            } catch (err) {
                error(`Failed to update use case: ${err}`);
            } finally {
                setLoading(false);
            }
        }, 500);

        return () => {
            if (saveTimeoutRef.current) {
                window.clearTimeout(saveTimeoutRef.current);
            }
        };
    }, [formData, selectedEntities, useCase, isLoadingUseCases, updateUseCase]);

    // TODO: Fetch entities for the entity dropdown
    // This would need to be moved to a custom hook or context

    const handleEntityChange = (entityId: number, checked: boolean) => {
        const newSelectedEntities = checked 
            ? [...selectedEntities, entityId]
            : selectedEntities.filter(id => id !== entityId);
        
        setSelectedEntities(newSelectedEntities);
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
                    <Stack>
                        <TextInput
                            id="useCaseName"
                            label="Name"
                            value={formData.name}
                            onChange={(e) => {
                                const newName = e.target.value;
                                setFormData({...formData, name: newName});
                            }}
                            disabled={loading}
                        />

                        <Checkbox
                            id="useCaseValidator"
                            label="Validator"
                            checked={formData.validator}
                            onChange={(e) => {
                                const newValidator = e.target.checked;
                                setFormData({...formData, validator: newValidator});
                            }}
                            disabled={loading}
                        />

                        <Checkbox
                            id="useCaseUndoable"
                            label="Undoable"
                            checked={formData.undoable}
                            onChange={(e) => {
                                const newUndoable = e.target.checked;
                                setFormData({...formData, undoable: newUndoable});
                            }}
                            disabled={loading}
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
                                    disabled={loading}
                                />
                            ))}
                        </Stack>

                        <DtoSelector
                            label="DTO In"
                            value={formData.dto_in}
                            useCaseId={selectedUseCaseId}
                            isDtoOut={false}
                            onChange={(dtoId) => {
                                setFormData({...formData, dto_in: dtoId});
                            }}
                        />

                        <DtoSelector
                            label="DTO Out"
                            value={formData.dto_out}
                            useCaseId={selectedUseCaseId}
                            isDtoOut={true}
                            onChange={(dtoId) => {
                                setFormData({...formData, dto_out: dtoId});
                            }}
                        />
                    </Stack>
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
