import {useEffect, useState} from 'react';
import {Button, Group, Paper, Select, Stack, TextInput, Title} from '@mantine/core';
import {error, info} from '@tauri-apps/plugin-log';
import {createGlobal, CreateGlobalDTO, getGlobal, GlobalDto, updateGlobal} from '#controller/global_controller';
import {getRootMulti, getRootRelationship, RootRelationshipField} from '#controller/root_controller';
import {listen} from "@tauri-apps/api/event";

const Project = () => {
    const [globalId, setGlobalId] = useState<number | null>(null);
    const [formData, setFormData] = useState<CreateGlobalDTO>({
        language: 'Rust',
        application_name: '',
        organisation_name: '',
        organisation_domain: '',
        prefix_path: '',
    });
    const [loading, setLoading] = useState(false);
    const [isEditing, setIsEditing] = useState(false);
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

    // Language options
    const languageOptions = [
        {value: 'Rust', label: 'Rust'},
        {value: 'C++', label: 'C++'},
        {value: 'C++ / Qt', label: 'C++ / Qt'},
        {value: 'Python', label: 'Python'},
    ];

    // Fetch global data on component mount
    useEffect(() => {
        fetchGlobalData();


        const unlisten_direct_access_global_update = listen('direct_access_global_updated', () => {

            info(`Direct access global updated event received`);
            fetchGlobalData().catch((err => error(err)));
        });

        const unlisten_direct_access_all_reset = listen('direct_access_all_reset', () => {
            info(`Direct access all reset event received`);
            fetchGlobalData().catch((err => error(err)));
        });

        return () => {
            unlisten_direct_access_global_update.then(f => f());
            unlisten_direct_access_all_reset.then(f => f());
        }
    }, []);

    const fetchGlobalData = async () => {
        try {
            setLoading(true);
            // Get the global ID from the root relationship
            const currentRootId = await getRootId();
            const rootGlobalId = await getRootRelationship(currentRootId, RootRelationshipField.Global);

            if (rootGlobalId && rootGlobalId.length > 0) {
                const id = rootGlobalId[0];
                setGlobalId(id);

                // Fetch the global data
                const globalData = await getGlobal(id);
                if (globalData) {
                    info(`Global data fetched successfully : ${JSON.stringify(globalData)}`);
                    // Update form data with fetched data
                    setFormData({
                        language: globalData.language,
                        application_name: globalData.application_name,
                        organisation_name: globalData.organisation_name,
                        organisation_domain: globalData.organisation_domain,
                        prefix_path: globalData.prefix_path,
                    });
                    setIsEditing(true);
                }
            }
        } catch (err) {
            error(`Failed to fetch global data: ${err}`);
        } finally {
            setLoading(false);
        }
    };

    const handleSubmit = async (e: React.FormEvent) => {
        e.preventDefault();

        try {
            setLoading(true);

            if (isEditing && globalId) {
                // Update existing global
                const updatedGlobal: GlobalDto = {
                    id: globalId,
                    ...formData
                };
                await updateGlobal(updatedGlobal);
                info("Global settings updated successfully");
            } else {
                // Create new global
                const newGlobal = await createGlobal(formData);
                setGlobalId(newGlobal.id);
                setIsEditing(true);
                info("Global settings created successfully");
            }
        } catch (err) {
            error(`Failed to save global settings: ${err}`);
        } finally {
            setLoading(false);
        }
    };

    return (
        <div className="p-10">
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
                        />

                        <TextInput
                            label="Application Name"
                            placeholder="Enter application name"
                            value={formData.application_name}
                            onChange={(e) => setFormData({...formData, application_name: e.target.value})}
                            required
                        />

                        <TextInput
                            label="Organisation Name"
                            placeholder="Enter organisation name"
                            value={formData.organisation_name}
                            onChange={(e) => setFormData({...formData, organisation_name: e.target.value})}
                            required
                        />

                        <TextInput
                            label="Organisation Domain"
                            placeholder="Enter organisation domain (e.g., com.example)"
                            value={formData.organisation_domain}
                            onChange={(e) => setFormData({...formData, organisation_domain: e.target.value})}
                            required
                        />

                        <TextInput
                            label="Prefix Path"
                            placeholder="Enter prefix path"
                            value={formData.prefix_path}
                            onChange={(e) => setFormData({...formData, prefix_path: e.target.value})}

                        />

                        <Group align="right" mt="md">
                            <Button type="submit" loading={loading}>
                                {isEditing ? 'Update Settings' : 'Create Settings'}
                            </Button>
                        </Group>
                    </Stack>
                </form>
            </Paper>
        </div>
    );
}

export default Project;
