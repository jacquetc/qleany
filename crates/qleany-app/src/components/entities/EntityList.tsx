import { ReactNode } from 'react';
import { ActionIcon, Alert, Group, Title } from '@mantine/core';
import ReorderableList from '../ReorderableList.tsx';
import { useEntityContext } from '@/contexts/EntityContext';
import { EntityDTO } from '@/services/entity-service';
import ErrorBoundary from '@/components/ErrorBoundary';

const EntityList = () => {
    // Use the entity context instead of the model
    const {
        entities,
        selectedEntityId,
        isLoadingEntities,
        entityError,
        selectEntity,
        createEntity,
        reorderEntities
    } = useEntityContext();

    // Create header component for ReorderableList
    const header = (
        <Group>
            <Title order={2} id="entitiesListHeading">Entities</Title>
            <ActionIcon
                variant="filled"
                aria-label="Add new entity"
                onClick={createEntity}
            >
                +
            </ActionIcon>
        </Group>
    );

    // Define renderItemContent function for entity items
    const renderEntityContent = (entity: EntityDTO): ReactNode => (
        <div>
            <div>{entity.name}</div>
            <div style={{color: 'dimmed', fontSize: 'small'}}>
                {entity.fields.length > 0 ? `${entity.fields.length} fields` : 'No fields'}
            </div>
        </div>
    );

    // Custom fallback component for error state
    const errorFallback = (
        <Alert color="yellow" title="Entities could not be loaded">
            There was an issue loading the entity list. Please try again later.
        </Alert>
    );

    // Loading state
    if (isLoadingEntities) {
        return (
            <Alert color="blue" title="Loading entities">
                Please wait...
            </Alert>
        );
    }

    // Error state
    if (entityError) {
        return (
            <Alert color="red" title="Error loading entities">
                {entityError instanceof Error ? entityError.message : 'An unknown error occurred'}
            </Alert>
        );
    }

    return (
        <ErrorBoundary fallback={errorFallback}>
            <ReorderableList
                items={entities}
                selectedItemId={selectedEntityId}
                onSelectItem={selectEntity}
                onReorder={async (reorderedIds) => {
                    await reorderEntities(reorderedIds);
                }}
                getItemId={(entity) => entity.id}
                renderItemContent={renderEntityContent}
                droppableId="dnd-list"
                draggableIdPrefix="entity"
                itemType="entity"
                header={header}
            />
        </ErrorBoundary>
    );
};

export default EntityList;
