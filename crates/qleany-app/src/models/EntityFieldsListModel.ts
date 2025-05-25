import {useEffect, useState} from 'react';
import {listen} from "@tauri-apps/api/event";
import {error, info} from '@tauri-apps/plugin-log';
import {createField, FieldDto, FieldType, getFieldMulti} from "@/controller/field-controller.ts";
import {EntityRelationshipField, getEntityRelationship, setEntityRelationship} from "@/controller/entity-controller.ts";
import {beginComposite, endComposite} from "#controller/undo-redo-controller.ts";
import {useDebouncedCallback} from "@mantine/hooks";

export interface EntityFieldsListModelProps {
    entityId: number | null;
    onFieldsChanged: (fields: FieldDto[]) => void;
}

export function useEntityFieldsListModel(
    {
        entityId,
        onFieldsChanged
    }
    : EntityFieldsListModelProps) {
    const [fields, setFields] = useState<FieldDto[]>([]);

    // Function to fetch field data from the backend
    async function fetchFieldData() {
        if (!entityId) {
            await error("No entity selected");
            return [];
        }
        const fieldIds = await getEntityRelationship(entityId, EntityRelationshipField.Fields);
        const fields = await getFieldMulti(fieldIds);
        const filteredFields = fields.filter((field) => field !== null) as FieldDto[];

        setFields(filteredFields);
        onFieldsChanged(filteredFields);

        return filteredFields;
    }

    async function createNewField() {
        try {
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
            await beginComposite()
            const newField = await createField(dto);

            if (!entityId) {
                return;
            }

            // Update field relationship
            const entityFields = await getEntityRelationship(entityId, EntityRelationshipField.Fields) || [];

            await setEntityRelationship({
                id: entityId,
                field: EntityRelationshipField.Fields,
                right_ids: [...entityFields, newField.id],
            });
            await endComposite();

            // Update fields state
            const updatedFields = [...fields, newField];
            setFields(updatedFields);
            onFieldsChanged(updatedFields);

            await info("Field created successfully");
            return newField;
        } catch (err) {
            await error(`Failed to create field: ${err}`);
            throw err;
        }
    }

    // Function to handle reordering of fields
    async function handleReorder(reorderedIds: number[]): Promise<void> {
        try {
            if (!entityId) {
                await error("No entity selected for reordering fields");
                return;
            }

            // Update the entity relationship with the new order
            await setEntityRelationship({
                id: entityId,
                field: EntityRelationshipField.Fields,
                right_ids: reorderedIds,
            });

            info("Field order updated successfully");
            await fetchFieldData();
        } catch (err) {
            error(`Failed to update field order: ${err}`);
            throw err;
        }
    }

    const entityUpdaterHandler = useDebouncedCallback(async (event) => {
            const payload = event.payload as { ids: number[] };

            if (!entityId) {
                return;
            }

            if (!payload.ids.includes(entityId)) {
                return; // Ignore updates for other entities
            }

            info(`Entity updated event received: ${payload.ids}`);
            const updatedEntities = await getEntityRelationship(entityId, EntityRelationshipField.Fields);

            // If the fields relationship has changed, fetch the updated fields
            const fieldsIds = fields.map(field => field.id);

            if (JSON.stringify(updatedEntities) !== JSON.stringify(fieldsIds)) {
                info(`Fields relationship has changed for entity ${entityId}, fetching updated fields`);
                await fetchFieldData().catch((err) => error(err));
            } else {
                info(`Fields relationship has not changed for entity ${entityId}`);
            }
        }
        , 1000);

    const fieldUpdaterHandler = useDebouncedCallback(async (event) => {
            const payload = event.payload as { ids: number[] };
            info(`Field updated event received: ${payload.ids}`);
            const updatedFields = await getFieldMulti(payload.ids);

            for (const updatedField of updatedFields) {

                if (!updatedField) {
                    info(`Field not found in the current state.`);
                    continue;
                }
                const index = fields.findIndex((field) => field.id === updatedField.id);
                if (index !== -1) {
                    const updatedFieldsList = [...fields];
                    updatedFieldsList[index] = updatedField;
                    setFields(updatedFieldsList);
                    onFieldsChanged(updatedFieldsList);
                } else {
                    info(`Field not found in the current state.`);
                }
            }
        }
        , 1000);

    // Setup event listeners
    useEffect(() => {
        fetchFieldData().catch((err) => error(err));

        // mounting the event listeners
        const unlisten_direct_access_field_created = listen('direct_access_field_created', (event) => {
            const payload = event.payload as { ids: string[] };
            info(`Field created event received: ${payload.ids}`);

            fetchFieldData().catch((err) => error(err));
        });

        // Listen for field removal events
        const unlisten_direct_access_field_removed = listen('direct_access_field_removed', async (event) => {
            const payload = event.payload as { ids: number[] };
            info(`Field removed event received: ${payload.ids}`);

            // Filter out the removed fields from the current state
            const updatedFields = fields.filter(field => !payload.ids.includes(field.id));
            setFields(updatedFields);
            onFieldsChanged(updatedFields);
        });

        // Listen for field updates


        const unlisten_direct_access_field_updated = listen('direct_access_field_updated', fieldUpdaterHandler);

        // listen to any entity update event, filter to the current entity, check if the "fields" relationship has changed
        // and update the fields state accordingly


        const unlisten_direct_access_entity_updated = listen('direct_access_entity_updated', entityUpdaterHandler);

        const unlisten_direct_access_all_reset = listen('direct_access_all_reset', () => {
            info(`Direct access all reset event received`);
            fetchFieldData().then((dtos) => info(`Fields data reset successfully: ${JSON.stringify(dtos)}`)
            ).catch((err) => error(err));
        });

        return () => {
            unlisten_direct_access_field_created.then(f => f());
            unlisten_direct_access_field_removed.then(f => f());
            unlisten_direct_access_field_updated.then(f => f());
            unlisten_direct_access_entity_updated.then(f => f());
            unlisten_direct_access_all_reset.then(f => f());
        };
    }, [fields, entityId]);

    return {
        fields,
        createNewField,
        handleReorder,
        fetchFieldData
    };
}