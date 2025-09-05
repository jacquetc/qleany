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

    const handleFieldUpdate = async (updates: Partial<typeof formData>) => {
        if (!selectedField) return;

        try {
            const updatedField: FieldDTO = {
                ...selectedField,
                name: updates.name !== undefined ? updates.name : formData.name,
                field_type: updates.field_type !== undefined ? updates.field_type : formData.field_type,
                entity: (updates.field_type !== undefined ? updates.field_type : formData.field_type) === FieldType.Entity 
                    ? (updates.entity !== undefined ? updates.entity : formData.entity) 
                    : null,
                is_nullable: updates.is_nullable !== undefined ? updates.is_nullable : formData.is_nullable,
                is_primary_key: updates.is_primary_key !== undefined ? updates.is_primary_key : formData.is_primary_key,
                is_list: updates.is_list !== undefined ? updates.is_list : formData.is_list,
                single: updates.single !== undefined ? updates.single : formData.single,
                strong: updates.strong !== undefined ? updates.strong : formData.strong,
                ordered: updates.ordered !== undefined ? updates.ordered : formData.ordered,
                list_model: updates.list_model !== undefined ? updates.list_model : formData.list_model,
                list_model_displayed_field: updates.list_model_displayed_field !== undefined ? updates.list_model_displayed_field : formData.list_model_displayed_field,
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
            <Stack>
                <TextInput
                    id="fieldName"
                    label="Name"
                    value={formData.name}
                    onChange={(e) => {
                        const newName = e.target.value;
                        setFormData({...formData, name: newName});
                        handleFieldUpdate({name: newName});
                    }}
                    disabled={isLoadingFields}
                />

                <Select
                    id="fieldType"
                    label="Type"
                    value={formData.field_type}
                    onChange={(value) => {
                        if (value) {
                            const newFieldType = value as FieldType;
                            const newEntity = newFieldType === FieldType.Entity ? formData.entity : null;
                            setFormData({
                                ...formData,
                                field_type: newFieldType,
                                entity: newEntity
                            });
                            handleFieldUpdate({field_type: newFieldType, entity: newEntity});
                        }
                    }}
                    data={Object.values(FieldType).map(type => ({
                        value: type,
                        label: type
                    }))}
                    disabled={isLoadingFields}
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
                            handleFieldUpdate({entity: entityValue});
                        }}
                        data={[
                            {value: '', label: 'None'},
                            ...entities.map(entity => ({
                                value: entity.id.toString(),
                                label: entity.name
                            }))
                        ]}
                        disabled={isLoadingFields}
                    />
                )}

                <Checkbox
                    id="fieldNullable"
                    label="Nullable"
                    checked={formData.is_nullable}
                    onChange={(e) => {
                        const newValue = e.target.checked;
                        setFormData({...formData, is_nullable: newValue});
                        handleFieldUpdate({is_nullable: newValue});
                    }}
                    disabled={isLoadingFields}
                />

                <Checkbox
                    id="fieldPrimaryKey"
                    label="Primary Key"
                    checked={formData.is_primary_key}
                    onChange={(e) => {
                        const newValue = e.target.checked;
                        setFormData({...formData, is_primary_key: newValue});
                        handleFieldUpdate({is_primary_key: newValue});
                    }}
                    disabled={isLoadingFields}
                />

                <Checkbox
                    id="fieldIsList"
                    label="Is List"
                    checked={formData.is_list}
                    onChange={(e) => {
                        const newValue = e.target.checked;
                        setFormData({...formData, is_list: newValue});
                        handleFieldUpdate({is_list: newValue});
                    }}
                    disabled={isLoadingFields}
                />

                {formData.field_type === FieldType.Entity && (
                    <>
                        <Checkbox
                            id="fieldSingle"
                            label="Single"
                            checked={formData.single}
                            onChange={(e) => {
                                const newValue = e.target.checked;
                                setFormData({...formData, single: newValue});
                                handleFieldUpdate({single: newValue});
                            }}
                            disabled={isLoadingFields}
                        />

                        <Checkbox
                            id="fieldStrong"
                            label="Strong"
                            checked={formData.strong}
                            onChange={(e) => {
                                const newValue = e.target.checked;
                                setFormData({...formData, strong: newValue});
                                handleFieldUpdate({strong: newValue});
                            }}
                            disabled={isLoadingFields}
                        />

                        <Checkbox
                            id="fieldOrdered"
                            label="Ordered"
                            checked={formData.ordered}
                            onChange={(e) => {
                                const newValue = e.target.checked;
                                setFormData({...formData, ordered: newValue});
                                handleFieldUpdate({ordered: newValue});
                            }}
                            disabled={isLoadingFields}
                        />
                    </>
                )}

                <Checkbox
                    id="fieldListModel"
                    label="List Model"
                    checked={formData.list_model}
                    onChange={(e) => {
                        const newValue = e.target.checked;
                        setFormData({...formData, list_model: newValue});
                        handleFieldUpdate({list_model: newValue});
                    }}
                    disabled={isLoadingFields}
                />

                {formData.list_model && (
                    <TextInput
                        id="fieldListModelDisplayedField"
                        label="List Model Displayed Field"
                        value={formData.list_model_displayed_field || ''}
                        onChange={(e) => {
                            const newValue = e.target.value || null;
                            setFormData({
                                ...formData,
                                list_model_displayed_field: newValue
                            });
                            handleFieldUpdate({list_model_displayed_field: newValue});
                        }}
                        disabled={isLoadingFields}
                    />
                )}
            </Stack>
        </ErrorBoundary>
    );
};

export default FieldDetails;
