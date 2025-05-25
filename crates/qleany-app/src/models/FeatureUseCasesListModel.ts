import {useEffect, useState} from 'react';
import {listen} from "@tauri-apps/api/event";
import {error, info} from '@tauri-apps/plugin-log';
import {createUseCase, getUseCaseMulti, UseCaseDto} from "@/controller/use-case-controller.ts";
import {
    FeatureRelationshipField,
    getFeatureRelationship,
    setFeatureRelationship
} from "@/controller/feature-controller.ts";
import {beginComposite, endComposite} from "#controller/undo-redo-controller.ts";
import {useDebouncedCallback} from "@mantine/hooks";

export interface FeatureUseCasesListModelProps {
    featureId: number | null;
    onUseCasesChanged: (useCases: UseCaseDto[]) => void;
}

export function useFeatureUseCasesListModel(
    {
        featureId,
        onUseCasesChanged
    }
    : FeatureUseCasesListModelProps) {
    const [useCases, setUseCases] = useState<UseCaseDto[]>([]);

    // Function to fetch useCase data from the backend
    async function fetchUseCaseData() {
        if (!featureId) {
            await error("No feature selected");
            return [];
        }
        const useCaseIds = await getFeatureRelationship(featureId, FeatureRelationshipField.UseCases);
        const useCases = await getUseCaseMulti(useCaseIds);
        const filteredUseCases = useCases.filter((useCase) => useCase !== null) as UseCaseDto[];

        setUseCases(filteredUseCases);
        onUseCasesChanged(filteredUseCases);

        return filteredUseCases;
    }

    async function createNewUseCase() {
        try {
            // Create useCase
            const dto = {
                name: 'New UseCase',
                validator: false,
                undoable: false,
                dto_in: null,
                dto_out: null,
                entities: [],
            };
            await beginComposite()
            const newUseCase = await createUseCase(dto);

            if (!featureId) {
                return;
            }

            // Update useCase relationship
            const featureUseCases = await getFeatureRelationship(featureId, FeatureRelationshipField.UseCases) || [];

            await setFeatureRelationship({
                id: featureId,
                field: FeatureRelationshipField.UseCases,
                right_ids: [...featureUseCases, newUseCase.id],
            });
            await endComposite();

            // Update useCases state
            const updatedUseCases = [...useCases, newUseCase];
            setUseCases(updatedUseCases);
            onUseCasesChanged(updatedUseCases);

            await info("UseCase created successfully");
            return newUseCase;
        } catch (err) {
            await error(`Failed to create useCase: ${err}`);
            throw err;
        }
    }

    // Function to handle reordering of useCases
    async function handleReorder(reorderedIds: number[]): Promise<void> {
        try {
            if (!featureId) {
                await error("No feature selected for reordering useCases");
                return;
            }

            // Update the feature relationship with the new order
            await setFeatureRelationship({
                id: featureId,
                field: FeatureRelationshipField.UseCases,
                right_ids: reorderedIds,
            });

            info("UseCase order updated successfully");
            await fetchUseCaseData();
        } catch (err) {
            error(`Failed to update useCase order: ${err}`);
            throw err;
        }
    }

    const featureUpdaterHandler = useDebouncedCallback(async (event) => {
            const payload = event.payload as { ids: number[] };

            if (!featureId) {
                return;
            }

            if (!payload.ids.includes(featureId)) {
                return; // Ignore updates for other entities
            }

            info(`Feature updated event received: ${payload.ids}`);
            const updatedEntities = await getFeatureRelationship(featureId, FeatureRelationshipField.UseCases);

            // If the useCases relationship has changed, fetch the updated useCases
            const useCasesIds = useCases.map(useCase => useCase.id);

            if (JSON.stringify(updatedEntities) !== JSON.stringify(useCasesIds)) {
                info(`UseCases relationship has changed for feature ${featureId}, fetching updated useCases`);
                await fetchUseCaseData().catch((err) => error(err));
            } else {
                info(`UseCases relationship has not changed for feature ${featureId}`);
            }
        }
        , 1000);

    const useCaseUpdaterHandler = useDebouncedCallback(async (event) => {
            const payload = event.payload as { ids: number[] };
            info(`UseCase updated event received: ${payload.ids}`);
            const updatedUseCases = await getUseCaseMulti(payload.ids);

            for (const updatedUseCase of updatedUseCases) {

                if (!updatedUseCase) {
                    info(`UseCase not found in the current state.`);
                    continue;
                }
                const index = useCases.findIndex((useCase) => useCase.id === updatedUseCase.id);
                if (index !== -1) {
                    const updatedUseCasesList = [...useCases];
                    updatedUseCasesList[index] = updatedUseCase;
                    setUseCases(updatedUseCasesList);
                    onUseCasesChanged(updatedUseCasesList);
                } else {
                    info(`UseCase not found in the current state.`);
                }
            }
        }
        , 1000);

    // Setup event listeners
    useEffect(() => {
        fetchUseCaseData().catch((err) => error(err));

        // mounting the event listeners
        const unlisten_direct_access_useCase_created = listen('direct_access_useCase_created', (event) => {
            const payload = event.payload as { ids: string[] };
            info(`UseCase created event received: ${payload.ids}`);

            fetchUseCaseData().catch((err) => error(err));
        });

        // Listen for useCase removal events
        const unlisten_direct_access_useCase_removed = listen('direct_access_useCase_removed', async (event) => {
            const payload = event.payload as { ids: number[] };
            info(`UseCase removed event received: ${payload.ids}`);

            // Filter out the removed useCases from the current state
            const updatedUseCases = useCases.filter(useCase => !payload.ids.includes(useCase.id));
            setUseCases(updatedUseCases);
            onUseCasesChanged(updatedUseCases);
        });

        // Listen for useCase updates


        const unlisten_direct_access_useCase_updated = listen('direct_access_useCase_updated', useCaseUpdaterHandler);

        // listen to any feature update event, filter to the current feature, check if the "useCases" relationship has changed
        // and update the useCases state accordingly


        const unlisten_direct_access_feature_updated = listen('direct_access_feature_updated', featureUpdaterHandler);

        const unlisten_direct_access_all_reset = listen('direct_access_all_reset', () => {
            info(`Direct access all reset event received`);
            fetchUseCaseData().then((dtos) => info(`UseCases data reset successfully: ${JSON.stringify(dtos)}`)
            ).catch((err) => error(err));
        });

        return () => {
            unlisten_direct_access_useCase_created.then(f => f());
            unlisten_direct_access_useCase_removed.then(f => f());
            unlisten_direct_access_useCase_updated.then(f => f());
            unlisten_direct_access_feature_updated.then(f => f());
            unlisten_direct_access_all_reset.then(f => f());
        };
    }, [useCases, featureId]);

    return {
        useCases,
        createNewUseCase,
        handleReorder,
        fetchUseCaseData
    };
}