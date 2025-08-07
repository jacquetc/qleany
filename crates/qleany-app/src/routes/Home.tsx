import {Alert, Button, Group, LoadingOverlay, Paper, Space, Stack, Text, Title} from '@mantine/core';
import ErrorBoundary from '@/components/ErrorBoundary';
import {HomeProvider, useHomeContext} from "../contexts/HomeContext";

// Custom fallback component for error state
const errorFallback = (
    <Alert color="yellow" title="Home page could not be loaded">
        There was an issue loading the home page. Please try again later.
    </Alert>
);

const HomeContent = () => {
    const {
        isLoading,
        errorMessage,
        handleNewManifest,
        handleOpenManifest,
        handleSaveManifest,
        handleCloseManifest,
        handleOpenQleanyManifest,
        handleExit
    } = useHomeContext();

    return (
        <div className="p-10" style={{position: 'relative'}}>
            {/* Loading overlay */}
            <LoadingOverlay visible={isLoading} overlayProps={{blur: 2}}/>

            {/* Error message */}
            {errorMessage && (
                <Alert color="red" title="Error" mb="md" withCloseButton onClose={() => {
                }}>
                    {errorMessage}
                </Alert>
            )}

            <Title order={1} mb="xl">Qleany</Title>
            <Text size="lg" mb="xl">Welcome to Qleany! Use the buttons below to manage your manifests.</Text>

            <Paper shadow="xs" p="md" withBorder>
                <Stack gap="md">
                    <Group align="center" gap="md">
                        <Button onClick={handleNewManifest} loading={isLoading}>New Manifest</Button>
                        <Button onClick={handleOpenManifest} loading={isLoading}>Open Manifest</Button>
                        <Button onClick={handleSaveManifest} loading={isLoading}>Save Manifest</Button>
                        <Button onClick={handleCloseManifest} loading={isLoading}>Close Current
                            Manifest</Button>
                        <Button color="red" onClick={handleExit} loading={isLoading}>Exit</Button>
                    </Group>

                    <Space h="md"/>

                    <Paper shadow="xs" p="md" withBorder bg="gray.1">
                        <Text size="sm" mb="xs" fw={700}>For Testing</Text>
                        <Button variant="outline" onClick={handleOpenQleanyManifest} loading={isLoading}>
                            Open Qleany Manifest
                        </Button>
                    </Paper>
                </Stack>
            </Paper>
        </div>
    );
};

const Home = () => {
    return (
        <ErrorBoundary fallback={errorFallback}>
            <HomeProvider>
                <HomeContent/>
            </HomeProvider>
        </ErrorBoundary>
    );
}

export default Home;
