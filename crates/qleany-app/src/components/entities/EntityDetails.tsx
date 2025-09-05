import { useEffect, useState, useRef } from 'react';
import { Alert, Button, Checkbox, Modal, Select, Stack, Text, TextInput, Title } from '@mantine/core';
import { error, info } from '@tauri-apps/plugin-log';
import { useEntityContext } from '@/contexts/EntityContext';
import { EntityDTO } from '@/services/entity-service';
import ErrorBoundary from '@/components/ErrorBoundary';

const EntityDetails = () => {
    const {
        entities,
        selectedEntityId,
        isLoadingEntities,
        entityError,
        updateEntity,
        refetchAll
    } = useEntityContext();

    // Find the selected entity from the entities array
    const selectedEntity = selectedEntityId 
        ? entities.find(e => e.id === selectedEntityId) || null 
        : null;

    const [formData, setFormData] = useState<{
        name: string;
        directAccess: boolean;
        parent: number | null;
        only_for_heritage: boolean;
    }>({
        name: '',
        directAccess: true,
        parent: null,
        only_for_heritage: false
    });

    // State for confirmation modal
    const [showConfirmModal, setShowConfirmModal] = useState(false);
    const [tempFormData, setTempFormData] = useState<typeof formData | null>(null);
    const [entitiesWithHeritage, setEntitiesWithHeritage] = useState<EntityDTO[]>([]);
    const [loading, setLoading] = useState(false);
    
    // Refs to prevent focus loss similar to ProjectContext
    const saveTimeoutRef = useRef<number | null>(null);
    const isLoadingDataRef = useRef(false);

    // Update form data when selected entity changes
    useEffect(() => {
        if (selectedEntity) {
            // Flag that we're loading data from external source
            isLoadingDataRef.current = true;
            
            // Update form data from the selected entity
            setFormData({
                name: selectedEntity.name,
                directAccess: selectedEntity.allow_direct_access,
                parent: selectedEntity.parent,
                only_for_heritage: selectedEntity.only_for_heritage
            });

            // Find entities with heritage
            const heritageEntities = entities.filter(entity => entity.only_for_heritage);
            setEntitiesWithHeritage(heritageEntities);
            
            // Reset flag after a brief timeout to allow state update to complete
            setTimeout(() => {
                isLoadingDataRef.current = false;
            }, 0);
        } else {
            // Reset form data if no entity is selected
            setFormData({
                name: '',
                directAccess: true,
                parent: null,
                only_for_heritage: false
            });
        }
    }, [selectedEntity, entities]);

    // Add auto-save effect with debouncing similar to ProjectContext
    useEffect(() => {
        if (isLoadingEntities) return;
        if (!selectedEntity) return; // Skip if entity data hasn't loaded yet
        if (isLoadingDataRef.current) return; // Skip if formData change is from external data loading

        // Check if formData actually differs from the current entity
        const hasChanges = (
            formData.name !== selectedEntity.name ||
            formData.directAccess !== selectedEntity.allow_direct_access ||
            formData.parent !== selectedEntity.parent ||
            formData.only_for_heritage !== selectedEntity.only_for_heritage
        );

        if (!hasChanges) return; // Skip if no actual changes

        if (saveTimeoutRef.current) {
            window.clearTimeout(saveTimeoutRef.current);
        }

        saveTimeoutRef.current = window.setTimeout(async () => {
            setLoading(true);
            try {
                // Check if the only_for_heritage field is being unchecked
                if (selectedEntity.only_for_heritage && !formData.only_for_heritage) {
                    // Find entities that have this entity as a parent
                    const entitiesToUpdate = entities.filter(entity => entity.parent === selectedEntityId);
                    
                    // Update each entity to remove the parent
                    for (const entity of entitiesToUpdate) {
                        const updatedEntity = {
                            ...entity,
                            parent: null
                        };
                        await updateEntity(updatedEntity);
                    }
                }

                // Update the entity with the form data
                const updatedEntity = {
                    ...selectedEntity,
                    name: formData.name,
                    allow_direct_access: formData.directAccess,
                    parent: formData.parent,
                    only_for_heritage: formData.only_for_heritage
                };

                // Call the context's update function (without refetchAll)
                await updateEntity(updatedEntity);
                
                info("Entity updated successfully");
            } catch (err) {
                error(`Failed to update entity: ${err}`);
            } finally {
                setLoading(false);
            }
        }, 500);

        return () => {
            if (saveTimeoutRef.current) {
                window.clearTimeout(saveTimeoutRef.current);
            }
        };
    }, [formData, selectedEntity, isLoadingEntities, updateEntity, entities, selectedEntityId]);

    const handleEntityUpdate = (updates: Partial<typeof formData>) => {
        if (!selectedEntity) return;
        setFormData(prev => ({...prev, ...updates}));
    };

    // Custom fallback component for error state
    const errorFallback = (
        <Alert color="yellow" title="Entity details could not be loaded">
            There was an issue loading the entity details. Please try again later.
        </Alert>
    );

    // Loading state
    if (isLoadingEntities) {
        return (
            <Alert color="blue" title="Loading entity details">
                Please wait...
            </Alert>
        );
    }

    // Error state
    if (entityError) {
        return (
            <Alert color="red" title="Error loading entity details">
                {entityError instanceof Error ? entityError.message : 'An unknown error occurred'}
            </Alert>
        );
    }

    // No entity selected state
    if (!selectedEntityId || !selectedEntity) {
        return (
            <Alert color="gray" title="No entity selected">
                Please select an entity to view its details.
            </Alert>
        );
    }

    // Confirmation modal for unchecking only_for_heritage
    const confirmationModal = (
        <Modal
            opened={showConfirmModal}
            onClose={() => setShowConfirmModal(false)}
            title="Warning: Removing Heritage Status"
            centered
        >
            <Text size="sm" mb="md">
                Removing the heritage status will remove this entity as a parent from any entities that currently use
                it.
                This may affect the structure of your data model.
            </Text>
            <Stack align="flex-end" gap="xs">
                <Button
                    color="red"
                    onClick={() => {
                        if (tempFormData) {
                            handleEntityUpdate({only_for_heritage: false});
                            setTempFormData(null);
                        }
                        setShowConfirmModal(false);
                    }}
                >
                    Confirm
                </Button>
                <Button
                    variant="outline"
                    onClick={() => {
                        setTempFormData(null);
                        setShowConfirmModal(false);
                    }}
                >
                    Cancel
                </Button>
            </Stack>
        </Modal>
    );

    return (
        <ErrorBoundary fallback={errorFallback}>
            {confirmationModal}
            <Title order={2}>"{formData.name}" details</Title>
            <Stack>
                <TextInput
                    id="entityName"
                    label="Name"
                    value={formData.name}
                    onChange={(e) => {
                        const newName = e.target.value;
                        handleEntityUpdate({name: newName});
                    }}
                    disabled={loading}
                />
                <Checkbox
                    id="directAccess"
                    label="Direct access"
                    checked={formData.directAccess}
                    onChange={(e) => {
                        const newDirectAccess = e.target.checked;
                        handleEntityUpdate({directAccess: newDirectAccess});
                    }}
                    disabled={loading}
                />
                <Select
                    id="entityParent"
                    label="Parent"
                    placeholder="Select a parent entity"
                    value={formData.parent !== null ? formData.parent.toString() : ''}
                    onChange={(value) => {
                        const parentValue = !value || value === '' ? null : parseInt(value, 10);
                        handleEntityUpdate({parent: parentValue});
                    }}
                    data={
                        entitiesWithHeritage.map(entity => ({
                            value: entity.id.toString(),
                            label: entity.name
                        }))
                    }
                    disabled={loading}
                />
                <Checkbox
                    id="heritage"
                    label="Heritage"
                    checked={formData.only_for_heritage}
                    onChange={(e) => {
                        // If changing from checked to unchecked, show confirmation
                        if (formData.only_for_heritage && !e.target.checked) {
                            setTempFormData({...formData, only_for_heritage: false});
                            setShowConfirmModal(true);
                        } else {
                            // Otherwise, update directly
                            const newHeritageValue = e.target.checked;
                            handleEntityUpdate({only_for_heritage: newHeritageValue});
                        }
                    }}
                    disabled={loading}
                />
            </Stack>
        </ErrorBoundary>
    );
};

export default EntityDetails;