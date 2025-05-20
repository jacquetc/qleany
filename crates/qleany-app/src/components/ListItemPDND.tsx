import React, {FC, useEffect, useRef, useState} from "react";
import invariant from "tiny-invariant";
import {draggable, dropTargetForElements} from "@atlaskit/pragmatic-drag-and-drop/element/adapter";
import {combine} from "@atlaskit/pragmatic-drag-and-drop/combine";
import {attachClosestEdge, extractClosestEdge} from "@atlaskit/pragmatic-drag-and-drop-hitbox/closest-edge";
import DropIndicator from "./DropIndicator";

interface ListItemProps {
    id: number;
    dndType: string;
    children: React.ReactNode;
}

const ListItemPDND: FC<ListItemProps> = ({children, ...card}) => {
    const cardRef = useRef(null);
    const [isDragging, setIsDragging] = useState(false);
    const [closestEdge, setClosestEdge] = useState(null);

    useEffect(() => {
        const cardEl = cardRef.current;
        invariant(cardEl);

        // Combine draggable and dropTargetForElements cleanup functions
        // to return a single cleanup function
        return combine(
            draggable({
                element: cardEl,
                getInitialData: () => ({type: card.dndType, cardId: card.id}),
                onDragStart: () => setIsDragging(true),
                onDrop: () => setIsDragging(false),
            }),
            // Add dropTargetForElements to make the card a drop target
            dropTargetForElements({
                element: cardEl,
                getData: ({input, element}) => {
                    // To attach card data to a drop target
                    const data = {type: card.dndType, cardId: card.id};

                    // Attaches the closest edge (top or bottom) to the data object
                    // This data will be used to determine where to drop card relative
                    // to the target card.
                    return attachClosestEdge(data, {
                        input,
                        element,
                        allowedEdges: ["top", "bottom"],
                    });
                },
                getIsSticky: () => true, // To make a drop target "sticky"
                onDragEnter: (args) => {
                    if (args.source.data.cardId !== card.id) {
                        console.log("onDragEnter", args);
                    }

                    // Update the closest edge when a draggable item enters the drop zone
                    if (args.source.data.cardId !== card.id) {
                        setClosestEdge(extractClosestEdge(args.self.data));
                    }
                },
                onDrag: (args) => {
                    // Continuously update the closest edge while dragging over the drop zone
                    if (args.source.data.cardId !== card.id) {
                        setClosestEdge(extractClosestEdge(args.self.data));
                    }
                },
                onDragLeave: () => {
                    // Reset the closest edge when the draggable item leaves the drop zone
                    setClosestEdge(null);
                },
                onDrop: () => {
                    // Reset the closest edge when the draggable item is dropped
                    setClosestEdge(null);
                },
            })
        );
        // Update the dependency array
    }, [card.id]);
    return (
        <div className={`card ${isDragging ? "dragging" : "cursor-grab"}`} ref={cardRef}>
            {children}
            {/* render the DropIndicator if there's a closest edge */}
            {closestEdge && <DropIndicator edge={closestEdge} gap="8px"/>}
        </div>
    );
};

export default ListItemPDND;
