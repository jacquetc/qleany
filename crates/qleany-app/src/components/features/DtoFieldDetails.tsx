import {useEffect, useState} from 'react';
import {Button, Checkbox, Select, Stack, TextInput, Title} from '@mantine/core';
import {error, info} from '@tauri-apps/plugin-log';
import {useDtoFields} from '../../hooks/useDtoFields';
import {DtoFieldDTO, DtoFieldType} from '../../services/dto-field-service';

interface DtoFieldDetailsProps {
    selectedDtoField: number | null;
    dtoId: number | null;
}

const DtoFieldDetails = ({selectedDtoField, dtoId}: DtoFieldDetailsProps) => {
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

    const [dtoFieldData, setDtoFieldData] = useState<DtoFieldDTO | null>(null);
    
    // Use the hook to access DTO fields data and operations
    const { dtoFields, updateDtoField, isLoading } = useDtoFields(dtoId);
    
    // Find the selected field in the dtoFields array
    useEffect(() => {
        if (selectedDtoField && dtoFields.length > 0) {
            const field = dtoFields.find(field => field.id === selectedDtoField);
            if (field) {
                setDtoFieldData(field);
                setFormData({
                    name: field.name,
                    field_type: field.field_type,
                    is_nullable: field.is_nullable,
                    is_list: field.is_list,
                });
            }
        }
    }, [selectedDtoField, dtoFields]);

    const handleSubmit = async (e: React.FormEvent) => {
        e.preventDefault();

        if (!dtoFieldData) return;

        try {
            // Update the DTO field with the form data
            const updatedDtoField: DtoFieldDTO = {
                ...dtoFieldData,
                name: formData.name,
                field_type: formData.field_type,
                is_nullable: formData.is_nullable,
                is_list: formData.is_list,
            };

            // Use the hook's updateDtoField method
            updateDtoField(updatedDtoField);
            
            // The hook will automatically refresh the data through React Query
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

                    <Button type="submit" loading={isLoading}>Save Changes</Button>
                </Stack>
            </form>
        </>
    );
};

export default DtoFieldDetails;
