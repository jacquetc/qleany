import { ReactNode, useEffect, useState } from 'react';
import { Checkbox, Group } from '@mantine/core';
import cx from 'clsx';
import classes from './DndListHandle.module.css';

interface CheckableListProps<T> {
    items: T[];
    selectedItemId: number | null;
    checkedItemIds: number[];
    onSelectItem: (itemId: number) => void;
    onCheckItem: (itemId: number, checked: boolean) => void;
    getItemId: (item: T) => number;
    renderItemContent: (item: T) => ReactNode;
    itemType: string;
    header?: ReactNode;
}

function CheckableList<T>({
    items,
    selectedItemId,
    checkedItemIds,
    onSelectItem,
    onCheckItem,
    getItemId,
    renderItemContent,
    itemType,
    header
}: CheckableListProps<T>) {
    const [listItems, setListItems] = useState<T[]>(items);

    useEffect(() => {
        setListItems(items);
    }, [items]);

    const renderedItems = listItems.map((item) => {
        const itemId = getItemId(item);
        const isChecked = checkedItemIds.includes(itemId);
        
        return (
            <Group
                key={itemId}
                align="left"
                className={cx(classes.item, {
                    [classes.itemSelected]: itemId === selectedItemId
                })}
                onClick={() => onSelectItem(itemId)}
            >
                <Checkbox
                    checked={isChecked}
                    onChange={(event) => {
                        event.stopPropagation();
                        onCheckItem(itemId, event.currentTarget.checked);
                    }}
                    onClick={(event) => event.stopPropagation()}
                    className="ml-2"
                />
                {renderItemContent(item)}
            </Group>
        );
    });

    return (
        <>
            {header}
            <div
                style={{
                    height: '100%',
                    overflow: 'auto',
                    flexGrow: 1
                }}
            >
                {renderedItems}
            </div>
        </>
    );
}

export default CheckableList;