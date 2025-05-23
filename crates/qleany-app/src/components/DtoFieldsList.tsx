import {ReactNode, useEffect, useState} from 'react';
import {ActionIcon, Group, Title, Tooltip} from '@mantine/core';
import {DtoRelationshipField, getDto, setDtoRelationship} from "../controller/dto_controller";
import {error, info} from '@tauri-apps/plugin-log';
import {createDtoField, DtoFieldDto, DtoFieldType, getDtoFieldMulti} from "../controller/dto_field_controller.ts";
import ReorderableList from './ReorderableList';
import {listen} from '@tauri-apps/api/event';

interface DtoFieldsListProps {
    selectedDto: number | null;
    onSelectDtoField: (fieldId: number | null) => void;
}


const DtoFieldsList = ({
                           selectedDto, onSelectDtoField
                       }: DtoFieldsListProps) => {
    const [fields, setFields] = useState<DtoFieldDto[]>([]);
    const [selectedField, setSelectedField] = useState<number | null>(null);

    async function createNewDtoField() {
        if (!selectedDto) {
            await error("No DTO selected");
            return;
        }

        try {
            const dto = await getDto(selectedDto);
            // Find the selected DTO
            if (!dto) {
                await error("Selected DTO not found");
                return;
            }

            // Create field
            const fieldDto = {
                name: 'New Field',
                field_type: DtoFieldType.String,
                is_nullable: false,
                is_list: false,
            };

            const newField = await createDtoField(fieldDto);

            // Update DTO with the new field
            const updatedFields = [...dto.fields, newField.id];

            // Update the relationship
            await setDtoRelationship({
                id: dto.id,
                field: DtoRelationshipField.Fields,
                right_ids: updatedFields,
            });

            // Refresh data
            await fetchFieldData();

            // Select the newly created field
            setSelectedField(newField.id);

            await info("DTO field created successfully");
        } catch (err) {
            await error(`Failed to create DTO field: ${err}`);
        }
    }

    useEffect(() => {

        // mounting the event listeners
        const unlisten_direct_access_dto_field_updated = listen('direct_access_dto_field_updated', async (event) => {
            const payload = event.payload as { ids: number[] };
            info(`Dto Field updated event received: ${payload.ids}`);
            const updatedDtoFields = await getDtoFieldMulti(payload.ids);

            for (const updatedDtoField of updatedDtoFields) {
                if (!updatedDtoField) {
                    info(`Use case not found in the current state.`);
                    continue;
                }

                const index = fields.findIndex((dtoField) => dtoField.id === updatedDtoField.id);
                if (index !== -1) {
                    const updatedDtoFieldsList = [...fields];
                    updatedDtoFieldsList[index] = updatedDtoField;
                    setFields(updatedDtoFieldsList);
                } else {
                    info(`Use case not found in the current state.`);
                }

            }

        });


        return () => {
            unlisten_direct_access_dto_field_updated.then(f => f());
        };

    }, [fields]);

    // Function to fetch field data from the backend
    async function fetchFieldData() {
        if (!selectedDto) return;

        const dto = await getDto(selectedDto);
        if (!dto) return;

        const fieldIds = dto.fields;
        const fields = await getDtoFieldMulti(fieldIds);
        setFields(fields.filter((field) => field !== null) as DtoFieldDto[]);
    }

    // change fields when selected DTO changes
    useEffect(() => {
        if (selectedDto) {
            // Fetch field data using the field IDs from the selected DTO
            const fetchFields = async () => {
                try {
                    const dtoData = await getDto(selectedDto);
                    if (!dtoData) {
                        setFields([]);
                        return;
                    }
                    const fieldData = await getDtoFieldMulti(dtoData.fields);
                    const validFields = fieldData.filter((field): field is DtoFieldDto => field !== null);
                    setFields(validFields);

                    setSelectedField(null);
                    onSelectDtoField(null);

                } catch (err) {
                    error(`Failed to fetch DTO fields: ${err}`);
                }
            };

            fetchFields();
        } else {
            setFields([]);
        }
    }, [selectedDto]);

    if (!selectedDto) {
        return null;
    }

    // Create header component for ReorderableList
    const header = (
        <Group id="dtoFieldsListHeading">
            <Title order={3}>DTO Fields</Title>
            <Tooltip label="Add new DTO field">
                <ActionIcon
                    variant="filled"
                    aria-label="Add new DTO field"
                    onClick={createNewDtoField}
                >
                    +
                </ActionIcon>
            </Tooltip>
        </Group>
    );

    // Define renderItemContent function for field items
    const renderFieldContent = (field: DtoFieldDto): ReactNode => (
        <div>
            <div>{field.name}</div>
            <div style={{color: 'dimmed', fontSize: 'small'}}>
                {field.field_type}
                {field.is_nullable ? ' (nullable)' : ''}
                {field.is_list ? ' (list)' : ''}
            </div>
        </div>
    );

    // Define onReorder function to handle reordering
    const handleReorder = async (reorderedIds: number[]): Promise<void> => {
        if (!selectedDto) return;

        try {
            // Update the DTO with the new field order
            await setDtoRelationship({
                id: selectedDto,
                field: DtoRelationshipField.Fields,
                right_ids: reorderedIds,
            });

            info("DTO field order updated successfully");
            await fetchFieldData();
        } catch (err) {
            error(`Failed to update DTO field order: ${err}`);
        }
    };

    // Define onSelectItem function
    const handleSelectItem = (fieldId: number): void => {
        setSelectedField(fieldId);
        onSelectDtoField(fieldId);
    };

    return (
        <ReorderableList
            items={fields}
            selectedItemId={selectedField}
            onSelectItem={handleSelectItem}
            onReorder={handleReorder}
            getItemId={(field) => field.id}
            renderItemContent={renderFieldContent}
            droppableId="dto-fields-list"
            draggableIdPrefix="dto-field"
            itemType="dto-field"
            header={header}
        />
    );
};

export default DtoFieldsList;