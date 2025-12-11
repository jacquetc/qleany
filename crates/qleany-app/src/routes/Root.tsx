import {Outlet} from "react-router";
import NavBar from "#components/NavBar.tsx";
import {ActionIcon, Alert, AppShell, Burger, Button, Group, Title} from '@mantine/core';
import {useDisclosure} from '@mantine/hooks';
import {IconArrowBackUp, IconArrowForwardUp} from '@tabler/icons-react';
import {useUndoRedo} from '../hooks/useUndoRedo';
import ErrorBoundary from '@/components/ErrorBoundary';
import {HomeProvider, useHomeContext} from "../contexts/HomeContext";

const SaveButton = () => {
    const {handleSaveManifest} = useHomeContext();
    return (
        <Button onClick={handleSaveManifest}>Save</Button>
    );
};

const Root = () => {
    const [opened, {toggle}] = useDisclosure();
    const {canUndo: canUndoState, canRedo: canRedoState, undo: handleUndo, redo: handleRedo} = useUndoRedo();

    return (
        <ErrorBoundary fallback={<div>Something went wrong in the Root component.</div>}>
            <AppShell
                className={"no-select"}
                header={{height: {base: 60, md: 70, lg: 80}}}
                navbar={{
                    width: {base: 200, md: 300, lg: 400},
                    breakpoint: 'sm',
                    collapsed: {mobile: !opened},
                }}
                padding="md"
                style={{height: '100%', display: 'flex', flexDirection: 'column'}}
                onContextMenu={(e) => e.preventDefault()}
            >
                <AppShell.Header>
                    <Group h="100%" px="md" gap="xs">
                        <Group>
                            <Burger opened={opened} onClick={toggle} hiddenFrom="sm" size="sm"/>
                            <Title className="">Qleany</Title>
                        </Group>
                        <ActionIcon
                            variant="filled"
                            color="blue"
                            onClick={handleUndo}
                            disabled={!canUndoState}
                            title="Undo"
                        >
                            <IconArrowBackUp size={18}/>
                        </ActionIcon>
                        <ActionIcon
                            variant="filled"
                            color="blue"
                            onClick={handleRedo}
                            disabled={!canRedoState}
                            title="Redo"
                        >
                            <IconArrowForwardUp size={18}/>
                        </ActionIcon>
                        <HomeProvider>
                            <SaveButton/>
                        </HomeProvider>
                    </Group>
                </AppShell.Header>
                <AppShell.Navbar p="md" maw={200}>
                    <NavBar/>
                </AppShell.Navbar>
                <AppShell.Main
                    pl={205}
                    pb={5}
                    pt={80}
                    style={{
                        flex: '1 1 auto',
                        overflow: 'auto',
                        height: 'calc(100% - 80px)' /* Subtract header height */
                    }}
                >
                    <Outlet/>
                </AppShell.Main>
            </AppShell>
        </ErrorBoundary>
    );
}

export default Root;
