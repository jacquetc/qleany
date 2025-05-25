import {loadManifest, LoadManifestDto} from "#controller/handling-manifest-controller.ts";
import {Button, Group, Paper, Space, Stack, Text, Title} from '@mantine/core';
import {error, info} from '@tauri-apps/plugin-log';
import {open, save} from '@tauri-apps/plugin-dialog';
import {exit} from '@tauri-apps/plugin-process';
import {removeRoot} from "#controller/root-controller.ts";

const Home = () => {
    async function handleNewManifest() {
        info("New manifest action triggered");
        // This would typically create a new manifest
        // For now, just log a message
        info("New manifest functionality not yet implemented");
    }

    async function handleOpenManifest() {
        try {
            // Open file dialog to select a manifest file
            const selected = await open({
                multiple: false,
                directory: false,
                filters: [{
                    name: 'Manifest Files',
                    extensions: ['yaml', 'yml']
                }]
            });

            if (selected) {
                // If it's an array, take the first file
                const filePath = Array.isArray(selected) ? selected[0] : selected;

                const dto: LoadManifestDto = {
                    manifest_path: filePath
                };

                await loadManifest(dto);
                info(`Manifest loaded: ${filePath}`);
            }
        } catch (e) {
            error(`Failed to open manifest: ${e}`);
        }
    }

    async function handleSaveManifest() {
        info("Save manifest action triggered");
        const path = await save({
            filters: [
                {
                    name: 'Manifest Files',
                    extensions: ['yaml', 'yml'],
                },
            ],
        });
        console.log(path);
    }

    async function handleCloseManifest() {
        info("Close manifest action triggered");

        await removeRoot(1)
    }

    async function handleExit() {
        info("Exit application triggered");
        try {
            await exit(0);
        } catch (e) {
            error(`Failed to exit application: ${e}`);
        }
    }

    async function handleOpenQleanyManifest() {
        const dto = {
            manifest_path: "C:\\Users\\cyril\\Devel\\qleany\\qleany.yaml"
        }
        loadManifest(dto).catch(e => console.error(e));
    }

    return (
        <div className="p-10">
            <Title order={1} mb="xl">Qleany</Title>
            <Text size="lg" mb="xl">Welcome to Qleany! Use the buttons below to manage your manifests.</Text>

            <Paper shadow="xs" p="md" withBorder>
                <Stack gap="md">
                    <Group align="center" gap="md">
                        <Button onClick={handleNewManifest}>New Manifest</Button>
                        <Button onClick={handleOpenManifest}>Open Manifest</Button>
                        <Button onClick={handleSaveManifest}>Save Manifest</Button>
                        <Button onClick={handleCloseManifest}>Close Current Manifest</Button>
                        <Button color="red" onClick={handleExit}>Exit</Button>
                    </Group>

                    <Space h="md"/>

                    <Paper shadow="xs" p="md" withBorder bg="gray.1">
                        <Text size="sm" mb="xs" fw={700}>For Testing</Text>
                        <Button variant="outline" onClick={handleOpenQleanyManifest}>
                            Open Qleany Manifest
                        </Button>
                    </Paper>
                </Stack>
            </Paper>
        </div>
    );
}

export default Home;
