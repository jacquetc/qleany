import {useEffect, useState} from 'react';
import {EntityDto, getEntity, getEntityMulti, updateEntity, updateEntityMulti} from "#controller/entity-controller.ts";
import {Button, Checkbox, Modal, Select, Stack, Text, TextInput, Title} from '@mantine/core';
import {error, info} from '@tauri-apps/plugin-log';
import {listen} from "@tauri-apps/api/event";

interface EntityDetailsProps {
    selectedEntity: number | null;
}

const EntityDetails = ({selectedEntity}: EntityDetailsProps) => {

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
    const [entitiesWithHeritage, setEntitiesWithHeritage] = useState<EntityDto[]>([]);

    // Update form data when selected entity changes
    useEffect(() => {
        if (selectedEntity) {
            async function fetchEntityData() {
                try {
                    if (!selectedEntity) {
                        return;
                    }
                    const entityData = await getEntity(selectedEntity);
                    if (entityData) {
                        setFormData({
                            name: entityData.name,
                            directAccess: entityData.allow_direct_access,
                            parent: entityData.parent,
                            only_for_heritage: entityData.only_for_heritage
                        });
                        // Fetch entities with heritage
                        const allEntities = await getEntityMulti([]);
                        const entities = allEntities.filter(entity => entity !== null) as EntityDto[];
                        const heritageEntities = entities.filter(entity => entity.only_for_heritage);
                        setEntitiesWithHeritage(heritageEntities);
                    }
                } catch (err) {
                    error(`Failed to fetch entity data: ${err}`);
                }
            }

            fetchEntityData().catch(err => error(err));

            const unlisten_direct_access_entity_updated = listen('direct_access_entity_updated', async (event) => {
                info(`Direct access entity updated event received in EntityDetails: ${event.payload}`);
                // Refresh the entity data
                await fetchEntityData();
            });

            const unlisten_direct_access_entity_removed = listen('direct_access_entity_removed', async (event) => {
                const payload = event.payload as { id: number };
                info(`Direct access entity removed event received in EntityDetails: ${payload.id}`);

                // If the removed entity is the currently selected one, reset form data
                if (selectedEntity === payload.id) {
                    setFormData({
                        name: '',
                        directAccess: true,
                        parent: null,
                        only_for_heritage: false
                    });
                }

                // Refresh the entity data
                await fetchEntityData();
            });

            const unlisten_direct_access_all_reset = listen('direct_access_all_reset', () => {
                info(`Direct access all reset event received in EntityDetails`);
                fetchEntityData().catch(err => error(err));
            });

            return () => {
                unlisten_direct_access_entity_updated.then(f => f());
                unlisten_direct_access_entity_removed.then(f => f());
                unlisten_direct_access_all_reset.then(f => f());
            };


        }
    }, [selectedEntity]);

    const handleSubmit = async (e: React.FormEvent) => {
        e.preventDefault();

        if (!selectedEntity) {
            return;
        }
        // Find the selected entity in the data
        const selectedEntityData = await getEntity(selectedEntity);

        if (selectedEntityData) {
            try {

                // Check if the only_for_heritage field is being unchecked
                if (formData.only_for_heritage && !tempFormData?.only_for_heritage) {
                    // update all entities that have this entity as a parent so that to remove the parent
                    const allEntities = await getEntityMulti([]);
                    const entities = allEntities.filter(entity => entity !== null) as EntityDto[];
                    const entitiesToUpdate = entities.filter(entity => entity.parent === selectedEntity);
                    const updatedEntities = entitiesToUpdate.map(entity => ({
                        ...entity,
                        parent: null
                    }));
                    await updateEntityMulti(updatedEntities);
                }

                // Update the entity with the form data
                const updatedEntity = {
                    ...selectedEntityData,
                    name: formData.name,
                    parent: formData.parent,
                    only_for_heritage: formData.only_for_heritage
                };


                // Call the API to update the entity
                await updateEntity(updatedEntity);

                // Notify parent component to refresh data
                //onEntityUpdated();

                info("Entity updated successfully");
            } catch (err) {
                error(`Failed to update entity: ${err}`);
            }
        }
    };

    const renderContent = () => {
        if (!selectedEntity) {
            return null;
        }


        return (
            <>
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
                        <Button type="submit">Save Changes</Button>
                    </Stack>
                </form>
            </>
        );
    };

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
        <>
            {confirmationModal}
            {renderContent()}
        </>
    );
};

export default EntityDetails;