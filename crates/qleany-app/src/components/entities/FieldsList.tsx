import {ReactNode, useEffect, useState} from 'react';
import {ActionIcon, Group, Title, Tooltip} from '@mantine/core';
import {
    EntityRelationshipField,
    getEntity,
    setEntityRelationship,
    updateEntity
} from "#controller/entity_controller.ts";
import {error, info} from '@tauri-apps/plugin-log';
import {createField, FieldDto, FieldType, getFieldMulti} from "#controller/field_controller.ts";
import ReorderableList from '../ReorderableList.tsx';
import {listen} from '@tauri-apps/api/event';
import {beginComposite, endComposite} from "#controller/undo_redo_controller.ts";

interface FieldsListProps {
    selectedEntity: number | null;
    onSelectField: (fieldId: number | null) => void;
}

const FieldsList = ({
                        selectedEntity, onSelectField
                    }: FieldsListProps) => {
    const [fields, setFields] = useState<FieldDto[]>([]);
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

            await beginComposite();

            const newField = await createField(dto);

            // Update entity with the new field
            const updatedFields = [...entity.fields, newField.id];
            const updatedEntity = {
                ...entity,
                fields: updatedFields
            };

            await updateEntity(updatedEntity);

            await endComposite();

            // Select the newly created field
            setSelectedField(newField.id);

            await info("Field created successfully");
        } catch (err) {
            await error(`Failed to create field: ${err}`);
        }
    }

    useEffect(() => {

        // mounting the event listeners
        const unlisten_direct_access_field_updated = listen('direct_access_field_updated', async (event) => {
            const payload = event.payload as { ids: number[] };
            info(`Field updated event received: ${payload.ids}`);
            const updatedFields = await getFieldMulti(payload.ids);

            for (const updatedField of updatedFields) {
                if (!updatedField) {
                    info(`Use case not found in the current state.`);
                    continue;
                }
                const index = fields.findIndex((field) => field.id === updatedField.id);
                if (index !== -1) {
                    const updatedFieldsList = [...fields];
                    updatedFieldsList[index] = updatedField;
                    setFields(updatedFieldsList);
                } else {
                    info(`Use case not found in the current state.`);
                }

            }

        });

        const unlisten_direct_access_field_removed = listen('direct_access_field_removed', async (event) => {
            const payload = event.payload as { ids: number[] };
            info(`Field removed event received: ${payload.ids}`);
            const updatedFields = fields.filter((field) => !payload.ids.includes(field.id));
            setFields(updatedFields);

            // If the selected field was removed, clear the selection
            if (selectedField && payload.ids.includes(selectedField)) {
                setSelectedField(null);
                onSelectField(null);
            }
        });

        const unlisten_direct_access_field_created = listen('direct_access_field_created', async (event) => {
            const payload = event.payload as { ids: number[] };
            info(`Field created event received: ${payload.ids}`);
            const newFields = await getFieldMulti(payload.ids);
            setFields(prevFields => [...prevFields, ...newFields.filter((field): field is FieldDto => field !== null)]);
        });

        const unlisten_direct_access_all_reset = listen('direct_access_all_reset', () => {
            info(`Direct access all reset event received in FieldsList`);
            fetchFieldData().catch((err => error(err)));
        });

        return () => {
            unlisten_direct_access_field_updated.then(f => f());
            unlisten_direct_access_field_removed.then(f => f());
            unlisten_direct_access_field_created.then(f => f());
            unlisten_direct_access_all_reset.then(f => f());
        };

    }, [fields]);

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

    // Define onReorder function to handle reordering
    const handleReorder = async (reorderedIds: number[]): Promise<void> => {
        if (!selectedEntity) return;

        try {
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
    };

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
