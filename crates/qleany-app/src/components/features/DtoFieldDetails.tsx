import {useEffect, useRef, useState} from 'react';
import {Checkbox, Select, Stack, TextInput, Title} from '@mantine/core';
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

    // Refs for debouncing
    const saveTimeoutRef = useRef<number | null>(null);
    const isLoadingDataRef = useRef(false);

    // Use the hook to access DTO fields data and operations
    const {dtoFields, updateDtoField, isLoading} = useDtoFields(dtoId);

    // Find the selected field in the dtoFields array
    useEffect(() => {
        if (selectedDtoField && dtoFields.length > 0) {
            const field = dtoFields.find(field => field.id === selectedDtoField);
            if (field) {
                // Flag that we're loading data from external source
                isLoadingDataRef.current = true;

                setDtoFieldData(field);
                setFormData({
                    name: field.name,
                    field_type: field.field_type,
                    is_nullable: field.is_nullable,
                    is_list: field.is_list,
                });

                // Reset flag after a brief timeout to allow state update to complete
                setTimeout(() => {
                    isLoadingDataRef.current = false;
                }, 0);
            }
        }
    }, [selectedDtoField, dtoFields]);

    // Add auto-save effect with debouncing
    useEffect(() => {
        if (isLoading) return;
        if (!dtoFieldData) return; // Skip if field data hasn't loaded yet
        if (isLoadingDataRef.current) return; // Skip if formData change is from external data loading

        // Check if formData actually differs from the current field
        const hasChanges = (
            formData.name !== dtoFieldData.name ||
            formData.field_type !== dtoFieldData.field_type ||
            formData.is_nullable !== dtoFieldData.is_nullable ||
            formData.is_list !== dtoFieldData.is_list
        );

        if (!hasChanges) return; // Skip if no actual changes

        if (saveTimeoutRef.current) {
            window.clearTimeout(saveTimeoutRef.current);
        }

        saveTimeoutRef.current = window.setTimeout(async () => {
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
        }, 500);

        return () => {
            if (saveTimeoutRef.current) {
                window.clearTimeout(saveTimeoutRef.current);
            }
        };
    }, [formData, dtoFieldData, isLoading, updateDtoField]);


    if (!selectedDtoField || !dtoFieldData) {
        return null;
    }

    return (
        <>
            <Title order={3}>{formData.field_type} details</Title>
            <Stack>
                <TextInput
                    id="dtoFieldName"
                    label="Name"
                    value={formData.name}
                    onChange={(e) => {
                        const newName = e.target.value;
                        setFormData({...formData, name: newName});
                    }}
                    disabled={isLoading}
                />

                <Select
                    id="dtoFieldType"
                    label="Field Type"
                    value={formData.field_type}
                    onChange={(value) => {
                        if (value) {
                            const newFieldType = value as DtoFieldType;
                            setFormData({...formData, field_type: newFieldType});
                        }
                    }}
                    data={Object.values(DtoFieldType).map(type => ({
                        value: type,
                        label: type
                    }))}
                    disabled={isLoading}
                />

                <Checkbox
                    id="dtoFieldNullable"
                    label="Nullable"
                    checked={formData.is_nullable}
                    onChange={(e) => {
                        const newIsNullable = e.target.checked;
                        setFormData({...formData, is_nullable: newIsNullable});
                    }}
                    disabled={isLoading}
                />

                <Checkbox
                    id="dtoFieldList"
                    label="List"
                    checked={formData.is_list}
                    onChange={(e) => {
                        const newIsList = e.target.checked;
                        setFormData({...formData, is_list: newIsList});
                    }}
                    disabled={isLoading}
                />
            </Stack>
        </>
    );
};

export default DtoFieldDetails;
