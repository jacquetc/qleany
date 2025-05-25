import {Outlet} from "react-router";
import NavBar from "#components/NavBar";
import {ActionIcon, AppShell, Burger, Group, Title} from '@mantine/core';
import {useDisclosure} from '@mantine/hooks';
import {IconArrowBackUp, IconArrowForwardUp} from '@tabler/icons-react';
import {canRedo, canUndo, redo, undo} from '#controller/undo_redo_controller';
import {useEffect, useState} from 'react';


const Root = () => {
    const [opened, {toggle}] = useDisclosure();
    const [canUndoState, setCanUndoState] = useState(false);
    const [canRedoState, setCanRedoState] = useState(false);

    // Check if undo is available
    useEffect(() => {
        const checkUndoAvailability = async () => {
            try {
                const undoAvailable = await canUndo();
                setCanUndoState(undoAvailable);
            } catch (error) {
                console.error("Error checking undo availability:", error);
            }
        };

        // Check if redo is available
        const checkRedoAvailability = async () => {
            try {
                const redoAvailable = await canRedo();
                setCanRedoState(redoAvailable);
            } catch (error) {
                console.error("Error checking redo availability:", error);
            }
        };

        // Check initially
        checkUndoAvailability();
        checkRedoAvailability();

        // Set up interval to check periodically
        const intervalId = setInterval(checkUndoAvailability, 100);
        const redoIntervalId = setInterval(checkRedoAvailability, 100);

        // Clean up interval on unmount
        return () => {
            clearInterval(intervalId);
            clearInterval(redoIntervalId);
        }
    }, []);

    const handleUndo = async () => {
        try {
            await undo();
            // Update undo availability after performing undo
            const undoAvailable = await canUndo();
            setCanUndoState(undoAvailable);
            // Optionally, you can also check redo availability
            const redoAvailable = await canRedo();
            setCanRedoState(redoAvailable);

        } catch (error) {
            console.error("Error performing undo:", error);
        }
    };

    const handleRedo = async () => {
        try {
            await redo();
            // Update undo availability after performing redo
            const undoAvailable = await canUndo();
            setCanUndoState(undoAvailable);
            // Optionally, you can also check redo availability
            const redoAvailable = await canRedo();
            setCanRedoState(redoAvailable);
        } catch (error) {
            console.error("Error performing redo:", error);
        }
    }

    return (


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

    );
}

export default Root;
