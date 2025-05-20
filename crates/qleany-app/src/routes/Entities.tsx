import {useRef, useState} from 'react';
import {createEntity} from "../controller/entity_controller";
import {DragDropContext, Draggable, Droppable} from '@hello-pangea/dnd';
import {IconGripVertical} from '@tabler/icons-react';
import cx from 'clsx';
import {ActionIcon, Box, Divider, Flex, Group, ScrollArea, Stack, Text, Title, Tooltip} from '@mantine/core';
import {useListState} from '@mantine/hooks';
import classes from './DndListHandle.module.css';
import {listen} from '@tauri-apps/api/event';
import {info} from '@tauri-apps/plugin-log';

const Entities = () => {
    const [selectedEntity, setSelectedEntity] = useState('');
    const [selectedField, setSelectedField] = useState('');
    const scrollAreaRef = useRef(null);


    async function createNewEntity() {
        const dto = {
            name: 'New Entity',
            only_for_heritage: false,
            parent: null,
            fields: [],
            relationships: [],
        };
        await createEntity(dto)
            .then((res) => {
                return info(res.id.toString());
            })
            .catch((error) => console.error(error));
        info("Entity created");
    }

    listen('direct_access_entity_created', (event) => {
        const payload = event.payload as { ids: string[] };
        info(`Entity created event received: ${payload.ids}`);
        // Handle the event, e.g., update the UI
    });

    const handleNavigate = (direction: 'up' | 'down') => {
        const entities = ['Entity 1', 'Entity 2']; // Add more entities as needed
        const currentIndex = entities.indexOf(selectedEntity);
        if (direction === 'up' && currentIndex > 0) {
            setSelectedEntity(entities[currentIndex - 1]);
        } else if (direction === 'down' && currentIndex < entities.length - 1) {
            setSelectedEntity(entities[currentIndex + 1]);
        }
    };

    const data = [
        {id: 1, heritage: true, name: 'Carbon'},
        {id: 2, heritage: false, name: 'Nitrogen'},
        {id: 3, heritage: false, name: 'Nitrogen'},
        {id: 4, heritage: false, name: 'Oxygen'},
        {id: 5, heritage: false, name: 'Phosphorus'},
        {id: 6, heritage: false, name: 'Sulfur'},
        {id: 7, heritage: false, name: 'Fluorine'},
        {id: 8, heritage: false, name: 'Chlorine'},
        {id: 9, heritage: false, name: 'Bromine'},
        {id: 10, heritage: false, name: 'Iodine'},

    ];

    const [listState, handlers] = useListState(data);
    const items = listState.map((item, index) => (
        <Draggable key={item.id} index={index} draggableId={`entity-${item.id}`}>
            {(provided, snapshot) => (
                <Group
                    align="left"
                    className={cx(classes.item, {[classes.itemDragging]: snapshot.isDragging})}
                    ref={provided.innerRef}
                    onClick={
                        () => {
                            setSelectedEntity(item.name);
                        }
                    }
                    {...provided.draggableProps}
                >
                    <div {...provided.dragHandleProps} className={`${classes.dragHandle} cursor-grab`}>
                        <IconGripVertical size={18} stroke={1.5}/>
                    </div>
                    <div>
                        <Text>{item.name}</Text>
                        <Text c="dimmed" size="sm">
                            {item.heritage ? 'heritage' : ''}
                        </Text>
                    </div>
                </Group>
            )}
        </Draggable>
    ));

    const handleDragOver = (event: React.DragEvent<HTMLDivElement>) => {
        const scrollArea = scrollAreaRef.current;
        if (!scrollArea) return;

        const {clientY} = event;
        const {top, bottom, scrollTop, scrollHeight, clientHeight} = scrollArea;

        const padding = 50; // Adjust this value to control the scroll sensitivity

        if (clientY < top + padding) {
            scrollArea.scrollTo({top: scrollTop - 20, behavior: 'smooth'});
        } else if (clientY > bottom - padding) {
            scrollArea.scrollTo({top: scrollTop + 20, behavior: 'smooth'});
        }
    };

    return (
        <Box style={{display: 'flex', flexDirection: 'column', height: '80vh'}}>
            <Title order={1} id="entitiesTitle">Entities</Title>
            <Flex style={{
                height: '100%',
                margin: '0 20px',
            }}>
                <Stack miw={300}>
                    <Group>
                        <Title order={2} id="entitiesListHeading">Entities</Title>
                        <ActionIcon variant="filled" aria-label="Add new entity"
                                    onClick={createNewEntity}>+</ActionIcon>
                    </Group>

                    <DragDropContext
                        onDragEnd={({destination, source}) =>
                            handlers.reorder({from: source.index, to: destination?.index || 0})
                        }
                    >
                        <Droppable droppableId="dnd-list" direction="vertical" type="entity">
                            {(provided) => (
                                <ScrollArea type="auto"
                                            offsetScrollbars="present"
                                            style={{flexGrow: 1}}
                                            {...provided.droppableProps}
                                            ref={(node) => {
                                                scrollAreaRef.current = node;
                                                provided.innerRef(node);
                                            }}
                                            onDragOver={handleDragOver}>
                                    {items}
                                    {provided.placeholder}
                                </ScrollArea>
                            )}
                        </Droppable>
                    </DragDropContext>
                    {/*                    <ul aria-labelledby="entitiesListHeading">
                        <ListItem
                            title="Entity 1"
                            subtitle="heritage"
                            moveable
                            onSelect={() => setSelectedEntity('Entity 1')}
                            onRemove={() => console.log('remove entity 1')}
                            selected={selectedEntity === 'Entity 1'}
                            onNavigate={handleNavigate}
                        />
                        <ListItem
                            title="Entity 2"
                            subtitle=""
                            moveable
                            onSelect={() => setSelectedEntity('Entity 2')}
                            onRemove={() => console.log('remove entity 2')}
                            selected={selectedEntity === 'Entity 2'}
                            onNavigate={handleNavigate}
                        />
                    </ul>*/}
                </Stack>
                <Divider orientation="vertical" style={{
                    height: '100%',
                    margin: '0 20px',
                }}></Divider>

                <Stack flex={1}>
                    {selectedEntity && (
                        <>
                            <Title order={2}>Entity details</Title>
                            <form>
                                <label className="label" htmlFor="entityName">
                                    <span className="label-text">Name</span>
                                    <input id="entityName" type="text"/>
                                </label>
                                <label className="cursor-pointer" htmlFor="directAccess">
                                    <span>Direct access</span>
                                    <input id="directAccess" type="checkbox"/>
                                </label>
                            </form>
                            <div className="mt-4">
                                <Group id="fieldsListHeading">
                                    <Title order={3}>Fields</Title>
                                    <Tooltip label="Add new field">
                                        <ActionIcon aria-label="Add new field">+</ActionIcon>
                                    </Tooltip>
                                </Group>
                                <ul aria-labelledby="fieldsListHeading">
                                    {/* ...list fields... */}
                                    <li
                                        role="option"
                                        aria-selected={selectedField === 'Field 1'}
                                        className={`cursor-pointer ${selectedField === 'Field 1' ? 'bg-base-300' : ''}`}
                                        onClick={() => setSelectedField('Field 1')}
                                    >
                                        Field 1

                                    </li>
                                    {/* ...existing code... */}
                                </ul>
                            </div>
                        </>
                    )}
                </Stack>
                <Divider orientation="horizontal"></Divider>
                <div className="flex-1">
                    {selectedField && (
                        <>
                            <h2 className="text-xl font-semibold">Field details</h2>
                            {/* ...form or details related to selected field... */}
                        </>
                    )}
                </div>
            </Flex>
        </Box>
    );
}

export default Entities;
