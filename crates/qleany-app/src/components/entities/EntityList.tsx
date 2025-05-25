import {ReactNode, useState} from 'react';
import {EntityDto} from "#controller/entity_controller.ts";
import {ActionIcon, Group, Title} from '@mantine/core';
import ReorderableList from '../ReorderableList.tsx';
import {useEntityListModel} from './RootEntitiesListModel.ts';

interface EntityListProps {
    onSelectEntity: (entityId: number) => void;
}

const EntityList = ({
                        onSelectEntity,
                    }: EntityListProps) => {
    const [selectedEntity, setSelectedEntity] = useState<number | null>(null);

    // Use the entity list model
    const {entities, createNewEntity, handleReorder} = useEntityListModel({
        onEntitiesChanged: (__newEntities) => {
            // If needed, you can perform additional actions when entities change
        }
    });

    // Create header component for ReorderableList
    const header = (
        <Group>
            <Title order={2} id="entitiesListHeading">Entities</Title>
            <ActionIcon
                variant="filled"
                aria-label="Add new entity"
                onClick={createNewEntity}
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

    // Handle entity selection
    const handleSelectEntity = (entityId: number) => {
        setSelectedEntity(entityId);
        onSelectEntity(entityId);
    };

    return (
        <ReorderableList
            items={entities}
            selectedItemId={selectedEntity}
            onSelectItem={handleSelectEntity}
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
