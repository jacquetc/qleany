import React from "react";
import ReactDOM from "react-dom/client";
import "./index.css";
import '@mantine/core/styles.css';
import App from "./App";
import {debug, error, info, trace, warn} from '@tauri-apps/plugin-log';
import { QueryProvider } from "./providers/QueryProvider";


function forwardConsole(
    fnName: 'log' | 'debug' | 'info' | 'warn' | 'error',
    logger: (message: string) => Promise<void>
) {
    const original = console[fnName];
    console[fnName] = (...args) => {
        original(...args);
        
        // Convert all arguments to a single string for logging with improved error handling
        const message = args.map(arg => {
            if (arg instanceof Error) {
                // Special handling for Error objects
                return `Error: ${arg.message}\nStack: ${arg.stack || 'No stack trace available'}`;
            } else if (typeof arg === 'object') {
                try {
                    // Try to stringify with a custom replacer to handle circular references
                    return JSON.stringify(arg, (key, value) => {
                        // Handle special cases like functions, undefined, etc.
                        if (typeof value === 'function') {
                            return '[Function]';
                        }
                        if (value === undefined) {
                            return 'undefined';
                        }
                        if (value instanceof Error) {
                            return {
                                name: value.name,
                                message: value.message,
                                stack: value.stack
                            };
                        }
                        return value;
                    }, 2);
                } catch (e) {
                    // Fallback if JSON.stringify fails (e.g., circular references)
                    return `[Object: ${Object.prototype.toString.call(arg)}]`;
                }
            } else {
                return String(arg);
            }
        }).join(' ');
        
        logger(message);
    };
}

forwardConsole('log', trace);
forwardConsole('debug', debug);
forwardConsole('info', info);
forwardConsole('warn', warn);
forwardConsole('error', error);


ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
    <React.StrictMode>
        <QueryProvider>
            <App/>
        </QueryProvider>
    </React.StrictMode>
);
