import EntitiesIcon from "../assets/entities.svg?react";
import FeaturesIcon from "../assets/features.svg?react";
import {  NavLink as RouterNavLink } from "react-router";
import { Box, NavLink as MantineNavLink} from '@mantine/core';
import { IconHome2, IconChevronRight } from '@tabler/icons-react';
import { useState } from "react";

const data = [
  {
    href: "/home",
    icon: IconHome2, 
    label: 'Home',
    description: null,
  },
  {
    href:"/entities",
    icon: EntitiesIcon,
    label: 'Entities',
    rightSection: <IconChevronRight size={16} stroke={1.5} />,
  },
  {
    href: "/features",
    icon: FeaturesIcon,
    label: 'Features',
    rightSection: <IconChevronRight size={16} stroke={1.5} />,
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
      leftSection={<item.icon width={24} height={24} />}
      onClick={() => setActive(index)}
      renderRoot={(props) => <RouterNavLink to={item.href} {...props} />}


    />
  ));


  return (
    <Box >{items}</Box>
    

    
  );
}


export default NavBar;

