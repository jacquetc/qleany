import {ReactNode, useEffect} from 'react';
import {DragDropContext, Draggable, Droppable} from '@hello-pangea/dnd';
import {IconGripVertical} from '@tabler/icons-react';
import cx from 'clsx';
import {Group} from '@mantine/core';
import {useListState} from '@mantine/hooks';
import classes from './DndListHandle.module.css';

interface ReorderableListProps<T> {
    items: T[];
    selectedItemId: number | null;
    onSelectItem: (itemId: number) => void;
    onReorder: (reorderedIds: number[]) => Promise<void>;
    getItemId: (item: T) => number;
    renderItemContent: (item: T) => ReactNode;
    droppableId: string;
    draggableIdPrefix: string;
    itemType: string;
    header?: ReactNode;
}

function ReorderableList<T>({
                                items,
                                selectedItemId,
                                onSelectItem,
                                onReorder,
                                getItemId,
                                renderItemContent,
                                droppableId,
                                draggableIdPrefix,
                                itemType,
                                header
                            }: ReorderableListProps<T>) {
    const [listState, handlers] = useListState<T>(items);

    useEffect(() => {
        handlers.setState(items);
    }, [items]);

    const listItems = listState.map((item, index) => {
        const itemId = getItemId(item);
        return (
            <Draggable key={itemId} index={index} draggableId={`${draggableIdPrefix}-${itemId}`}>
                {(provided, snapshot) => (
                    <Group
                        align="left"
                        className={cx(classes.item, {
                            [classes.itemDragging]: snapshot.isDragging,
                            [classes.itemSelected]: itemId === selectedItemId
                        })}
                        ref={provided.innerRef}
                        onClick={() => onSelectItem(itemId)}
                        {...provided.draggableProps}
                    >
                        <div {...provided.dragHandleProps} className={`${classes.dragHandle} cursor-grab`}>
                            <IconGripVertical size={18} stroke={1.5}/>
                        </div>
                        {renderItemContent(item)}
                    </Group>
                )}
            </Draggable>
        );
    });

    return (
        <>
            {header}

            <DragDropContext
                onDragEnd={async ({destination, source}) => {
                    if (!destination) return;

                    // Update local state
                    handlers.reorder({from: source.index, to: destination.index});

                    // Get the current order of items
                    const reorderedIds = listState.map(item => getItemId(item));

                    // Reorder the IDs based on the drag operation
                    const [removed] = reorderedIds.splice(source.index, 1);
                    reorderedIds.splice(destination.index, 0, removed);

                    // Call the provided reorder callback
                    await onReorder(reorderedIds);
                }}
            >
                <Droppable droppableId={droppableId} direction="vertical" type={itemType}>
                    {(provided) => (
                        <div
                            style={{
                                height: '100%',
                                overflow: 'auto',
                                flexGrow: 1
                            }}
                            {...provided.droppableProps}
                            ref={provided.innerRef}
                        >
                            {listItems}
                            {provided.placeholder}
                        </div>
                    )}
                </Droppable>
            </DragDropContext>
        </>
    );
}

export default ReorderableList;