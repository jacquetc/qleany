import React, {FC, useEffect, useRef, useState} from "react";
import invariant from "tiny-invariant";
import {dropTargetForElements} from "@atlaskit/pragmatic-drag-and-drop/element/adapter";
import ListItemPDND from "./ListItemPDND.tsx";

interface ListColumnProps {
    columnId: string;
    title: string;
    cards: Array<
        {
            id: number;
            dndType: string;
            content: React.ReactNode;
        }
    >;
}

const ListColumn: FC<ListColumnProps> = ({columnId, title, cards}) => {
    const columnRef = useRef(null); // Create a ref for the column
    const [isDraggedOver, setIsDraggedOver] = useState(false);

    useEffect(() => {
        const columnEl = columnRef.current;
        invariant(columnEl); // Ensure the column element exists

        // Set up the drop target for the column element
        return dropTargetForElements({
            element: columnEl,
            onDragStart: () => setIsDraggedOver(true),
            onDragEnter: () => setIsDraggedOver(true),
            onDragLeave: () => setIsDraggedOver(false),
            onDrop: () => setIsDraggedOver(false),
            getData: () => ({columnId}),
            getIsSticky: () => true,
        });
    }, [columnId]);
    return (
        <div
            className={`${isDraggedOver ? "dragged-over" : ""}`}
            ref={columnRef} // attach a columnRef to the column div
        >
            <h2>{title}</h2>
            {cards.map((card) => (
                <ListItemPDND key={card.id} {...card}>
                    {card.content}
                </ListItemPDND>
            ))}
        </div>
    );
};

export default ListColumn;