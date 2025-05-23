import { useEffect, useState } from 'react';
import { Button, Divider, Stack, TextInput, Title } from '@mantine/core';
import { DtoDto, getDto, updateDto } from "../controller/dto_controller";
import { error, info } from '@tauri-apps/plugin-log';
import DtoFieldsList from './DtoFieldsList';
import DtoFieldDetails from './DtoFieldDetails';

interface DtoDetailsProps {
    selectedDto: number | null;
}

const DtoDetails = ({ selectedDto }: DtoDetailsProps) => {
    const [formData, setFormData] = useState<{
        name: string;
    }>({
        name: '',
    });

    const [dtoData, setDtoData] = useState<DtoDto | null>(null);
    const [selectedDtoField, setSelectedDtoField] = useState<number | null>(null);
    const [loading, setLoading] = useState(false);

    // Fetch DTO data when selected DTO changes
    useEffect(() => {
        if (selectedDto) {
            const fetchDtoData = async () => {
                setLoading(true);
                try {
                    const data = await getDto(selectedDto);
                    if (data) {
                        setDtoData(data);
                        setFormData({
                            name: data.name,
                        });
                    }
                } catch (err) {
                    error(`Failed to fetch DTO data: ${err}`);
                } finally {
                    setLoading(false);
                }
            };

            fetchDtoData();
        }
    }, [selectedDto]);

    const handleSubmit = async (e: React.FormEvent) => {
        e.preventDefault();

        if (!dtoData) return;

        try {
            // Update the DTO with the form data
            const updatedDto: DtoDto = {
                ...dtoData,
                name: formData.name,
            };

            await updateDto(updatedDto);

            // Refresh data
            const refreshedData = await getDto(dtoData.id);
            if (refreshedData) {
                setDtoData(refreshedData);
            }

            info("DTO updated successfully");
        } catch (err) {
            error(`Failed to update DTO: ${err}`);
        }
    };

    if (!selectedDto || !dtoData) {
        return null;
    }

    return (
        <>
            <Title order={2}>"{formData.name}" details</Title>
            <form onSubmit={handleSubmit}>
                <Stack>
                    <TextInput
                        id="dtoName"
                        label="Name"
                        value={formData.name}
                        onChange={(e) => setFormData({ ...formData, name: e.target.value })}
                    />

                    <Button type="submit" loading={loading}>Save Changes</Button>
                </Stack>
            </form>

            <Divider my="md" />

            <Stack>
                <DtoFieldsList
                    selectedDto={selectedDto}
                    onSelectDtoField={setSelectedDtoField}
                />
            </Stack>

            {selectedDtoField && (
                <>
                    <Divider my="md" />
                    <Stack>
                        <DtoFieldDetails selectedDtoField={selectedDtoField} />
                    </Stack>
                </>
            )}
        </>
    );
};

export default DtoDetails;