import {useEffect, useState} from 'react';
import {listen} from "@tauri-apps/api/event";
import {error, info} from '@tauri-apps/plugin-log';
import {createEntity, EntityDto, getEntityMulti} from "#controller/entity-controller.ts";
import {
    getRootMulti,
    getRootRelationship,
    RootRelationshipField,
    setRootRelationship
} from "@/controller/root-controller.ts";

export interface EntityListModelCallbacks {
    onEntitiesChanged: (entities: EntityDto[]) => void;
}

export function useEntityListModel(callbacks: EntityListModelCallbacks) {
    const [entities, setEntities] = useState<EntityDto[]>([]);
    const [rootId, setRootId] = useState<number>(1);

    // Function to get the root ID
    async function getRootIdFromBackend() {
        const roots = await getRootMulti([]);
        if (roots.length > 0 && roots[0] !== null) {
            setRootId(roots[0]!.id);
            return roots[0]!.id;
        }
        return 1; // Fallback to default
    }

    // Function to fetch entity data from the backend
    async function fetchEntityData() {
        const currentRootId = await getRootIdFromBackend();
        const entityIds = await getRootRelationship(currentRootId, RootRelationshipField.Entities);
        const entities = await getEntityMulti(entityIds);
        const filteredEntities = entities.filter((entity) => entity !== null) as EntityDto[];

        setEntities(filteredEntities);
        callbacks.onEntitiesChanged(filteredEntities);

        return filteredEntities;
    }

    async function createNewEntity() {
        try {
            // Create entity
            const dto = {
                name: 'New Entity',
                only_for_heritage: false,
                parent: null,
                allow_direct_access: true,
                fields: [],
                relationships: [],
            };

            const newEntity = await createEntity(dto);

            // Update root relationship
            const currentRootId = await getRootIdFromBackend();
            const rootEntities = await getRootRelationship(currentRootId, RootRelationshipField.Entities) || [];

            await setRootRelationship({
                id: currentRootId,
                field: RootRelationshipField.Entities,
                right_ids: [...rootEntities, newEntity.id],
            });

            // Update entities state
            const updatedEntities = [...entities, newEntity];
            setEntities(updatedEntities);
            callbacks.onEntitiesChanged(updatedEntities);

            await info("Entity created successfully");
            return newEntity;
        } catch (err) {
            await error(`Failed to create entity: ${err}`);
            throw err;
        }
    }

    // Function to handle reordering of entities
    async function handleReorder(reorderedIds: number[]): Promise<void> {
        try {
            // Update the root relationship with the new order
            await setRootRelationship({
                id: rootId,
                field: RootRelationshipField.Entities,
                right_ids: reorderedIds,
            });

            info("Entity order updated successfully");
            await fetchEntityData();
        } catch (err) {
            error(`Failed to update entity order: ${err}`);
            throw err;
        }
    }

    // Setup event listeners
    useEffect(() => {
        fetchEntityData().catch((err) => error(err));

        // mounting the event listeners
        const unlisten_direct_access_entity_created = listen('direct_access_entity_created', (event) => {
            const payload = event.payload as { ids: string[] };
            info(`Entity created event received: ${payload.ids}`);

            fetchEntityData().catch((err) => error(err));
        });

        const unlisten_direct_access_entity_updated = listen('direct_access_entity_updated', async (event) => {
                const payload = event.payload as { ids: number[] };
                info(`Entity updated event received: ${payload.ids}`);
                const updatedEntities = await getEntityMulti(payload.ids);

                for (const updatedEntity of updatedEntities) {

                    if (!updatedEntity) {
                        info(`Entity not found in the current state.`);
                        continue;
                    }
                    const index = entities.findIndex((entity) => entity.id === updatedEntity.id);
                    if (index !== -1) {
                        const updatedEntitiesList = [...entities];
                        updatedEntitiesList[index] = updatedEntity;
                        setEntities(updatedEntitiesList);
                        callbacks.onEntitiesChanged(updatedEntitiesList);
                    } else {
                        info(`Entity not found in the current state.`);
                    }
                }
            }
        );

        const unlisten_direct_access_all_reset = listen('direct_access_all_reset', () => {
            info(`Direct access all reset event received`);
            fetchEntityData().catch((err) => error(err));
        });

        return () => {
            unlisten_direct_access_entity_created.then(f => f());
            unlisten_direct_access_entity_updated.then(f => f());
            unlisten_direct_access_all_reset.then(f => f());
        };
    }, [entities]);

    return {
        entities,
        createNewEntity,
        handleReorder,
        fetchEntityData
    };
}