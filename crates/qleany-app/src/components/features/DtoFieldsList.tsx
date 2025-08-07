import {ReactNode, useEffect, useState} from 'react';
import {ActionIcon, Group, Title, Tooltip} from '@mantine/core';
import {DtoRelationshipField, dtoService} from "../../services/dto-service";
import {error, info} from '@tauri-apps/plugin-log';
import {DtoFieldDTO, DtoFieldType} from "../../services/dto-field-service";
import {useDtoFields} from "../../hooks/useDtoFields";
import ReorderableList from '../ReorderableList.tsx';

interface DtoFieldsListProps {
    selectedDto: number | null;
    onSelectDtoField: (fieldId: number | null) => void;
}


const DtoFieldsList = ({
                           selectedDto, onSelectDtoField
                       }: DtoFieldsListProps) => {
    const [selectedField, setSelectedField] = useState<number | null>(null);
    
    // Use the hook to access DTO fields data and operations
    const { 
        dtoFields, 
        createDtoField, 
        isLoading, 
        error: hookError 
    } = useDtoFields(selectedDto);

    async function createNewDtoField() {
        if (!selectedDto) {
            error("No DTO selected");
            return;
        }

        try {
            // Create field using the hook
            const fieldData = {
                name: 'New Field',
                fieldType: DtoFieldType.String,
                isNullable: false,
                isList: false,
            };

            // The hook handles creating the field and updating the DTO relationship
            const newField = await createDtoField(fieldData);

            // Select the newly created field
            if (newField) {
                setSelectedField(newField.id);
                onSelectDtoField(newField.id);
            }

            info("DTO field created successfully");
        } catch (err) {
            error(`Failed to create DTO field: ${err}`);
        }
    }

    // Reset selection when selected DTO changes
    useEffect(() => {
        setSelectedField(null);
        onSelectDtoField(null);
    }, [selectedDto, onSelectDtoField]);

    if (!selectedDto) {
        return null;
    }

    // Create header component for ReorderableList
    const header = (
        <Group id="dtoFieldsListHeading">
            <Title order={3}>DTO Fields</Title>
            <Tooltip label="Add new DTO field">
                <ActionIcon
                    variant="filled"
                    aria-label="Add new DTO field"
                    onClick={createNewDtoField}
                >
                    +
                </ActionIcon>
            </Tooltip>
        </Group>
    );

    // Define renderItemContent function for field items
    const renderFieldContent = (field: DtoFieldDTO): ReactNode => (
        <div>
            <div>{field.name}</div>
            <div style={{color: 'dimmed', fontSize: 'small'}}>
                {field.field_type}
                {field.is_nullable ? ' (nullable)' : ''}
                {field.is_list ? ' (list)' : ''}
            </div>
        </div>
    );

    // Define onReorder function to handle reordering
    const handleReorder = async (reorderedIds: number[]): Promise<void> => {
        if (!selectedDto) return;

        try {
            // Update the DTO with the new field order
            await dtoService.setDtoRelationship({
                id: selectedDto,
                field: DtoRelationshipField.Fields,
                right_ids: reorderedIds,
            });

            info("DTO field order updated successfully");
            // The hook will automatically refresh the data
        } catch (err) {
            error(`Failed to update DTO field order: ${err}`);
        }
    };

    // Define onSelectItem function
    const handleSelectItem = (fieldId: number): void => {
        setSelectedField(fieldId);
        onSelectDtoField(fieldId);
    };

    return (
        <ReorderableList
            items={dtoFields}
            selectedItemId={selectedField}
            onSelectItem={handleSelectItem}
            onReorder={handleReorder}
            getItemId={(field) => field.id}
            renderItemContent={renderFieldContent}
            droppableId="dto-fields-list"
            draggableIdPrefix="dto-field"
            itemType="dto-field"
            header={header}
        />
    );
};

export default DtoFieldsList;
