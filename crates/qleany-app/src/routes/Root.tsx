import {Outlet} from "react-router";
import NavBar from "../components/NavBar";
import {AppShell, Burger, Group, Title} from '@mantine/core';
import {useDisclosure} from '@mantine/hooks';


const Root = () => {
    const [opened, {toggle}] = useDisclosure();
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

            onContextMenu={(e) => e.preventDefault()}
        >
            <AppShell.Header>
                <Group h="100%" px="md">
                    <Burger opened={opened} onClick={toggle} hiddenFrom="sm" size="sm"/>
                    <Title className="">Qleany</Title>
                </Group>
            </AppShell.Header>
            <AppShell.Navbar p="md" maw={200}>
                <NavBar/>
            </AppShell.Navbar>
            <AppShell.Main pl={210} pb={20}
            > <Outlet/>
            </AppShell.Main>
        </AppShell>

    );
}

export default Root;