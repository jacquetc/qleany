import {useEffect, useState} from 'react';
import {Button, Checkbox, Select, Stack, TextInput, Title, Alert} from '@mantine/core';
import {error, info} from '@tauri-apps/plugin-log';
import {useEntityContext} from '@/contexts/EntityContext';
import {FieldDTO, FieldType} from '@/services/field-service';
import ErrorBoundary from '@/components/ErrorBoundary';
import {useFields} from '@/hooks/useFields';

const FieldDetails = () => {
    const {
        entities,
        selectedFieldId
    } = useEntityContext();
    
    // Use the useFields hook to access field data and operations
    const {
        fields,
        isLoading: isLoadingFields,
        error: fieldError,
        updateField
    } = useFields(null); // Pass null to get all fields
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

    // Find the selected field from the fields array
    const selectedField = selectedFieldId 
        ? fields.find(f => f.id === selectedFieldId) || null 
        : null;

    // Update form data when selected field changes
    useEffect(() => {
        if (selectedField) {
            setFormData({
                name: selectedField.name,
                field_type: selectedField.field_type as FieldType,
                entity: selectedField.entity,
                is_nullable: selectedField.is_nullable,
                is_primary_key: selectedField.is_primary_key,
                is_list: selectedField.is_list,
                single: selectedField.single,
                strong: selectedField.strong,
                ordered: selectedField.ordered,
                list_model: selectedField.list_model,
                list_model_displayed_field: selectedField.list_model_displayed_field,
            });
        } else {
            // Reset form data if no field is selected
            setFormData({
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
        }
    }, [selectedField, fields]);

    const handleSubmit = async (e: React.FormEvent) => {
        e.preventDefault();

        if (!selectedField) return;

        try {
            const updatedField: FieldDTO = {
                ...selectedField,
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

            // Use the updateField function from the useFields hook
            updateField(updatedField);
            info("Field updated successfully");
        } catch (err) {
            error(`Failed to update field: ${err}`);
        }
    };

    // Custom fallback component for error state
    const errorFallback = (
        <Alert color="yellow" title="Field details could not be loaded">
            There was an issue loading the field details. Please try again later.
        </Alert>
    );

    // Loading state
    if (isLoadingFields) {
        return (
            <Alert color="blue" title="Loading field details">
                Please wait...
            </Alert>
        );
    }

    // Error state
    if (fieldError) {
        return (
            <Alert color="red" title="Error loading field details">
                {fieldError instanceof Error ? fieldError.message : 'An unknown error occurred'}
            </Alert>
        );
    }

    // No field selected state
    if (!selectedFieldId || !selectedField) {
        return (
            <Alert color="gray" title="No field selected">
                Please select a field to view its details.
            </Alert>
        );
    }

    return (
        <ErrorBoundary fallback={errorFallback}>
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

                    <Button type="submit" loading={isLoadingFields}>Save Changes</Button>
                </Stack>
            </form>
        </ErrorBoundary>
    );
};

export default FieldDetails;
