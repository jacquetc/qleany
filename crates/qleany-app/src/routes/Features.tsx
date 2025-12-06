import "#components/DndListHandle.module.css"
import React, { useEffect, useState } from 'react';
import { Alert, Button, Divider, Flex, Group, Stack, Title } from '@mantine/core';
import { error, info } from '@tauri-apps/plugin-log';
import { rootService } from '@/services/root-service';
import FeatureList from '#components/features/FeatureList.tsx';
import FeatureDetails from '#components/features/FeatureDetails.tsx';
import UseCaseList from '#components/features/UseCaseList.tsx';
import UseCaseDetails from '#components/features/UseCaseDetails.tsx';
import ErrorBoundary from '@/components/ErrorBoundary';
import { FeatureProvider, useFeatureContext } from '@/contexts/FeatureContext';

// Inner component that uses the FeatureContext
const FeaturesContent = () => {
    const {
        selectedFeatureId,
        selectedUseCaseId,
        selectFeature,
        selectUseCase
    } = useFeatureContext();

    return (
        <Flex direction="row" style={{
            height: '100%',
            margin: '0 0px',
        }}>
            <Stack miw={300} style={{overflow: 'auto', height: '100%'}}>
                <FeatureList />
            </Stack>
            <Divider orientation="vertical" mb={0} mt={0} ml={5} mr={5}></Divider>

            <Stack flex={1} style={{overflow: 'auto', height: '100%'}}>
                <FeatureDetails />
                <UseCaseList />
            </Stack>
            {selectedUseCaseId !== null && selectedUseCaseId > 0 && (
                <>
                    <Divider orientation="vertical" style={{
                        height: '100%',
                        margin: '0 5px',
                    }}></Divider>

                    <Stack className="flex-1" miw={400} style={{overflow: 'auto', height: '100%'}}>
                        <UseCaseDetails />
                    </Stack>
                </>
            )}
        </Flex>
    );
};

// Main Features component that sets up the context
const Features = () => {
    const [rootId, setRootId] = useState<number | null>(null);
    const [isLoading, setIsLoading] = useState<boolean>(true);
    const [loadError, setLoadError] = useState<string | null>(null);

    // Initialize root ID on component mount
    useEffect(() => {
        setIsLoading(true);
        setLoadError(null);

        const fetchData = async () => {
            try {
                const rootIdFromStorage = sessionStorage.getItem("rootId");
                const rootId = rootIdFromStorage ? parseInt(rootIdFromStorage, 10) : null;
                if (!rootId) {
                    setLoadError("No root found. Please create a root first.");
                    setIsLoading(false);
                    return;
                }
                setRootId(rootId);
                setIsLoading(false);
            } catch (err) {
                const errorMessage = `Failed to fetch data: ${err}`;
                await error(errorMessage);
                setLoadError(errorMessage);
                setIsLoading(false);
            }
        };

        fetchData().catch((err) => {
            const errorMessage = `Unexpected error: ${err}`;
            error(errorMessage);
            setLoadError(errorMessage);
            setIsLoading(false);
        });
    }, []);

    // Main error fallback for the entire Features component
    const mainErrorFallback = (
        <Alert
            color="red"
            title="Something went wrong"
            className="p-10"
        >
            <p>There was an error loading the Features page. Please try again later.</p>
            <Group position="right" mt="md">
                <Button onClick={() => window.location.reload()} color="red" variant="light">
                    Reload Page
                </Button>
            </Group>
        </Alert>
    );

    // Loading state
    if (isLoading) {
        return (
            <div className="p-10">
                <Title order={1} mb="xl">Features</Title>
                <Alert color="blue" title="Loading">
                    Loading features data...
                </Alert>
            </div>
        );
    }

    // Error state
    if (loadError) {
        return (
            <div className="p-10">
                <Title order={1} mb="xl">Features</Title>
                <Alert
                    color="red"
                    title="Error Loading Data"
                >
                    <p>{loadError}</p>
                    <Group position="right" mt="md">
                        <Button onClick={() => window.location.reload()} color="red" variant="light">
                            Try Again
                        </Button>
                    </Group>
                </Alert>
            </div>
        );
    }

    return (
        <ErrorBoundary fallback={mainErrorFallback}>
            <div className="p-10">
                <Title order={1} mb="xl">Features</Title>
                
                {/* Wrap the components with FeatureProvider to provide feature data */}
                <FeatureProvider rootId={rootId}>
                    <FeaturesContent />
                </FeatureProvider>
            </div>
        </ErrorBoundary>
    );
};

export default Features;
