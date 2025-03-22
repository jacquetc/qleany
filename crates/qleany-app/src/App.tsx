
import { MantineProvider, createTheme } from '@mantine/core';
import { BrowserRouter, Routes, Route } from "react-router";
import Root from "./routes/Root";
import Home from "./routes/Home";
import Entities from "./routes/Entities";
import Features from "./routes/Features";
import { useState } from 'react';


const App = () => {


  const [theme, setTheme] = useState(createTheme({
    primaryColor: 'teal',

  }));

  return (

    <MantineProvider theme={theme}>

      <BrowserRouter>
        <Routes>
          <Route path="/" element={<Root />}>
            <Route index element={<Home />} />
            <Route path="home" element={<Home />} />
            <Route path="entities" element={<Entities />} />
            <Route path="features" element={<Features />} />
          </Route>
        </Routes>
      </BrowserRouter>

    </MantineProvider>

  );

}

export default App;