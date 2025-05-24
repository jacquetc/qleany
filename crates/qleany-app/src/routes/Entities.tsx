import {useState} from 'react';
import {Divider, Flex, Stack} from '@mantine/core';
import EntityList from '../components/entities/EntityList.tsx';
import EntityDetails from '../components/entities/EntityDetails.tsx';
import FieldsList from '../components/entities/FieldsList.tsx';
import FieldDetails from '../components/entities/FieldDetails.tsx';

const Entities = () => {
    const [selectedEntity, setSelectedEntity] = useState<number | null>(0);
    const [selectedField, setSelectedField] = useState<number | null>(0);


    return (
        <Flex direction="row" style={{
            height: '100%',
            margin: '0 0px',
        }}>
            <Stack miw={300} style={{overflow: 'auto', height: '100%'}}>
                <EntityList
                    onSelectEntity={setSelectedEntity}
                />
            </Stack>
            <Divider orientation="vertical" mb={0} mt={0} ml={5} mr={5}
            ></Divider>

            <Stack flex={1} style={{overflow: 'auto', height: '100%'}}>
                <EntityDetails
                    selectedEntity={selectedEntity}
                />
                <FieldsList
                    selectedEntity={selectedEntity}
                    onSelectField={setSelectedField}
                />
            </Stack>
            {selectedField !== null && selectedField > 0 && (
                <>
                    <Divider orientation="vertical" style={{
                        height: '100%',
                        margin: '0 5px',
                    }}></Divider>

                    <Stack className="flex-1" style={{overflow: 'auto', height: '100%'}}>
                        <FieldDetails selectedField={selectedField}/>
                    </Stack>
                </>
            )}
        </Flex>
    );
}

export default Entities;
