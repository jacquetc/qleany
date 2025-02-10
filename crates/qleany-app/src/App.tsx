import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";
import EntityList from "./EntityList.tsx";
import menuIcon from "./assets/menu.svg";
import entitiesIcon from "./assets/entities.svg";
import featuresIcon from "./assets/features.svg";

const App = () => {
  return (
    <div className="App">
      <div className="bg-base-200 absolute inset-y-0 left-0">
        <ul className="menu bg-base-200 rounded-box w-40 ">
          <li>
            <a>
              <img src={menuIcon} alt="Menu" className="h-15 w-15" />
              Menu
            </a>
          </li>
          <li>
            <a>
              <img src={entitiesIcon} alt="Entities" className="h-15 w-15" />
              Entities
            </a>
          </li>
          <li>
            <a>
              <img src={featuresIcon} alt="Features" className="h-15 w-15" />
              Features
            </a>
          </li>

        </ul>
        <ThemeComboBox />
      </div>
      <h1 className="text-3xl text-center">Qleany</h1>
      <EntityList />
    </div>
  );
}


const ThemeComboBox = () => {

  return (
    <div className="dropdown mb-72">
      <div tabIndex={0} role="button" className="btn m-1">
        Theme
        <svg
          width="12px"
          height="12px"
          className="inline-block h-2 w-2 fill-current opacity-60"
          xmlns="http://www.w3.org/2000/svg"
          viewBox="0 0 2048 2048">
          <path d="M1799 349l242 241-1017 1017L7 590l242-241 775 775 775-775z"></path>
        </svg>
      </div>
      <ul tabIndex={0} className="dropdown-content bg-base-300 rounded-box z-1 w-52 p-2 shadow-2xl">
        <li>
          <input
            type="radio"
            name="theme-dropdown"
            className="theme-controller w-full btn btn-sm btn-block btn-ghost justify-start"
            aria-label="Default"
            value="default" />
        </li>
        <li>
          <input
            type="radio"
            name="theme-dropdown"
            className="theme-controller w-full btn btn-sm btn-block btn-ghost justify-start"
            aria-label="Retro"
            value="retro" />
        </li>
        <li>
          <input
            type="radio"
            name="theme-dropdown"
            className="theme-controller w-full btn btn-sm btn-block btn-ghost justify-start"
            aria-label="Cyberpunk"
            value="cyberpunk" />
        </li>
        <li>
          <input
            type="radio"
            name="theme-dropdown"
            className="theme-controller w-full btn btn-sm btn-block btn-ghost justify-start"
            aria-label="Valentine"
            value="valentine" />
        </li>
        <li>
          <input
            type="radio"
            name="theme-dropdown"
            className="theme-controller w-full btn btn-sm btn-block btn-ghost justify-start"
            aria-label="Aqua"
            value="aqua" />
        </li>
      </ul>
    </div>
  )
}

export default App;

