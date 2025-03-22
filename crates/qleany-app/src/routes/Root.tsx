import { Outlet } from "react-router";
import NavBar from "../components/NavBar";
import { AppShell, Burger, Group, Skeleton, Title } from '@mantine/core';
import { useDisclosure } from '@mantine/hooks';


const Root = () => {
  const [opened, { toggle }] = useDisclosure();
  return (


    <AppShell
      header={{ height: { base: 60, md: 70, lg: 80 } }}
      navbar={{
        width: { base: 200, md: 300, lg: 400 },
        breakpoint: 'sm',
        collapsed: { mobile: !opened },
      }}
      padding="md"

      onContextMenu={(e)=> e.preventDefault()}
    >
      <AppShell.Header>
        <Group h="100%" px="md">
          <Burger opened={opened} onClick={toggle} hiddenFrom="sm" size="sm" />
          <Title>Qleany</Title>
        </Group>
      </AppShell.Header>
      <AppShell.Navbar p="md">
        <NavBar />
      </AppShell.Navbar>
      <AppShell.Main>        <Outlet />
      </AppShell.Main>
    </AppShell>

  );
}

export default Root;