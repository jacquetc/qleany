import {ReactNode, useEffect, useState} from 'react';
import {ActionIcon, Group, Title, Tooltip} from '@mantine/core';
import {
    FeatureRelationshipField,
    getFeature,
    setFeatureRelationship,
    updateFeature
} from "#controller/feature-controller.ts";
import {error, info} from '@tauri-apps/plugin-log';
import {createUseCase, getUseCaseMulti, UseCaseDto} from "#controller/use-case-controller.ts";
import ReorderableList from '../ReorderableList.tsx';
import {listen} from '@tauri-apps/api/event';

interface UseCaseListProps {
    selectedFeature: number | null;
    onSelectUseCase: (useCaseId: number | null) => void;
}

const UseCaseList = ({
                         selectedFeature, onSelectUseCase
                     }: UseCaseListProps) => {
    const [useCases, setUseCases] = useState<UseCaseDto[]>([]);
    const [selectedUseCase, setSelectedUseCase] = useState<number | null>(null);

    async function createNewUseCase() {
        if (!selectedFeature) {
            await error("No feature selected");
            return;
        }

        try {
            const feature = await getFeature(selectedFeature);
            // Find the selected feature
            if (!feature) {
                await error("Selected feature not found");
                return;
            }

            // Create use case
            const dto = {
                name: 'New Use Case',
                validator: false,
                entities: [],
                undoable: false,
                dto_in: null,
                dto_out: null,
            };

            const newUseCase = await createUseCase(dto);

            // Update feature with the new use case
            const updatedUseCases = [...feature.use_cases, newUseCase.id];
            const updatedFeature = {
                ...feature,
                use_cases: updatedUseCases
            };

            await updateFeature(updatedFeature);

            // Refresh data
            await fetchUseCaseData();

            // Select the newly created use case
            setSelectedUseCase(newUseCase.id);
            onSelectUseCase(newUseCase.id);

            await info("Use Case created successfully");
        } catch (err) {
            await error(`Failed to create use case: ${err}`);
        }
    }

    // Function to fetch use case data from the backend
    async function fetchUseCaseData() {
        if (!selectedFeature) return;

        const feature = await getFeature(selectedFeature);
        if (!feature) return;

        const useCaseIds = feature.use_cases;
        const useCases = await getUseCaseMulti(useCaseIds);
        setUseCases(useCases.filter((useCase) => useCase !== null) as UseCaseDto[]);
    }

    useEffect(() => {

        // mounting the event listeners
        const unlisten_direct_access_use_case_updated = listen('direct_access_use_case_updated', async (event) => {
            const payload = event.payload as { ids: number[] };
            info(`Use case updated event received: ${payload.ids}`);
            const updatedUseCases = await getUseCaseMulti(payload.ids);

            for (const updatedUseCase of updatedUseCases) {
                if (!updatedUseCase) {
                    info(`Use case not found in the current state.`);
                    continue;
                }

                const index = useCases.findIndex((useCase) => useCase.id === updatedUseCase.id);
                if (index !== -1) {
                    const updatedUseCasesList = [...useCases];
                    updatedUseCasesList[index] = updatedUseCase;
                    setUseCases(updatedUseCasesList);
                } else {
                    info(`Use case not found in the current state.`);
                }

            }

            //fetchUseCaseData().catch((err => error(err)));
        });

        const unlisten_direct_access_all_reset = listen('direct_access_all_reset', () => {
            info(`Direct access all reset event received in UseCaseList`);
            fetchUseCaseData().catch((err => error(err)));
        });

        return () => {
            unlisten_direct_access_use_case_updated.then(f => f());
            unlisten_direct_access_all_reset.then(f => f());
        };

    }, [useCases]);

    // change use cases when selected feature changes
    useEffect(() => {
        console.info("Selected feature changed:", selectedFeature);
        if (selectedFeature) {
            // Fetch use case data using the use case IDs from the selected feature
            const fetchUseCases = async () => {
                try {
                    const featureData = await getFeature(selectedFeature);
                    if (!featureData) {
                        setUseCases([]);
                        return;
                    }
                    const useCaseData = await getUseCaseMulti(featureData.use_cases);
                    const validUseCases = useCaseData.filter((useCase): useCase is UseCaseDto => useCase !== null);
                    setUseCases(validUseCases);

                    setSelectedUseCase(null);
                    onSelectUseCase(null);

                } catch (err) {
                    error(`Failed to fetch use cases: ${err}`);
                }
            };

            fetchUseCases().catch((err => error(err)));
        } else {
            setUseCases([]);
        }


    }, [selectedFeature]);

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

    // Define onReorder function to handle reordering
    const handleReorder = async (reorderedIds: number[]): Promise<void> => {
        if (!selectedFeature) return;

        try {
            // Update the feature with the new use case order
            await setFeatureRelationship({
                id: selectedFeature,
                field: FeatureRelationshipField.UseCases,
                right_ids: reorderedIds,
            });

            info("Use Case order updated successfully");
            await fetchUseCaseData();
        } catch (err) {
            error(`Failed to update use case order: ${err}`);
        }
    };

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
