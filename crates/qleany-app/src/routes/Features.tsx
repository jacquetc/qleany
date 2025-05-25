import "#components/DndListHandle.module.css"
import {useEffect, useState} from 'react';
import {createFeature, FeatureDto, getFeatureMulti} from "#controller/feature_controller.ts";
import {Divider, Flex, Stack} from '@mantine/core';
import {listen} from '@tauri-apps/api/event';
import {error, info} from '@tauri-apps/plugin-log';
import {
    getRootMulti,
    getRootRelationship,
    RootRelationshipField,
    setRootRelationship
} from "#controller/root_controller.ts";
import FeatureList from '#components/features/FeatureList.tsx';
import FeatureDetails from '#components/features/FeatureDetails.tsx';
import UseCaseList from '#components/features/UseCaseList.tsx';
import UseCaseDetails from '#components/features/UseCaseDetails.tsx';

const Features = () => {
    const [selectedFeature, setSelectedFeature] = useState<number | null>(0);
    const [featureData, setFeatureData] = useState<FeatureDto[]>([]);
    const [selectedUseCase, setSelectedUseCase] = useState<number | null>(0);
    const [rootId, setRootId] = useState<number>(1);

    // Function to get the root ID
    async function getRootId() {
        const roots = await getRootMulti([]);
        if (roots.length > 0 && roots[0] !== null) {
            setRootId(roots[0]!.id);
            return roots[0]!.id;
        }
        return 1; // Fallback to default
    }

    async function createNewFeature() {
        try {
            // Create feature
            const dto = {
                name: 'New Feature',
                use_cases: [],
            };

            const newFeature = await createFeature(dto);

            // Update root relationship
            const currentRootId = await getRootId();
            const rootFeatures = await getRootRelationship(currentRootId, RootRelationshipField.Features) || [];

            await setRootRelationship({
                id: currentRootId,
                field: RootRelationshipField.Features,
                right_ids: [...rootFeatures, newFeature.id],
            });

            // Update UI state
            setFeatureData(prevData => [...prevData, newFeature]);

            // Optional: Select the newly created feature
            setSelectedFeature(newFeature.id);

            await info("Feature created successfully");
        } catch (err) {
            await error(`Failed to create feature: ${err}`);
        }
    }

    // Function to fetch feature data from the backend
    async function fetchFeatureData() {
        const currentRootId = await getRootId();
        const featureIds = await getRootRelationship(currentRootId, RootRelationshipField.Features);
        const features = await getFeatureMulti(featureIds);
        setFeatureData(features.filter((feature) => feature !== null) as FeatureDto[]);
    }

    useEffect(() => {
        fetchFeatureData().catch((err => error(err)));

        // mounting the event listeners
        const unlisten_feature_created = listen('feature_created', (event) => {
            const payload = event.payload as { ids: string[] };
            info(`Feature created event received: ${payload.ids}`);

            fetchFeatureData().catch((err => error(err)));
        });

        const unlisten_direct_access_all_reset = listen('direct_access_all_reset', () => {
            info(`Direct access all reset event received`);
            fetchFeatureData().catch((err => error(err)));
        });

        return () => {
            unlisten_feature_created.then(f => f());
            unlisten_direct_access_all_reset.then(f => f());
        };

    }, []);

    return (
        <Flex direction="row" style={{
            height: '100%',
            margin: '0 0px',
        }}>
            <Stack miw={300} style={{overflow: 'auto', height: '100%'}}>
                <FeatureList
                    features={featureData}
                    selectedFeature={selectedFeature}
                    onSelectFeature={setSelectedFeature}
                    onCreateFeature={createNewFeature}
                    onFeaturesReordered={fetchFeatureData}
                />
            </Stack>
            <Divider orientation="vertical" mb={0} mt={0} ml={5} mr={5}></Divider>

            <Stack flex={1} style={{overflow: 'auto', height: '100%'}}>
                <FeatureDetails
                    selectedFeature={selectedFeature}
                    features={featureData}
                    onFeatureUpdated={fetchFeatureData}
                />
                <UseCaseList
                    selectedFeature={selectedFeature}
                    onSelectUseCase={setSelectedUseCase}
                />
            </Stack>
            {selectedUseCase !== null && selectedUseCase > 0 && (
                <>
                    <Divider orientation="vertical" style={{
                        height: '100%',
                        margin: '0 5px',
                    }}></Divider>

                    <Stack className="flex-1" miw={400} style={{overflow: 'auto', height: '100%'}}>
                        <UseCaseDetails selectedUseCase={selectedUseCase}/>
                    </Stack>
                </>
            )}
        </Flex>
    );
}

export default Features;
