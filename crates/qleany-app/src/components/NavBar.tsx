import EntitiesIcon from "../assets/entities.svg?react";
import FeaturesIcon from "../assets/features.svg?react";
import {NavLink as RouterNavLink} from "react-router";
import {Box, MantineColorScheme, NavLink as MantineNavLink, Select, useMantineColorScheme} from '@mantine/core';
import {IconBuildingFactory2, IconHome2, IconTag} from '@tabler/icons-react';
import {useState} from "react";

const data = [
    {
        href: "/home",
        icon: IconHome2,
        label: 'Home',
        description: null,
        rightSection: null,
    },
    {
        href: "/project",
        icon: IconTag,
        label: 'Project',
    },
    {
        href: "/entities",
        icon: EntitiesIcon,
        label: 'Entities',
    },
    {
        href: "/features",
        icon: FeaturesIcon,
        label: 'Features',
    },
    {
        href: "/generate",
        icon: IconBuildingFactory2,
        label: 'Generate',
    }
];

const NavBar = () => {

    const [active, setActive] = useState(0);

    const items = data.map((item, index) => (
        <MantineNavLink
            key={item.label}
            active={index === active}
            label={item.label}
            description={item.description}
            rightSection={item.rightSection}
            leftSection={<item.icon width={24} height={24}/>}
            onClick={() => setActive(index)}
            renderRoot={(props) => <RouterNavLink to={item.href} {...props} />}


        />
    ));

    const {setColorScheme, clearColorScheme, colorScheme} = useMantineColorScheme({
        keepTransitions: true,
    });

    const themeOptions = [
        {value: 'light', label: 'Light'},
        {value: 'dark', label: 'Dark'},
        {value: 'auto', label: 'Auto'},
    ];

    return (
        <Box>
            {items}
            <Select
                label="Theme"
                value={colorScheme}
                onChange={(value: string | null) => {
                    if (value === 'auto') {
                        clearColorScheme();
                    } else {
                        setColorScheme(value as MantineColorScheme);
                    }
                }}
                data={themeOptions}
            />
        </Box>
    );
}


export default NavBar;

