import {useEffect, useState} from 'react';
import {ActionIcon, Button, Group, Modal, Popover, Stack, Text, TextInput} from '@mantine/core';
import {IconCheck, IconX} from '@tabler/icons-react';
import {createDto, DtoDto, getDtoMulti} from "../../controller/dto_controller.ts";
import {error, info} from '@tauri-apps/plugin-log';
import {getUseCaseRelationship, UseCaseRelationshipField} from "../../controller/use_case_controller.ts";

interface DtoSelectorProps {
    value: number | null;
    useCaseId: number;
    isDtoOut: boolean;
    onChange: (dtoId: number | null) => void;
    label: string;
}

const DtoSelector = ({value, useCaseId, isDtoOut, onChange, label}: DtoSelectorProps) => {
    const [_, setDtos] = useState<DtoDto[]>([]);
    const [modalOpened, setModalOpened] = useState(false);
    const [popoverOpened, setPopoverOpened] = useState(false);
    const [newDtoName, setNewDtoName] = useState('');
    const [loading, setLoading] = useState(false);
    const [currentDto, setCurrentDto] = useState<DtoDto | null>(null);

    // Fetch all DTOs
    useEffect(() => {
        const fetchDtos = async () => {
            try {

                const field = isDtoOut ? UseCaseRelationshipField.DtoOut : UseCaseRelationshipField.DtoIn;

                const dtoIdList = await getUseCaseRelationship(useCaseId, field);
                const dtoList = await getDtoMulti(dtoIdList);
                const validDtos = dtoList.filter((dto): dto is DtoDto => dto !== null);
                setDtos(validDtos);
                // If a value is selected, find the corresponding DTO
                if (value !== null) {
                    const selectedDto = validDtos.find(dto => dto.id === value);
                    info(`dto.id: ${value}`);
                    info(`Fetched DTOs: ${validDtos.map(dto => dto.id).join(', ')}`);
                    info(`Selected DTO: ${selectedDto ? selectedDto.name : 'None'}`);
                    setCurrentDto(selectedDto || null);
                }
            } catch (err) {
                error(`Failed to fetch DTOs: ${err}`);
            }
        };

        fetchDtos();
    }, []);

    const handleCreateDto = async () => {
        if (!newDtoName.trim()) {
            error("DTO name cannot be empty");
            return;
        }

        setLoading(true);
        try {
            const newDto = await createDto({
                name: newDtoName,
                fields: [],
            });

            // Update the list of DTOs
            setDtos(prev => [...prev, newDto]);

            // Select the newly created DTO
            onChange(newDto.id);

            // Close the modal and reset the form
            setModalOpened(false);
            setNewDtoName('');
            setCurrentDto(newDto);

            info("DTO created successfully");
        } catch (err) {
            error(`Failed to create DTO: ${err}`);
        } finally {
            setLoading(false);
        }
    };


    return (
        <>
            <Group align="flex-end">
                <div style={{flex: 1}}>
                    <Text size="sm" fw={500}>{label}</Text>
                    <Text>
                        {currentDto ? currentDto.name : 'None'}
                    </Text>
                </div>
                {value === null ? (
                    <ActionIcon
                        variant="filled"
                        aria-label="Create new DTO"
                        onClick={() => setModalOpened(true)}
                    >
                        +
                    </ActionIcon>
                ) : (
                    <Popover
                        opened={popoverOpened}
                        onChange={setPopoverOpened}
                        position="bottom"
                        width={100}
                        withArrow
                        shadow="md"
                        trapFocus
                    >
                        <Popover.Target>
                            <ActionIcon
                                variant="filled"
                                color="red"
                                aria-label="Remove DTO"
                                onClick={() => setPopoverOpened(true)}
                            >
                                -
                            </ActionIcon>
                        </Popover.Target>
                        <Popover.Dropdown>
                            <Group justify="center" gap="xs">
                                <ActionIcon
                                    variant="light"
                                    color="gray"
                                    onClick={() => setPopoverOpened(false)}
                                    aria-label="Cancel"
                                >
                                    <IconX size={16}/>
                                </ActionIcon>
                                <ActionIcon
                                    variant="filled"
                                    color="red"
                                    onClick={() => {
                                        onChange(null);
                                        setCurrentDto(null);
                                        setPopoverOpened(false);
                                    }}
                                    aria-label="Confirm"
                                >
                                    <IconCheck size={16}/>
                                </ActionIcon>
                            </Group>
                        </Popover.Dropdown>
                    </Popover>
                )}
            </Group>

            <Modal
                opened={modalOpened}
                onClose={() => setModalOpened(false)}
                title="Create New DTO"
            >
                <Stack>
                    <TextInput
                        label="DTO Name"
                        value={newDtoName}
                        onChange={(e) => setNewDtoName(e.target.value)}
                        required
                    />
                    <Button onClick={handleCreateDto} loading={loading}>
                        Create DTO
                    </Button>
                </Stack>
            </Modal>
        </>
    );
};

export default DtoSelector;
