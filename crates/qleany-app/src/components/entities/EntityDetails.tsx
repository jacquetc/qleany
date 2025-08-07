import { useEffect, useState } from 'react';
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

    // Update form data when selected entity changes
    useEffect(() => {
        if (selectedEntity) {
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

    const handleSubmit = async (e: React.FormEvent) => {
        e.preventDefault();

        if (!selectedEntity) {
            return;
        }

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
                parent: formData.parent,
                only_for_heritage: formData.only_for_heritage
            };

            // Call the context's update function
            await updateEntity(updatedEntity);
            
            // Refetch all data to ensure everything is up to date
            await refetchAll();

            info("Entity updated successfully");
        } catch (err) {
            error(`Failed to update entity: ${err}`);
        } finally {
            setLoading(false);
        }
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
                            setFormData(tempFormData);
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
            <form onSubmit={handleSubmit}>
                <Stack>
                    <TextInput
                        id="entityName"
                        label="Name"
                        value={formData.name}
                        onChange={(e) => setFormData({...formData, name: e.target.value})}
                    />
                    <Checkbox
                        id="directAccess"
                        label="Direct access"
                        checked={formData.directAccess}
                        onChange={(e) => setFormData({...formData, directAccess: e.target.checked})}
                    />
                    <Select
                        id="entityParent"
                        label="Parent"
                        placeholder="Select a parent entity"
                        value={formData.parent !== null ? formData.parent.toString() : ''}
                        onChange={(value) => {
                            const parentValue = !value || value === '' ? null : parseInt(value, 10);
                            setFormData({...formData, parent: parentValue});
                        }}
                        data={
                            entitiesWithHeritage.map(entity => ({
                                value: entity.id.toString(),
                                label: entity.name
                            }))
                        }
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
                                setFormData({...formData, only_for_heritage: e.target.checked});
                            }
                        }}
                    />
                    <Button type="submit" loading={loading}>Save Changes</Button>
                </Stack>
            </form>
        </ErrorBoundary>
    );
};

export default EntityDetails;