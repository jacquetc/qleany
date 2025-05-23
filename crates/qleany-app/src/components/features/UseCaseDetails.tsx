import {useEffect, useState} from 'react';
import {
    getUseCase,
    setUseCaseRelationship,
    updateUseCase,
    UseCaseDto,
    UseCaseRelationshipField
} from "../../controller/use_case_controller.ts";
import {Button, Checkbox, Stack, Tabs, TextInput, Title} from '@mantine/core';
import {error, info} from '@tauri-apps/plugin-log';
import {EntityDto, getEntityMulti} from "../../controller/entity_controller.ts";
import DtoSelector from './DtoSelector.tsx';
import DtoDetails from './DtoDetails.tsx';

interface UseCaseDetailsProps {
    selectedUseCase: number | null;
}

const UseCaseDetails = ({selectedUseCase}: UseCaseDetailsProps) => {
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

    const [useCaseData, setUseCaseData] = useState<UseCaseDto | null>(null);
    const [entities, setEntities] = useState<EntityDto[]>([]);
    const [selectedEntities, setSelectedEntities] = useState<number[]>([]);
    const [loading, setLoading] = useState(false);
    const [activeTab, setActiveTab] = useState<string | null>("details");

    // Fetch use case data when selected use case changes
    useEffect(() => {
        if (selectedUseCase) {
            const fetchUseCaseData = async () => {
                setLoading(true);
                try {
                    const data = await getUseCase(selectedUseCase);
                    if (data) {
                        setUseCaseData(data);
                        setFormData({
                            name: data.name,
                            validator: data.validator,
                            undoable: data.undoable,
                            dto_in: data.dto_in,
                            dto_out: data.dto_out,
                        });
                        setSelectedEntities(data.entities);
                    }
                } catch (err) {
                    error(`Failed to fetch use case data: ${err}`);
                } finally {
                    setLoading(false);
                }
            };

            fetchUseCaseData();
        }
    }, [selectedUseCase]);

    // Fetch entities for the entity dropdown
    useEffect(() => {
        const fetchEntities = async () => {
            try {
                const response = await getEntityMulti([]);
                const validEntities = response.filter((entity): entity is EntityDto => entity !== null);
                setEntities(validEntities);
            } catch (err) {
                error(`Failed to fetch entities: ${err}`);
            }
        };

        fetchEntities();
    }, []);

    const handleSubmit = async (e: React.FormEvent) => {
        e.preventDefault();

        if (!useCaseData) return;

        try {
            // Update the use case with the form data
            const updatedUseCase: UseCaseDto = {
                ...useCaseData,
                name: formData.name,
                validator: formData.validator,
                undoable: formData.undoable,
                dto_in: formData.dto_in,
                dto_out: formData.dto_out,
            };

            await updateUseCase(updatedUseCase);

            // Update entities relationship
            await setUseCaseRelationship({
                id: useCaseData.id,
                field: UseCaseRelationshipField.Entities,
                right_ids: selectedEntities,
            });

            // Refresh data
            const refreshedData = await getUseCase(useCaseData.id);
            if (refreshedData) {
                setUseCaseData(refreshedData);
                setSelectedEntities(refreshedData.entities);
            }

            info("Use Case updated successfully");
        } catch (err) {
            error(`Failed to update use case: ${err}`);
        }
    };

    const handleEntityChange = (entityId: number, checked: boolean) => {
        if (checked) {
            setSelectedEntities(prev => [...prev, entityId]);
        } else {
            setSelectedEntities(prev => prev.filter(id => id !== entityId));
        }
    };

    if (!selectedUseCase || !useCaseData) {
        return null;
    }

    return (
        <>
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
                                useCaseId={selectedUseCase}
                                isDtoOut={false}
                                onChange={(dtoId) => setFormData({...formData, dto_in: dtoId})}
                            />

                            <DtoSelector
                                label="DTO Out"
                                value={formData.dto_out}
                                useCaseId={selectedUseCase}
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
        </>
    );
};

export default UseCaseDetails;
