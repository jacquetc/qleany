import { ReactNode } from 'react';
import { ActionIcon, Alert, Group, Title, Tooltip } from '@mantine/core';
import { UseCaseDTO } from '@/services/use-case-service';
import ReorderableList from '../ReorderableList.tsx';
import { useFeatureContext } from '@/contexts/FeatureContext';
import ErrorBoundary from '@/components/ErrorBoundary';

const UseCaseList = () => {
    const {
        selectedFeatureId,
        selectedUseCaseId,
        useCases,
        isLoadingUseCases,
        useCaseError,
        selectUseCase,
        createUseCase,
        reorderUseCases
    } = useFeatureContext();

    // Custom fallback component for error state
    const errorFallback = (
        <Alert color="yellow" title="Use cases could not be loaded">
            There was an issue loading the use case list. Please try again later.
        </Alert>
    );

    // Loading state
    if (isLoadingUseCases) {
        return (
            <Alert color="blue" title="Loading use cases">
                Please wait...
            </Alert>
        );
    }

    // Error state
    if (useCaseError) {
        return (
            <Alert color="red" title="Error loading use cases">
                {useCaseError instanceof Error ? useCaseError.message : 'An unknown error occurred'}
            </Alert>
        );
    }

    // No feature selected state
    if (!selectedFeatureId) {
        return (
            <Alert color="gray" title="No feature selected">
                Please select a feature to view its use cases.
            </Alert>
        );
    }

    // Create header component for ReorderableList
    const header = (
        <Group id="useCasesListHeading">
            <Title order={3}>Use Cases</Title>
            <Tooltip label="Add new use case">
                <ActionIcon
                    variant="filled"
                    aria-label="Add new use case"
                    onClick={createUseCase}
                >
                    +
                </ActionIcon>
            </Tooltip>
        </Group>
    );

    // Define renderItemContent function for use case items
    const renderUseCaseContent = (useCase: UseCaseDTO): ReactNode => (
        <div>
            <div>{useCase.name}</div>
            <div style={{color: 'dimmed', fontSize: 'small'}}>
                {useCase.validator ? 'Validator' : ''}
                {useCase.undoable ? ' (Undoable)' : ''}
            </div>
        </div>
    );

    return (
        <ErrorBoundary fallback={errorFallback}>
            <ReorderableList
                items={useCases}
                selectedItemId={selectedUseCaseId}
                onSelectItem={selectUseCase}
                onReorder={reorderUseCases}
                getItemId={(useCase) => useCase.id}
                renderItemContent={renderUseCaseContent}
                droppableId="use-cases-list"
                draggableIdPrefix="use-case"
                itemType="use-case"
                header={header}
            />
        </ErrorBoundary>
    );
};

export default UseCaseList;
