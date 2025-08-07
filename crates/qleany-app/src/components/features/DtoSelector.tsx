import { useEffect, useState } from 'react';
import { ActionIcon, Button, Group, Modal, Popover, Stack, Text, TextInput } from '@mantine/core';
import { IconCheck, IconX } from '@tabler/icons-react';
import { error, info } from '@tauri-apps/plugin-log';
import { useFeatureContext } from '@/contexts/FeatureContext';
import { DtoDTO } from '@/services/dto-service';
import { UseCaseRelationshipField } from '@/services/use-case-service';

interface DtoSelectorProps {
    value: number | null;
    useCaseId: number;
    isDtoOut: boolean;
    onChange: (dtoId: number | null) => void;
    label: string;
}

const DtoSelector = ({ value, useCaseId, isDtoOut, onChange, label }: DtoSelectorProps) => {
    const {
        dtos,
        createDto
    } = useFeatureContext();
    
    const [modalOpened, setModalOpened] = useState(false);
    const [popoverOpened, setPopoverOpened] = useState(false);
    const [newDtoName, setNewDtoName] = useState('');
    const [loading, setLoading] = useState(false);
    const [currentDto, setCurrentDto] = useState<DtoDTO | null>(null);

    // Find the current DTO when value or dtos change
    useEffect(() => {
        if (value !== null) {
            const selectedDto = dtos.find(dto => dto.id === value);
            if (selectedDto) {
                setCurrentDto(selectedDto);
            }
        } else {
            setCurrentDto(null);
        }
    }, [value, dtos]);

    const handleCreateDto = async () => {
        if (!newDtoName.trim()) {
            error("DTO name cannot be empty");
            return;
        }

        setLoading(true);
        try {
            // Create the DTO using the context function
            createDto(newDtoName);
            
            // The newly created DTO will be added to the dtos array by the context
            // We'll select it in the next render cycle when it appears in the dtos array
            
            // Close the modal and reset the form
            setModalOpened(false);
            setNewDtoName('');
            
            info("DTO creation initiated");
        } catch (err) {
            error(`Failed to create DTO: ${err}`);
        } finally {
            setLoading(false);
        }
    };


    // Effect to select the newly created DTO when it appears in the dtos array
    useEffect(() => {
        // If we're in the process of creating a DTO (modal was just closed)
        if (!modalOpened && newDtoName === '' && !currentDto && dtos.length > 0) {
            // Find the most recently created DTO (assuming it has the highest ID)
            const latestDto = [...dtos].sort((a, b) => b.id - a.id)[0];
            if (latestDto) {
                onChange(latestDto.id);
                setCurrentDto(latestDto);
                info(`Selected newly created DTO: ${latestDto.name}`);
            }
        }
    }, [dtos, modalOpened, newDtoName, currentDto, onChange]);

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
