import { useState, useEffect } from 'react';
import { Alert, Button, Group, LoadingOverlay, Paper, Select, Stack, TextInput, Title } from '@mantine/core';
import { info } from '@tauri-apps/plugin-log';
import { CreateGlobalDTO, GlobalDTO } from '../services/global-service';
import { useGlobal } from '../hooks/useGlobal';
import { useRoot } from '../hooks/useRoot';
import ErrorBoundary from '@/components/ErrorBoundary';

const Project = () => {
    // Get the root entity (default ID is 1)
    const { root, isLoading: isLoadingRoot } = useRoot();
    
    // Use the root ID to get the global configuration
    const { 
        global, 
        isLoading: isLoadingGlobal, 
        error: globalError,
        createGlobal,
        updateGlobal,
        refetch: refetchGlobal
    } = useGlobal(root?.id || 1);
    
    // Form state
    const [formData, setFormData] = useState<CreateGlobalDTO>({
        language: 'Rust',
        application_name: '',
        organisation_name: '',
        organisation_domain: '',
        prefix_path: '',
    });
    
    const [isSubmitting, setIsSubmitting] = useState(false);

    // Language options
    const languageOptions = [
        {value: 'Rust', label: 'Rust'},
        {value: 'C++', label: 'C++'},
        {value: 'C++ / Qt', label: 'C++ / Qt'},
        {value: 'Python', label: 'Python'},
    ];

    // Update form data when global data changes
    useEffect(() => {
        if (global) {
            setFormData({
                language: global.language,
                application_name: global.application_name,
                organisation_name: global.organisation_name,
                organisation_domain: global.organisation_domain,
                prefix_path: global.prefix_path,
            });
        }
    }, [global]);

    // Handle form submission
    const handleSubmit = async (e: React.FormEvent) => {
        e.preventDefault();
        setIsSubmitting(true);
        
        try {
            if (global) {
                // Update existing global
                const updatedGlobal: GlobalDTO = {
                    id: global.id,
                    ...formData
                };
                updateGlobal(updatedGlobal);
                info("Global settings updated successfully");
            } else if (root) {
                // Create new global
                createGlobal(formData);
                info("Global settings created successfully");
            }
        } finally {
            setIsSubmitting(false);
        }
    };

    // Custom fallback component for error state
    const errorFallback = (
        <Alert color="yellow" title="Project settings could not be loaded">
            There was an issue loading the project settings. Please try again later.
        </Alert>
    );

    return (
        <ErrorBoundary fallback={errorFallback}>
            <div className="p-10" style={{ position: 'relative' }}>
                {/* Loading overlay */}
                <LoadingOverlay visible={isLoadingRoot || isLoadingGlobal || isSubmitting} overlayProps={{ blur: 2 }} />
                
                {/* Error message */}
                {globalError && (
                    <Alert color="red" title="Error loading project settings" mb="md">
                        {globalError instanceof Error ? globalError.message : 'An unknown error occurred'}
                    </Alert>
                )}
                
                <Title order={1} mb="xl">Project Settings</Title>

                <Paper shadow="xs" p="md" withBorder>
                    <form onSubmit={handleSubmit}>
                        <Stack gap="md">
                            <Select
                                label="Language"
                                placeholder="Select a language"
                                data={languageOptions}
                                value={formData.language}
                                onChange={(value) => setFormData({...formData, language: value || 'Rust'})}
                                required
                                disabled={isLoadingGlobal || isSubmitting}
                            />

                            <TextInput
                                label="Application Name"
                                placeholder="Enter application name"
                                value={formData.application_name}
                                onChange={(e) => setFormData({...formData, application_name: e.target.value})}
                                required
                                disabled={isLoadingGlobal || isSubmitting}
                            />

                            <TextInput
                                label="Organisation Name"
                                placeholder="Enter organisation name"
                                value={formData.organisation_name}
                                onChange={(e) => setFormData({...formData, organisation_name: e.target.value})}
                                required
                                disabled={isLoadingGlobal || isSubmitting}
                            />

                            <TextInput
                                label="Organisation Domain"
                                placeholder="Enter organisation domain (e.g., com.example)"
                                value={formData.organisation_domain}
                                onChange={(e) => setFormData({...formData, organisation_domain: e.target.value})}
                                required
                                disabled={isLoadingGlobal || isSubmitting}
                            />

                            <TextInput
                                label="Prefix Path"
                                placeholder="Enter prefix path"
                                value={formData.prefix_path}
                                onChange={(e) => setFormData({...formData, prefix_path: e.target.value})}
                                disabled={isLoadingGlobal || isSubmitting}
                            />

                            <Group align="right" mt="md">
                                <Button type="submit" loading={isSubmitting} disabled={isLoadingGlobal || isLoadingRoot}>
                                    {global ? 'Update Settings' : 'Create Settings'}
                                </Button>
                            </Group>
                        </Stack>
                    </form>
                </Paper>
            </div>
        </ErrorBoundary>
    );
}

export default Project;
