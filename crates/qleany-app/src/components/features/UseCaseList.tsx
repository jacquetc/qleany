import {ReactNode, useState} from 'react';
import {ActionIcon, Group, Title, Tooltip} from '@mantine/core';
import {UseCaseDto} from "#controller/use-case-controller.ts";
import ReorderableList from '../ReorderableList.tsx';
import {useFeatureUseCasesListModel} from "#models/FeatureUseCasesListModel.ts";

interface UseCaseListProps {
    selectedFeature: number | null;
    onSelectUseCase: (useCaseId: number | null) => void;
}

const UseCaseList = ({
                         selectedFeature, onSelectUseCase
                     }: UseCaseListProps) => {
    const [selectedUseCase, setSelectedUseCase] = useState<number | null>(null);

    // Use the use case list model
    const {useCases, createNewUseCase, handleReorder} = useFeatureUseCasesListModel({
        featureId: selectedFeature,
        onUseCasesChanged: (__newUseCases) => {

            // If needed, you can perform additional actions when entities change
        }
    });

    if (!selectedFeature) {
        return null;
    }

    // Create header component for ReorderableList
    const header = (
        <Group id="useCasesListHeading">
            <Title order={3}>Use Cases</Title>
            <Tooltip label="Add new use case">
                <ActionIcon
                    variant="filled"
                    aria-label="Add new use case"
                    onClick={createNewUseCase}
                >
                    +
                </ActionIcon>
            </Tooltip>
        </Group>
    );

    // Define renderItemContent function for use case items
    const renderUseCaseContent = (useCase: UseCaseDto): ReactNode => (
        <div>
            <div>{useCase.name}</div>
            <div style={{color: 'dimmed', fontSize: 'small'}}>
                {useCase.validator ? 'Validator' : ''}
                {useCase.undoable ? ' (Undoable)' : ''}
            </div>
        </div>
    );

    // Define onSelectItem function
    const handleSelectItem = (useCaseId: number): void => {
        setSelectedUseCase(useCaseId);
        onSelectUseCase(useCaseId);
    };

    return (
        <ReorderableList
            items={useCases}
            selectedItemId={selectedUseCase}
            onSelectItem={handleSelectItem}
            onReorder={handleReorder}
            getItemId={(useCase) => useCase.id}
            renderItemContent={renderUseCaseContent}
            droppableId="use-cases-list"
            draggableIdPrefix="use-case"
            itemType="use-case"
            header={header}
        />
    );
};

export default UseCaseList;
