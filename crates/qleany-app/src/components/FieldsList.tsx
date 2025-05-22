import {useEffect, useState} from 'react';
import {ActionIcon, Group, Title, Tooltip} from '@mantine/core';
import {EntityRelationshipField, getEntity, setEntityRelationship, updateEntity} from "../controller/entity_controller";
import {DragDropContext, Draggable, Droppable} from '@hello-pangea/dnd';
import {IconGripVertical} from '@tabler/icons-react';
import cx from 'clsx';
import {useListState} from '@mantine/hooks';
import classes from '../routes/DndListHandle.module.css';
import {error, info} from '@tauri-apps/plugin-log';
import {createField, FieldDto, FieldType, getFieldMulti} from "../controller/field_controller.ts";

interface FieldsListProps {
    selectedEntity: number | null;
    onSelectField: (fieldId: number | null) => void;
}

const FieldsList = ({
                        selectedEntity, onSelectField
                    }: FieldsListProps) => {
    const [fields, setFields] = useState<FieldDto[]>([]);
    const [listState, handlers] = useListState<FieldDto>([]);
    const [selectedField, setSelectedField] = useState<number | null>(null);

    // Find the selected entity data

    async function createNewField() {
        if (!selectedEntity) {
            await error("No entity selected");
            return;
        }

        try {
            const entity = await getEntity(selectedEntity);
            // Find the selected entity
            if (!entity) {
                await error("Selected entity not found");
                return;
            }

            // Create field
            const dto = {
                name: 'New Field',
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
            };

            const newField = await createField(dto);

            // Update entity with the new field
            const updatedFields = [...entity.fields, newField.id];
            const updatedEntity = {
                ...entity,
                fields: updatedFields
            };

            await updateEntity(updatedEntity);

            // Refresh data
            await fetchFieldData();

            // Select the newly created field
            setSelectedField(newField.id);

            await info("Field created successfully");
        } catch (err) {
            await error(`Failed to create field: ${err}`);
        }
    }

    // Function to fetch field data from the backend
    async function fetchFieldData() {
        if (!selectedEntity) return;

        const entity = await getEntity(selectedEntity);
        if (!entity) return;

        const fieldIds = entity.fields;
        const fields = await getFieldMulti(fieldIds);
        setFields(fields.filter((field) => field !== null) as FieldDto[]);

    }

    // change fields when selected entity changes
    useEffect(() => {

        if (selectedEntity) {
            // Fetch field data using the field IDs from the selected entity
            const fetchFields = async () => {
                try {
                    const entityData = await getEntity(selectedEntity);
                    if (!entityData) {
                        setFields([]);
                        return;
                    }
                    const fieldData = await getFieldMulti(entityData.fields);
                    const validFields = fieldData.filter((field): field is FieldDto => field !== null);
                    setFields(validFields);

                    setSelectedField(null);
                    onSelectField(null);


                } catch (err) {
                    error(`Failed to fetch fields: ${err}`);
                }
            };

            fetchFields();
        } else {
            setFields([]);
        }
    }, [selectedEntity]);

    useEffect(() => {
        handlers.setState(fields);
    }, [fields]);

    if (!selectedEntity) {
        return null;
    }

    const items = listState.map((field, index) => (
        <Draggable key={field.id} index={index} draggableId={`field-${field.id}`}>
            {(provided, snapshot) => (
                <Group
                    align="left"
                    className={cx(classes.item, {
                        [classes.itemDragging]: snapshot.isDragging,
                        [classes.itemSelected]: field.id === selectedField
                    })}
                    ref={provided.innerRef}
                    onClick={() => {
                        setSelectedField(field.id);
                        onSelectField(field.id)
                    }
                    }
                    {...provided.draggableProps}
                >
                    <div {...provided.dragHandleProps} className={`${classes.dragHandle} cursor-grab`}>
                        <IconGripVertical size={18} stroke={1.5}/>
                    </div>
                    <div>
                        <div>{field.name}</div>
                        <div style={{color: 'dimmed', fontSize: 'small'}}>
                            {field.field_type}
                            {field.is_primary_key ? ' (PK)' : ''}
                            {field.is_nullable ? ' (nullable)' : ''}
                        </div>
                    </div>
                </Group>
            )}
        </Draggable>
    ));

    return (
        <>
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

            <DragDropContext
                onDragEnd={async ({destination, source}) => {
                    if (!destination || !selectedEntity) return;

                    // Update local state
                    handlers.reorder({from: source.index, to: destination.index});

                    // Persist changes to backend
                    try {
                        // Get the current order of fields
                        const reorderedIds = listState.map(field => field.id);

                        // Reorder the IDs based on the drag operation
                        const [removed] = reorderedIds.splice(source.index, 1);
                        reorderedIds.splice(destination.index, 0, removed);

                        // Update the entity with the new field order
                        await setEntityRelationship({
                            id: selectedEntity,
                            field: EntityRelationshipField.Fields,
                            right_ids: reorderedIds,
                        });

                        info("Field order updated successfully");
                        await fetchFieldData();
                    } catch (err) {
                        error(`Failed to update field order: ${err}`);
                    }
                }}
            >
                <Droppable droppableId="fields-list" direction="vertical" type="field">
                    {(provided) => (
                        <div
                            style={{
                                height: '100%',
                                maxHeight: '60vh',
                                overflow: 'auto',
                                flexGrow: 1
                            }}
                            {...provided.droppableProps}
                            ref={provided.innerRef}
                        >
                            {items}
                            {provided.placeholder}
                        </div>
                    )}
                </Droppable>
            </DragDropContext>
        </>
    );
};

export default FieldsList;
