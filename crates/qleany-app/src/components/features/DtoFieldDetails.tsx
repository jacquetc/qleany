import {useEffect, useState} from 'react';
import {Button, Checkbox, Select, Stack, TextInput, Title} from '@mantine/core';
import {DtoFieldDto, DtoFieldType, getDtoField, updateDtoField} from "#controller/dto-field-controller.ts";
import {error, info} from '@tauri-apps/plugin-log';
import {listen} from '@tauri-apps/api/event';

interface DtoFieldDetailsProps {
    selectedDtoField: number | null;
}

const DtoFieldDetails = ({selectedDtoField}: DtoFieldDetailsProps) => {
    const [formData, setFormData] = useState<{
        name: string;
        field_type: DtoFieldType;
        is_nullable: boolean;
        is_list: boolean;
    }>({
        name: '',
        field_type: DtoFieldType.String,
        is_nullable: false,
        is_list: false,
    });

    const [dtoFieldData, setDtoFieldData] = useState<DtoFieldDto | null>(null);
    const [loading, setLoading] = useState(false);

    // Fetch DTO field data when selected DTO field changes
    useEffect(() => {
        if (selectedDtoField) {
            const fetchDtoFieldData = async () => {
                setLoading(true);
                try {
                    const data = await getDtoField(selectedDtoField);
                    if (data) {
                        setDtoFieldData(data);
                        setFormData({
                            name: data.name,
                            field_type: data.field_type,
                            is_nullable: data.is_nullable,
                            is_list: data.is_list,
                        });
                    }
                } catch (err) {
                    error(`Failed to fetch DTO field data: ${err}`);
                } finally {
                    setLoading(false);
                }
            };

            fetchDtoFieldData();

            // Listen for direct_access_all_reset event
            const unlisten_direct_access_all_reset = listen('direct_access_all_reset', () => {
                info(`Direct access all reset event received in DtoFieldDetails`);
                fetchDtoFieldData().catch((err => error(err)));
            });

            return () => {
                unlisten_direct_access_all_reset.then(f => f());
            };
        }
    }, [selectedDtoField]);

    const handleSubmit = async (e: React.FormEvent) => {
        e.preventDefault();

        if (!dtoFieldData) return;

        try {
            // Update the DTO field with the form data
            const updatedDtoField: DtoFieldDto = {
                ...dtoFieldData,
                name: formData.name,
                field_type: formData.field_type,
                is_nullable: formData.is_nullable,
                is_list: formData.is_list,
            };

            await updateDtoField(updatedDtoField);

            // Refresh data
            const refreshedData = await getDtoField(dtoFieldData.id);
            if (refreshedData) {
                setDtoFieldData(refreshedData);
            }

            info("DTO Field updated successfully");
        } catch (err) {
            error(`Failed to update DTO field: ${err}`);
        }
    };

    if (!selectedDtoField || !dtoFieldData) {
        return null;
    }

    return (
        <>
            <Title order={3}>"{formData.name}" details</Title>
            <form onSubmit={handleSubmit}>
                <Stack>
                    <TextInput
                        id="dtoFieldName"
                        label="Name"
                        value={formData.name}
                        onChange={(e) => setFormData({...formData, name: e.target.value})}
                    />

                    <Select
                        id="dtoFieldType"
                        label="Field Type"
                        value={formData.field_type}
                        onChange={(value) => {
                            if (value) {
                                setFormData({...formData, field_type: value as DtoFieldType});
                            }
                        }}
                        data={Object.values(DtoFieldType).map(type => ({
                            value: type,
                            label: type
                        }))}
                    />

                    <Checkbox
                        id="dtoFieldNullable"
                        label="Nullable"
                        checked={formData.is_nullable}
                        onChange={(e) => setFormData({...formData, is_nullable: e.target.checked})}
                    />

                    <Checkbox
                        id="dtoFieldList"
                        label="List"
                        checked={formData.is_list}
                        onChange={(e) => setFormData({...formData, is_list: e.target.checked})}
                    />

                    <Button type="submit" loading={loading}>Save Changes</Button>
                </Stack>
            </form>
        </>
    );
};

export default DtoFieldDetails;
