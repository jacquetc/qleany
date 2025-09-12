import {useEffect, useState} from 'react';
import {Alert, Button, Divider, Flex, Group, Stack, Title} from '@mantine/core';
import EntityList from '#components/entities/EntityList.tsx';
import EntityDetails from '#components/entities/EntityDetails.tsx';
import FieldsList from '#components/entities/FieldsList.tsx';
import FieldDetails from '#components/entities/FieldDetails.tsx';
import ErrorBoundary from '@/components/ErrorBoundary';
import {EntityProvider, useEntityContext} from '@/contexts/EntityContext';

// Inner component that uses the EntityContext
const EntitiesContent = () => {
    const {
        selectedEntityId,
        selectedFieldId,
        selectEntity,
        selectField
    } = useEntityContext();

    return (
        <Flex direction="row" style={{
            height: '100%',
            margin: '0 0px',
        }}>
            <Stack miw={300} style={{overflow: 'auto', height: '100%'}}>
                <EntityList/>
            </Stack>
            <Divider orientation="vertical" mb={0} mt={0} ml={5} mr={5}></Divider>

            <Stack flex={1} style={{overflow: 'auto', height: '100%'}}>
                <EntityDetails/>
                <FieldsList/>
            </Stack>
            {selectedFieldId !== null && selectedFieldId > 0 && (
                <>
                    <Divider orientation="vertical" style={{
                        height: '100%',
                        margin: '0 5px',
                    }}></Divider>

                    <Stack className="flex-1" style={{overflow: 'auto', height: '100%'}}>
                        <FieldDetails/>
                    </Stack>
                </>
            )}
        </Flex>
    );
};

// Main Entities component that sets up the context
const Entities = () => {

    const [rootId, setRootId] = useState<number | null>(null);
    // Get rootId from sessionStorage
    useEffect(() => {
        const rootIdFromStorage = sessionStorage.getItem("rootId");
        const rootId = rootIdFromStorage ? parseInt(rootIdFromStorage, 10) : null;
        setRootId(rootId);
    }, []);

    // Main error fallback for the entire Entities component
    const mainErrorFallback = (
        <Alert
            color="red"
            title="Something went wrong"
            className="p-10"
        >
            <p>There was an error loading the Entities page. Please try again later.</p>
            <Group align="right" mt="md">
                <Button onClick={() => window.location.reload()} color="red" variant="light">
                    Reload Page
                </Button>
            </Group>
        </Alert>
    );

    return (
        <ErrorBoundary fallback={mainErrorFallback}>
            <div className="p-10">
                <Title order={1} mb="xl">Entities</Title>

                {/* Wrap the components with EntityProvider to provide entity data */}
                <EntityProvider rootId={rootId}>
                    <EntitiesContent/>
                </EntityProvider>
            </div>
        </ErrorBoundary>
    );
};

export default Entities;
