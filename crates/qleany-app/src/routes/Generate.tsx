import React, {useEffect, useState} from 'react';
import {Alert, Button, Group, Title} from '@mantine/core';
import {error} from '@tauri-apps/plugin-log';
import {useRoot} from '@/hooks/useRoot';
import {useRustFileGeneration} from '@/hooks/useRustFileGeneration';
import GroupList from '@/components/generate/GroupList';
import FileList from '@/components/generate/FileList';
import ErrorBoundary from '@/components/ErrorBoundary';
import {FileProvider} from '@/contexts/FileContext';

const Generate = () => {
    // Use the root hook to get the root entity
    const {root, isLoading: isLoadingRoot, error: rootError} = useRoot();

    // Use the rust file generation hook
    const {
        isLoading: isLoadingFiles,
        operationError,
        listRustFiles
    } = useRustFileGeneration();

    const [loadError, setLoadError] = useState<string | null>(null);

    // Initialize on component mount
    useEffect(() => {
        const initializeData = async () => {
            try {
                // Check if root exists
                if (!root) {
                    setLoadError("No root found. Please create a root first.");
                    return;
                }

                // List rust files
                await listRustFiles(false);
            } catch (err) {
                const errorMessage = `Failed to initialize data: ${err}`;
                error(errorMessage);
                setLoadError(errorMessage);
            }
        };

        if (!isLoadingRoot && !rootError) {
            initializeData();
        }
    }, [root, isLoadingRoot, rootError, listRustFiles]);

    // Main error fallback for the entire Generate component
    const mainErrorFallback = (
        <Alert
            color="red"
            title="Something went wrong"
            className="p-10"
        >
            <p>There was an error loading the Generate page. Please try again later.</p>
            <Group align="right" mt="md">
                <Button onClick={() => window.location.reload()} color="red" variant="light">
                    Reload Page
                </Button>
            </Group>
        </Alert>
    );

    // Loading state
    if (isLoadingRoot || isLoadingFiles) {
        return (
            <div className="p-10">
                <Title order={1} mb="xl">Generate</Title>
                <Alert color="blue" title="Loading">
                    Loading file generation data...
                </Alert>
            </div>
        );
    }

    // Error state - check for root error first
    if (rootError) {
        return (
            <div className="p-10">
                <Title order={1} mb="xl">Generate</Title>
                <Alert
                    color="red"
                    title="Error Loading Root Data"
                >
                    <p>{rootError instanceof Error ? rootError.message : 'Unknown error loading root data'}</p>
                    <Group align="right" mt="md">
                        <Button onClick={() => window.location.reload()} color="red" variant="light">
                            Try Again
                        </Button>
                    </Group>
                </Alert>
            </div>
        );
    }

    // Check for operation error
    if (operationError) {
        return (
            <div className="p-10">
                <Title order={1} mb="xl">Generate</Title>
                <Alert
                    color="red"
                    title="Error Loading File Data"
                >
                    <p>{operationError.message}</p>
                    <Group align="right" mt="md">
                        <Button onClick={() => window.location.reload()} color="red" variant="light">
                            Try Again
                        </Button>
                    </Group>
                </Alert>
            </div>
        );
    }

    // Check for custom load error
    if (loadError) {
        return (
            <div className="p-10">
                <Title order={1} mb="xl">Generate</Title>
                <Alert
                    color="red"
                    title="Error Loading Data"
                >
                    <p>{loadError}</p>
                    <Group align="right" mt="md">
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
                <Title order={1} mb="xl">Generate</Title>

                {/* Wrap the components with FileProvider to provide file and group data */}
                <FileProvider rootId={root?.id || null}>
                    <div style={{display: 'flex', height: 'calc(100vh - 170px)'}}>
                        <div style={{
                            width: '33%',
                            height: '100%',
                            overflow: 'hidden',
                            paddingRight: '10px',
                            boxSizing: 'border-box'
                        }}>
                            <GroupList rootId={root?.id || null}/>
                        </div>
                        <div style={{
                            width: '67%',
                            height: '100%',
                            overflow: 'hidden',
                            paddingLeft: '10px',
                            boxSizing: 'border-box'
                        }}>
                            <FileList rootId={root?.id || null}/>
                        </div>
                    </div>
                </FileProvider>
            </div>
        </ErrorBoundary>
    );
}

export default Generate;
