import {ReactNode} from 'react';
import {EntityDto} from "../controller/entity_controller";
import {ActionIcon, Group, Title} from '@mantine/core';
import {error, info} from '@tauri-apps/plugin-log';
import {RootRelationshipField, setRootRelationship} from "../controller/root_controller.ts";
import ReorderableList from './ReorderableList';

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

    // Create header component for ReorderableList
    const header = (
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
    );

    // Define renderItemContent function for entity items
    const renderEntityContent = (entity: EntityDto): ReactNode => (
        <div>
            <div>{entity.name}</div>
            <div style={{color: 'dimmed', fontSize: 'small'}}>
                {entity.only_for_heritage ? 'heritage' : ''}
            </div>
        </div>
    );

    // Define onReorder function to handle reordering
    const handleReorder = async (reorderedIds: number[]): Promise<void> => {
        try {
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
    };

    return (
        <ReorderableList
            items={entities}
            selectedItemId={selectedEntity}
            onSelectItem={onSelectEntity}
            onReorder={handleReorder}
            getItemId={(entity) => entity.id}
            renderItemContent={renderEntityContent}
            droppableId="dnd-list"
            draggableIdPrefix="entity"
            itemType="entity"
            header={header}
        />
    );
};

export default EntityList;
