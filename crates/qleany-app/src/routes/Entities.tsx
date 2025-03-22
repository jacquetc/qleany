import { useState } from 'react';
// import ListItem from '../components/ListItem';
import { DragDropContext, Draggable, Droppable } from '@hello-pangea/dnd';
import { IconGripVertical } from '@tabler/icons-react';
import cx from 'clsx';
import { Text } from '@mantine/core';
import { useListState } from '@mantine/hooks';
import classes from './DndListHandle.module.css';

const Entities = () => {
    const [selectedEntity, setSelectedEntity] = useState('');
    const [selectedField, setSelectedField] = useState('');

    const handleNavigate = (direction: 'up' | 'down') => {
        const entities = ['Entity 1', 'Entity 2']; // Add more entities as needed
        const currentIndex = entities.indexOf(selectedEntity);
        if (direction === 'up' && currentIndex > 0) {
            setSelectedEntity(entities[currentIndex - 1]);
        } else if (direction === 'down' && currentIndex < entities.length - 1) {
            setSelectedEntity(entities[currentIndex + 1]);
        }
    };
{/*     
     const data = [
         { position: 6, mass: 12.011, symbol: 'C', name: 'Carbon' },
         { position: 7, mass: 14.007, symbol: 'N', name: 'Nitrogen' },
         { position: 39, mass: 88.906, symbol: 'Y', name: 'Yttrium' },
         { position: 56, mass: 137.33, symbol: 'Ba', name: 'Barium' },
         { position: 58, mass: 140.12, symbol: 'Ce', name: 'Cerium' },
     ];
     const [listState, handlers] = useListState(data);    const items = listState.map((item, index) => (
        <Draggable key={item.symbol} index={index} draggableId={item.symbol}>
            {(provided, snapshot) => (
                <div
                    className={cx(classes.item, { [classes.itemDragging]: snapshot.isDragging })}
                    ref={provided.innerRef}
                    {...provided.draggableProps}
                >
                    <div {...provided.dragHandleProps} className={classes.dragHandle}>
                        <IconGripVertical size={18} stroke={1.5} />
                    </div>
                    <Text className={classes.symbol}>{item.symbol}</Text>
                    <div>
                        <Text>{item.name}</Text>
                        <Text c="dimmed" size="sm">
                            Position: {item.position} • Mass: {item.mass}
                        </Text>
                    </div>
                </div>
            )}
        </Draggable>
    ));
    */}

    return (
        <div className="flex-1 p-4">
            <h1 className="text-4xl font-bold" id="entitiesTitle">Entities</h1>
            <div className="flex flex-row mt-4 space-x-4">
                <div className="flex-1">
                    <h2 id="entitiesListHeading" className="flex items-center space-x-2 text-xl font-semibold">
                        <span>Entities</span>
                        <button className="btn btn-primary btn-sm" aria-label="Add new entity">+</button>
                    </h2>

                    {/* <DragDropContext
                        onDragEnd={({ destination, source }) =>
                            handlers.reorder({ from: source.index, to: destination?.index || 0 })
                        }>
                        <Droppable droppableId="dnd-list" direction="vertical">
                            {(provided) => (
                                <div {...provided.droppableProps} ref={provided.innerRef}>
                                    {items}
                                    {provided.placeholder}
                                </div>
                            )}
                        </Droppable> */}
                        {/* <ul aria-labelledby="entitiesListHeading">
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
                    </ul> */}
                    {/* </DragDropContext> */}
                </div>
                <div className="divider divider-horizontal"></div>
                <div className="flex-1">
                    {selectedEntity && (
                        <>
                            <h2 className="text-xl font-semibold">Entity details</h2>
                            <form className="form-control space-y-2">
                                <label className="label" htmlFor="entityName">
                                    <span className="label-text">Name</span>
                                    <input id="entityName" type="text" className="input input-bordered" />
                                </label>
                                <label className="label cursor-pointer" htmlFor="directAccess">
                                    <span className="label-text">Direct access</span>
                                    <input id="directAccess" type="checkbox" className="checkbox" />
                                </label>
                            </form>
                            <div className="mt-4">
                                <h3 id="fieldsListHeading" className="flex items-center space-x-2 text-xl font-semibold">
                                    <span>Fields</span>
                                    <button className="btn btn-sm btn-accent" aria-label="Add new field">+</button>
                                </h3>
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
                </div>
                <div className="divider divider-horizontal"></div>
                <div className="flex-1">
                    {selectedField && (
                        <>
                            <h2 className="text-xl font-semibold">Field details</h2>
                            {/* ...form or details related to selected field... */}
                        </>
                    )}
                </div>
            </div>
        </div>
    );
}

export default Entities;
