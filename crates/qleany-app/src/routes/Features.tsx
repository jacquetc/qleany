import ListColumn from "../components/ListColumn.tsx";
import "./DndListHandle.module.css"
import {Box, Text, Title} from '@mantine/core';

const Features = () => {

    const cards = [
        {
            id: 1,
            dndType: "card",
            content: <div className="listItem">Card 1</div>,
        },
        {
            id: 2,
            dndType: "card",
            content: <div className="listItem">Card 2</div>,
        },
        {
            id: 3,
            dndType: "card",
            content: <div className="listItem">Card 3</div>,
        },
    ];


    return (
        <Box style={{display: 'flex', flexDirection: 'column', height: '80vh'}}>
            <Title order={1}>Features</Title>
            <Text>This is the Features page.</Text>
            <ListColumn columnId={"1"} cards={cards} title={"Features"}/>
        </Box>
    );
}

export default Features;