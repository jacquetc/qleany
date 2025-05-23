import {useEffect, useState} from 'react';
import {createEntity, EntityDto, getEntityMulti} from "../controller/entity_controller";
import {Divider, Flex, Stack} from '@mantine/core';
import {listen} from '@tauri-apps/api/event';
import {error, info} from '@tauri-apps/plugin-log';
import {getRootRelationship, RootRelationshipField, setRootRelationship} from "../controller/root_controller.ts";
import EntityList from '../components/entities/EntityList.tsx';
import EntityDetails from '../components/entities/EntityDetails.tsx';
import FieldsList from '../components/entities/FieldsList.tsx';
import FieldDetails from '../components/entities/FieldDetails.tsx';

const Entities = () => {
    const [selectedEntity, setSelectedEntity] = useState<number | null>(0);
    const [entityData, setEntityData] = useState<EntityDto[]>([]);
    const [selectedField, setSelectedField] = useState<number | null>(0);

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
            const rootEntities = await getRootRelationship(1, RootRelationshipField.Entities) || [];

            await setRootRelationship({
                id: 1,
                field: RootRelationshipField.Entities,
                right_ids: [...rootEntities, newEntity.id],
            });

            // Update UI state
            setEntityData(prevData => [...prevData, newEntity]);

            // Optional: Select the newly created entity
            setSelectedEntity(newEntity.id);

            await info("Entity created successfully");
        } catch (err) {
            await error(`Failed to create entity: ${err}`);
            // You could add user-facing error handling here
            // e.g., setError("Failed to create entity")
        }
    }

    // Function to fetch entity data from the backend
    async function fetchEntityData() {
        const entityIds = await getRootRelationship(1, RootRelationshipField.Entities);
        const entities = await getEntityMulti(entityIds);
        setEntityData(entities.filter((entity) => entity !== null) as EntityDto[]);
    }

    useEffect(() => {
        fetchEntityData().catch((err => error(err)));

        // mounting the event listeners
        const unlisten_direct_access_entity_created = listen('direct_access_entity_created', (event) => {
            const payload = event.payload as { ids: string[] };
            info(`Entity created event received: ${payload.ids}`);

            fetchEntityData().catch((err => error(err)));
        });


        return () => {
            unlisten_direct_access_entity_created.then(f => f());
        };

    }, []);


    return (
        <Flex direction="row" style={{
            height: '100%',
            margin: '0 0px',
        }}>
            <Stack miw={300} style={{overflow: 'auto', height: '100%'}}>
                <EntityList
                    entities={entityData}
                    selectedEntity={selectedEntity}
                    onSelectEntity={setSelectedEntity}
                    onCreateEntity={createNewEntity}
                    onEntitiesReordered={fetchEntityData}
                />
            </Stack>
            <Divider orientation="vertical" mb={0} mt={0} ml={5} mr={5}
            ></Divider>

            <Stack flex={1} style={{overflow: 'auto', height: '100%'}}>
                <EntityDetails
                    selectedEntity={selectedEntity}
                    entities={entityData}
                    onEntityUpdated={fetchEntityData}
                />
                <FieldsList
                    selectedEntity={selectedEntity}
                    onSelectField={setSelectedField}
                />
            </Stack>
            {selectedField !== null && selectedField > 0 && (
                <>
                    <Divider orientation="vertical" style={{
                        height: '100%',
                        margin: '0 5px',
                    }}></Divider>

                    <Stack className="flex-1" style={{overflow: 'auto', height: '100%'}}>
                        <FieldDetails selectedField={selectedField}/>
                    </Stack>
                </>
            )}
        </Flex>
    );
}

export default Entities;
