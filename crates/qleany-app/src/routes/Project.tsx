import {Alert, LoadingOverlay, Paper, Select, Stack, TextInput, Title} from '@mantine/core';
import ErrorBoundary from '@/components/ErrorBoundary';
import {ProjectProvider, useProjectContext} from '../contexts/ProjectContext';

const ProjectContent = () => {
    // Use the project context for all data and actions
    const {
        formData,
        isLoading,
        isSaving,
        error,
        updateFormData
    } = useProjectContext();

    // Language options
    const languageOptions = [
        {value: 'Rust', label: 'Rust'},
        {value: 'C++', label: 'C++'},
        {value: 'C++ / Qt', label: 'C++ / Qt'},
        {value: 'Python', label: 'Python'},
    ];

    return (
        <div className="p-10" style={{position: 'relative'}}>
            {/* Loading overlay */}
            <LoadingOverlay visible={isLoading || isSaving} overlayProps={{blur: 2}}/>

            {/* Error message */}
            {error && (
                <Alert color="red" title="Error loading project settings" mb="md">
                    {error instanceof Error ? error.message : 'An unknown error occurred'}
                </Alert>
            )}

            <Title order={1} mb="xl">Project Settings</Title>

            <Paper shadow="xs" p="md" withBorder>
                <form>
                    <Stack gap="md">
                        <Select
                            label="Language"
                            placeholder="Select a language"
                            data={languageOptions}
                            value={formData.language}
                            onChange={(value) => updateFormData({language: value || 'Rust'})}
                            required
                            disabled={isLoading || isSaving}
                        />

                        <TextInput
                            label="Application Name"
                            placeholder="Enter application name"
                            value={formData.application_name}
                            onChange={(e) => updateFormData({application_name: e.target.value})}
                            required
                            disabled={isLoading || isSaving}
                        />

                        <TextInput
                            label="Organisation Name"
                            placeholder="Enter organisation name"
                            value={formData.organisation_name}
                            onChange={(e) => updateFormData({organisation_name: e.target.value})}
                            required
                            disabled={isLoading || isSaving}
                        />

                        <TextInput
                            label="Organisation Domain"
                            placeholder="Enter organisation domain (e.g., com.example)"
                            value={formData.organisation_domain}
                            onChange={(e) => updateFormData({organisation_domain: e.target.value})}
                            required
                            disabled={isLoading || isSaving}
                        />

                        <TextInput
                            label="Prefix Path"
                            placeholder="Enter prefix path"
                            value={formData.prefix_path}
                            onChange={(e) => updateFormData({prefix_path: e.target.value})}
                            disabled={isLoading || isSaving}
                        />
                    </Stack>
                </form>
            </Paper>
        </div>
    );
};


// Custom fallback component for error state
const errorFallback = (
    <Alert color="yellow" title="Project settings could not be loaded">
        There was an issue loading the project settings. Please try again later.
    </Alert>
);

const Project = () => {
    return (
        <ErrorBoundary fallback={errorFallback}>
            <ProjectProvider>
                <ProjectContent/>
            </ProjectProvider>
        </ErrorBoundary>
    );
};
export default Project;