import {createTheme, MantineProvider} from '@mantine/core';
import {BrowserRouter, Route, Routes} from "react-router";
import Root from "./routes/Root";
import Home from "./routes/Home";
import Entities from "./routes/Entities";
import Features from "./routes/Features";
import Project from "./routes/Project";
import Generate from "./routes/Generate.tsx";
import EntityMap from "./routes/EntityMap.tsx";
import {useState} from 'react';


const App = () => {


    // @ts-ignore
    const [theme, setTheme] = useState(createTheme({
        primaryColor: 'teal'

    }));

    return (

        <MantineProvider theme={theme}>

            <BrowserRouter>
                <Routes>
                    <Route path="/" element={<Root/>}>
                        <Route index element={<Home/>}/>
                        <Route path="home" element={<Home/>}/>
                        <Route path="project" element={<Project/>}/>
                        <Route path="entities" element={<Entities/>}/>
                        <Route path="entity-map" element={<EntityMap/>}/>
                        <Route path="features" element={<Features/>}/>
                        <Route path="generate" element={<Generate/>}/>
                    </Route>
                </Routes>
            </BrowserRouter>

        </MantineProvider>

    );

}

export default App;