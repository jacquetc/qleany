import "#components/DndListHandle.module.css"
import {useState} from 'react';
import {Divider, Flex, Stack} from '@mantine/core';
import FeatureList from '#components/features/FeatureList.tsx';
import FeatureDetails from '#components/features/FeatureDetails.tsx';
import UseCaseList from '#components/features/UseCaseList.tsx';
import UseCaseDetails from '#components/features/UseCaseDetails.tsx';

const Features = () => {
    const [selectedFeature, setSelectedFeature] = useState<number | null>(0);
    const [selectedUseCase, setSelectedUseCase] = useState<number | null>(0);

    return (
        <Flex direction="row" style={{
            height: '100%',
            margin: '0 0px',
        }}>
            <Stack miw={300} style={{overflow: 'auto', height: '100%'}}>
                <FeatureList
                    onSelectFeature={setSelectedFeature}
                />
            </Stack>
            <Divider orientation="vertical" mb={0} mt={0} ml={5} mr={5}></Divider>

            <Stack flex={1} style={{overflow: 'auto', height: '100%'}}>
                <FeatureDetails
                    selectedFeature={selectedFeature}
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
