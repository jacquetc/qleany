import React, {useEffect, useMemo, useState} from 'react';
import {Alert, Box, Button, Code, Group, ScrollArea, Text, Title} from '@mantine/core';
import {error} from '@tauri-apps/plugin-log';
import {useRoot} from '@/hooks/useRoot';
import {useRustFileGeneration} from '@/hooks/useRustFileGeneration';
import GroupList from '@/components/generate/GroupList';
import FileList from '@/components/generate/FileList';
import ErrorBoundary from '@/components/ErrorBoundary';
import {FileProvider, useFileContext} from '@/contexts/FileContext';
import {GenerateRustCodeReturnDTO} from '@/services/rust-file-generation-service';

// CodeDisplay component that monitors file selection and generates code
const CodeDisplay = ({
                         generateRustCode
                     }: {
    generateRustCode: (fileId: number) => Promise<GenerateRustCodeReturnDTO>;
}) => {
    const {selectedFileId, files} = useFileContext();
    
    // Internal state for code generation
    const [generatedCode, setGeneratedCode] = useState<GenerateRustCodeReturnDTO | null>(null);
    const [isGeneratingCode, setIsGeneratingCode] = useState(false);
    const [codeGenerationError, setCodeGenerationError] = useState<string | null>(null);

    // Effect to generate code when a file is selected
    useEffect(() => {
        if (selectedFileId) {
            const generateCode = async () => {
                setIsGeneratingCode(true);
                setCodeGenerationError(null);
                setGeneratedCode(null);

                try {
                    const result = await generateRustCode(selectedFileId);
                    setGeneratedCode(result);
                } catch (err) {
                    const errorMessage = `Failed to generate code: ${err}`;
                    error(errorMessage);
                    setCodeGenerationError(errorMessage);
                } finally {
                    setIsGeneratingCode(false);
                }
            };

            generateCode();
        } else {
            setGeneratedCode(null);
            setCodeGenerationError(null);
        }
    }, [selectedFileId]);

    // Get the selected file info
    const selectedFile = selectedFileId ? files.find(f => f.id === selectedFileId) : null;

    // Loading state
    if (isGeneratingCode) {
        return (
            <Box h="100%">
                <Title order={4} mb="md">Generated Code</Title>
                <Alert color="blue" title="Generating code">
                    Generating code for the selected file...
                </Alert>
            </Box>
        );
    }

    // Error state
    if (codeGenerationError) {
        return (
            <Box h="100%">
                <Title order={4} mb="md">Generated Code</Title>
                <Alert color="red" title="Code Generation Error">
                    {codeGenerationError}
                </Alert>
            </Box>
        );
    }

    // No file selected
    if (!selectedFileId || !selectedFile) {
        return (
            <Box h="100%">
                <Title order={4} mb="md">Generated Code</Title>
                <Alert color="gray" title="No file selected">
                    Click on a file to generate and view its content.
                </Alert>
            </Box>
        );
    }

    // Display generated code
    return (
        <Box h="100%">
            <Title 
                order={4} 
                mb="md"
                style={{
                    direction: 'rtl',
                    textAlign: 'left',
                    overflow: 'hidden',
                    whiteSpace: 'nowrap',
                    textOverflow: 'ellipsis'
                }}
                title={`Generated Code - ${selectedFile.relative_path}${selectedFile.name}`}
            >
                <span style={{ direction: 'ltr' }}>
                    Generated Code - {selectedFile.relative_path}{selectedFile.name}
                </span>
            </Title>
            {generatedCode ? (
                <ScrollArea h="calc(100% - 60px)">
                    <Code block>
                        {generatedCode.generated_code}
                    </Code>
                    <Text size="xs" c="dimmed" mt="xs">
                        Generated on: {new Date(generatedCode.timestamp).toLocaleString()}
                    </Text>
                </ScrollArea>
            ) : (
                <Alert color="gray" title="No code generated">
                    No code has been generated yet.
                </Alert>
            )}
        </Box>
    );
};

const Generate = () => {
    // Use the root hook to get the root entity
    const {root, isLoading: isLoadingRoot, error: rootError} = useRoot();

    // Use the rust file generation hook
    const {
        isListing,
        operationError,
        listRustFiles,
        generateRustCode
    } = useRustFileGeneration();

    const [loadError, setLoadError] = useState<string | null>(null);

    // Memoize rootId to prevent FileProvider from unmounting/remounting during state changes
    const stableRootId = useMemo(() => {
        return root?.id || null;
    }, [root?.id]);

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
    if (isLoadingRoot || isListing) {
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
                <FileProvider rootId={stableRootId}>
                    <div style={{display: 'flex', height: 'calc(100vh - 170px)'}}>
                        <div style={{
                            width: '25%',
                            height: '100%',
                            overflow: 'hidden',
                            paddingRight: '10px',
                            boxSizing: 'border-box'
                        }}>
                            <GroupList rootId={stableRootId}/>
                        </div>
                        <div style={{
                            width: '35%',
                            height: '100%',
                            overflow: 'hidden',
                            padding: '0 10px',
                            boxSizing: 'border-box'
                        }}>
                            <FileList rootId={stableRootId}/>
                        </div>
                        <div style={{
                            width: '40%',
                            height: '100%',
                            overflow: 'hidden',
                            paddingLeft: '10px',
                            boxSizing: 'border-box'
                        }}>
                            <CodeDisplay
                                generateRustCode={generateRustCode}
                            />
                        </div>
                    </div>
                </FileProvider>
            </div>
        </ErrorBoundary>
    );
}

export default Generate;
