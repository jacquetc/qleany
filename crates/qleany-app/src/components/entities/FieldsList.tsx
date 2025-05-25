import {ReactNode, useState} from 'react';
import {ActionIcon, Group, Title, Tooltip} from '@mantine/core';
import {FieldDto} from "#controller/field_controller.ts";
import ReorderableList from '../ReorderableList.tsx';
import {useEntityFieldsListModel} from "#components/entities/EntityFieldsListModel.ts";

interface FieldsListProps {
    selectedEntity: number | null;
    onSelectField: (fieldId: number | null) => void;
}

const FieldsList = ({
                        selectedEntity, onSelectField
                    }: FieldsListProps) => {
    const [selectedField, setSelectedField] = useState<number | null>(null);

    // Use the entity list model
    const {fields, createNewField, handleReorder} = useEntityFieldsListModel({
        entityId: selectedEntity,
        onFieldsChanged: (__newFields) => {

            // If needed, you can perform additional actions when entities change
        }
    });

    if (!selectedEntity) {
        return null;
    }

    // Create header component for ReorderableList
    const header = (
        <Group id="fieldsListHeading">
            <Title order={3}>Fields</Title>
            <Tooltip label="Add new field">
                <ActionIcon
                    variant="filled"
                    aria-label="Add new field"
                    onClick={createNewField}
                >
                    +
                </ActionIcon>
            </Tooltip>
        </Group>
    );

    // Define renderItemContent function for field items
    const renderFieldContent = (field: FieldDto): ReactNode => (
        <div>
            <div>{field.name}</div>
            <div style={{color: 'dimmed', fontSize: 'small'}}>
                {field.field_type}
                {field.is_primary_key ? ' (PK)' : ''}
                {field.is_nullable ? ' (nullable)' : ''}
            </div>
        </div>
    );

    // Define onSelectItem function
    const handleSelectItem = (fieldId: number): void => {
        setSelectedField(fieldId);
        onSelectField(fieldId);
    };

    return (
        <ReorderableList
            items={fields}
            selectedItemId={selectedField}
            onSelectItem={handleSelectItem}
            onReorder={handleReorder}
            getItemId={(field) => field.id}
            renderItemContent={renderFieldContent}
            droppableId="fields-list"
            draggableIdPrefix="field"
            itemType="field"
            header={header}
        />
    );
};

export default FieldsList;
