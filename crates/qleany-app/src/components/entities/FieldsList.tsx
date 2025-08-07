import { ReactNode } from 'react';
import { ActionIcon, Alert, Group, Title, Tooltip } from '@mantine/core';
import { FieldDTO } from '@/services/field-service';
import ReorderableList from '../ReorderableList.tsx';
import { useEntityContext } from '@/contexts/EntityContext';
import ErrorBoundary from '@/components/ErrorBoundary';

const FieldsList = () => {
    const {
        fields,
        selectedEntityId,
        selectedFieldId,
        isLoadingFields,
        fieldError,
        selectField,
        createField
    } = useEntityContext();

    // Custom fallback component for error state
    const errorFallback = (
        <Alert color="yellow" title="Fields could not be loaded">
            There was an issue loading the field list. Please try again later.
        </Alert>
    );

    // Loading state
    if (isLoadingFields) {
        return (
            <Alert color="blue" title="Loading fields">
                Please wait...
            </Alert>
        );
    }

    // Error state
    if (fieldError) {
        return (
            <Alert color="red" title="Error loading fields">
                {fieldError instanceof Error ? fieldError.message : 'An unknown error occurred'}
            </Alert>
        );
    }

    // No entity selected state
    if (!selectedEntityId) {
        return (
            <Alert color="gray" title="No entity selected">
                Please select an entity to view its fields.
            </Alert>
        );
    }

    // Create header component for ReorderableList
    const header = (
        <Group id="fieldsListHeading">
            <Title order={3}>Fields</Title>
            <Tooltip label="Add new field">
                <ActionIcon
                    variant="filled"
                    aria-label="Add new field"
                    onClick={() => createField()}
                >
                    +
                </ActionIcon>
            </Tooltip>
        </Group>
    );

    // Define renderItemContent function for field items
    const renderFieldContent = (field: FieldDTO): ReactNode => (
        <div>
            <div>{field.name}</div>
            <div style={{color: 'dimmed', fontSize: 'small'}}>
                {field.field_type}
                {field.is_id ? ' (PK)' : ''}
                {field.is_nullable ? ' (nullable)' : ''}
            </div>
        </div>
    );

    return (
        <ErrorBoundary fallback={errorFallback}>
            <ReorderableList
                items={fields}
                selectedItemId={selectedFieldId}
                onSelectItem={selectField}
                onReorder={async (reorderedIds) => {
                    // TODO: Implement reordering logic if needed
                    console.log("Reordering fields:", reorderedIds);
                }}
                getItemId={(field) => field.id}
                renderItemContent={renderFieldContent}
                droppableId="fields-list"
                draggableIdPrefix="field"
                itemType="field"
                header={header}
            />
        </ErrorBoundary>
    );
};

export default FieldsList;
