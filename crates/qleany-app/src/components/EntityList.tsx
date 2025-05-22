import {useEffect, useRef} from 'react';
import {EntityDto} from "../controller/entity_controller";
import {DragDropContext, Draggable, Droppable} from '@hello-pangea/dnd';
import {IconGripVertical} from '@tabler/icons-react';
import cx from 'clsx';
import {ActionIcon, Group, ScrollArea, Title} from '@mantine/core';
import {useListState} from '@mantine/hooks';
import classes from '../routes/DndListHandle.module.css';
import {error, info} from '@tauri-apps/plugin-log';
import {RootRelationshipField, setRootRelationship} from "../controller/root_controller.ts";

interface EntityListProps {
    entities: EntityDto[];
    selectedEntity: number | null;
    onSelectEntity: (entityId: number) => void;
    onCreateEntity: () => void;
    onEntitiesReordered: () => void;
}

const EntityList = ({
                        entities,
                        selectedEntity,
                        onSelectEntity,
                        onCreateEntity,
                        onEntitiesReordered
                    }: EntityListProps) => {
    const scrollAreaRef = useRef<HTMLDivElement | null>(null);
    const [listState, handlers] = useListState(entities);

    useEffect(() => {
        handlers.setState(entities);
    }, [entities]);

    const items = listState.map((item, index) => (
        <Draggable key={item.id} index={index} draggableId={`entity-${item.id}`}>
            {(provided, snapshot) => (
                <Group
                    align="left"
                    className={cx(classes.item, {
                        [classes.itemDragging]: snapshot.isDragging,
                        [classes.itemSelected]: item.id === selectedEntity
                    })}
                    ref={provided.innerRef}
                    onClick={() => onSelectEntity(item.id)}
                    {...provided.draggableProps}
                >
                    <div {...provided.dragHandleProps} className={`${classes.dragHandle} cursor-grab`}>
                        <IconGripVertical size={18} stroke={1.5}/>
                    </div>
                    <div>
                        <div>{item.name}</div>
                        <div style={{color: 'dimmed', fontSize: 'small'}}>
                            {item.only_for_heritage ? 'heritage' : ''}
                        </div>
                    </div>
                </Group>
            )}
        </Draggable>
    ));

    return (
        <>
            <Group>
                <Title order={2} id="entitiesListHeading">Entities</Title>
                <ActionIcon
                    variant="filled"
                    aria-label="Add new entity"
                    onClick={onCreateEntity}
                >
                    +
                </ActionIcon>
            </Group>

            <DragDropContext
                onDragEnd={async ({destination, source}) => {
                    if (!destination) return;

                    // Update local state
                    handlers.reorder({from: source.index, to: destination.index});

                    // Persist changes to backend
                    try {
                        // Get the current order of entities
                        const reorderedIds = listState.map(entity => entity.id);

                        // Reorder the IDs based on the drag operation
                        const [removed] = reorderedIds.splice(source.index, 1);
                        reorderedIds.splice(destination.index, 0, removed);

                        // Update the root relationship with the new order
                        await setRootRelationship({
                            id: 1,
                            field: RootRelationshipField.Entities,
                            right_ids: reorderedIds,
                        });

                        info("Entity order updated successfully");
                        onEntitiesReordered();
                    } catch (err) {
                        error(`Failed to update entity order: ${err}`);
                    }
                }}
            >
                <Droppable droppableId="dnd-list" direction="vertical" type="entity">
                    {(provided) => (
                        <ScrollArea
                            type="auto"
                            offsetScrollbars="present"
                            style={{flexGrow: 1}}
                            {...provided.droppableProps}
                            ref={(node) => {
                                scrollAreaRef.current = node;
                                provided.innerRef(node);
                            }}
                        >
                            {items}
                            {provided.placeholder}
                        </ScrollArea>
                    )}
                </Droppable>
            </DragDropContext>
        </>
    );
};

export default EntityList;