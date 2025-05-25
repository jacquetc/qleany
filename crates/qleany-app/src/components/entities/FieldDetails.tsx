import {useEffect, useState} from 'react';
import {FieldDto, FieldType, getField, updateField} from "#controller/field_controller.ts";
import {Button, Checkbox, Select, Stack, TextInput, Title} from '@mantine/core';
import {error, info} from '@tauri-apps/plugin-log';
import {EntityDto, getEntityMulti} from "#controller/entity_controller.ts";
import {listen} from '@tauri-apps/api/event';

interface FieldDetailsProps {
    selectedField: number | null;
}

const FieldDetails = ({selectedField}: FieldDetailsProps) => {
    const [formData, setFormData] = useState<{
        name: string;
        field_type: FieldType;
        entity: number | null;
        is_nullable: boolean;
        is_primary_key: boolean;
        is_list: boolean;
        single: boolean;
        strong: boolean;
        ordered: boolean;
        list_model: boolean;
        list_model_displayed_field: string | null;
    }>({
        name: '',
        field_type: FieldType.String,
        entity: null,
        is_nullable: false,
        is_primary_key: false,
        is_list: false,
        single: true,
        strong: true,
        ordered: false,
        list_model: false,
        list_model_displayed_field: null,
    });

    const [fieldData, setFieldData] = useState<FieldDto | null>(null);
    const [entities, setEntities] = useState<EntityDto[]>([]);
    const [loading, setLoading] = useState(false);

    // Fetch field data when selected field changes
    useEffect(() => {
        if (selectedField) {
            const fetchFieldData = async () => {
                setLoading(true);
                try {
                    const data = await getField(selectedField);
                    if (data) {
                        setFieldData(data);
                        setFormData({
                            name: data.name,
                            field_type: data.field_type,
                            entity: data.entity,
                            is_nullable: data.is_nullable,
                            is_primary_key: data.is_primary_key,
                            is_list: data.is_list,
                            single: data.single,
                            strong: data.strong,
                            ordered: data.ordered,
                            list_model: data.list_model,
                            list_model_displayed_field: data.list_model_displayed_field,
                        });
                    }
                } catch (err) {
                    error(`Failed to fetch field data: ${err}`);
                } finally {
                    setLoading(false);
                }
            };

            fetchFieldData();
        }
    }, [selectedField]);

    // Fetch entities for the entity dropdown
    useEffect(() => {
        const fetchEntities = async () => {
            try {
                // This is a simplified approach - in a real app, you'd want to fetch only the entities
                // that can be referenced, not all entities
                const response = await getEntityMulti([]);
                const validEntities = response.filter((entity): entity is EntityDto => entity !== null);
                setEntities(validEntities);
            } catch (err) {
                error(`Failed to fetch entities: ${err}`);
            }
        };

        fetchEntities();

        // Listen for direct_access_all_reset event
        const unlisten_direct_access_all_reset = listen('direct_access_all_reset', () => {
            info(`Direct access all reset event received in FieldDetails`);
            if (selectedField) {
                const fetchFieldData = async () => {
                    try {
                        const data = await getField(selectedField);
                        if (data) {
                            setFieldData(data);
                            setFormData({
                                name: data.name,
                                field_type: data.field_type,
                                entity: data.entity,
                                is_nullable: data.is_nullable,
                                is_primary_key: data.is_primary_key,
                                is_list: data.is_list,
                                single: data.single,
                                strong: data.strong,
                                ordered: data.ordered,
                                list_model: data.list_model,
                                list_model_displayed_field: data.list_model_displayed_field,
                            });
                        }
                    } catch (err) {
                        error(`Failed to fetch field data: ${err}`);
                    }
                };
                fetchFieldData();
            }
            fetchEntities();
        });

        return () => {
            unlisten_direct_access_all_reset.then(f => f());
        };
    }, [selectedField]);

    const handleSubmit = async (e: React.FormEvent) => {
        e.preventDefault();

        if (!fieldData) return;

        try {
            const updatedField: FieldDto = {
                ...fieldData,
                name: formData.name,
                field_type: formData.field_type,
                entity: formData.field_type === FieldType.Entity ? formData.entity : null,
                is_nullable: formData.is_nullable,
                is_primary_key: formData.is_primary_key,
                is_list: formData.is_list,
                single: formData.single,
                strong: formData.strong,
                ordered: formData.ordered,
                list_model: formData.list_model,
                list_model_displayed_field: formData.list_model_displayed_field,
            };

            await updateField(updatedField);
            setFieldData(updatedField);
            info("Field updated successfully");
        } catch (err) {
            error(`Failed to update field: ${err}`);
        }
    };

    if (!selectedField || !fieldData) {
        return null;
    }

    return (
        <>
            <Title order={2}>"{formData.name}" details</Title>
            <form onSubmit={handleSubmit}>
                <Stack>
                    <TextInput
                        id="fieldName"
                        label="Name"
                        value={formData.name}
                        onChange={(e) => setFormData({...formData, name: e.target.value})}
                    />

                    <Select
                        id="fieldType"
                        label="Type"
                        value={formData.field_type}
                        onChange={(value) => {
                            if (value) {
                                setFormData({
                                    ...formData,
                                    field_type: value as FieldType,
                                    // Reset entity if type is not Entity
                                    entity: value === FieldType.Entity ? formData.entity : null
                                });
                            }
                        }}
                        data={Object.values(FieldType).map(type => ({
                            value: type,
                            label: type
                        }))}
                    />

                    {formData.field_type === FieldType.Entity && (
                        <Select
                            id="fieldEntity"
                            label="Referenced Entity"
                            placeholder="Select an entity"
                            value={formData.entity !== null ? formData.entity.toString() : ''}
                            onChange={(value) => {
                                const entityValue = !value || value === '' ? null : parseInt(value, 10);
                                setFormData({...formData, entity: entityValue});
                            }}
                            data={[
                                {value: '', label: 'None'},
                                ...entities.map(entity => ({
                                    value: entity.id.toString(),
                                    label: entity.name
                                }))
                            ]}
                        />
                    )}

                    <Checkbox
                        id="fieldNullable"
                        label="Nullable"
                        checked={formData.is_nullable}
                        onChange={(e) => setFormData({...formData, is_nullable: e.target.checked})}
                    />

                    <Checkbox
                        id="fieldPrimaryKey"
                        label="Primary Key"
                        checked={formData.is_primary_key}
                        onChange={(e) => setFormData({...formData, is_primary_key: e.target.checked})}
                    />

                    <Checkbox
                        id="fieldIsList"
                        label="Is List"
                        checked={formData.is_list}
                        onChange={(e) => setFormData({...formData, is_list: e.target.checked})}
                    />

                    {formData.field_type === FieldType.Entity && (
                        <>
                            <Checkbox
                                id="fieldSingle"
                                label="Single"
                                checked={formData.single}
                                onChange={(e) => setFormData({...formData, single: e.target.checked})}
                            />

                            <Checkbox
                                id="fieldStrong"
                                label="Strong"
                                checked={formData.strong}
                                onChange={(e) => setFormData({...formData, strong: e.target.checked})}
                            />

                            <Checkbox
                                id="fieldOrdered"
                                label="Ordered"
                                checked={formData.ordered}
                                onChange={(e) => setFormData({...formData, ordered: e.target.checked})}
                            />
                        </>
                    )}

                    <Checkbox
                        id="fieldListModel"
                        label="List Model"
                        checked={formData.list_model}
                        onChange={(e) => setFormData({...formData, list_model: e.target.checked})}
                    />

                    {formData.list_model && (
                        <TextInput
                            id="fieldListModelDisplayedField"
                            label="List Model Displayed Field"
                            value={formData.list_model_displayed_field || ''}
                            onChange={(e) => setFormData({
                                ...formData,
                                list_model_displayed_field: e.target.value || null
                            })}
                        />
                    )}

                    <Button type="submit" loading={loading}>Save Changes</Button>
                </Stack>
            </form>
        </>
    );
};

export default FieldDetails;
