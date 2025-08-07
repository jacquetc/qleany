import React, {createContext, useContext, useState} from 'react';
import {Alert} from '@mantine/core';
import {error as logError} from '@tauri-apps/plugin-log';
import {useHandlingManifest} from '../hooks/useHandlingManifest';
import {useSystem} from '../hooks/useSystem';

/**
 * Interface for the HomeContext value
 */
interface HomeContextValue {
    // Manifest handling data
    isManifestLoading: boolean;
    manifestError: string | null;
    manifestStatus: {
        loaded: boolean;
        saved: boolean;
        path?: string;
    };

    // System data
    isSystemLoading: boolean;
    systemError: string | null;

    // Combined loading and error states
    isLoading: boolean;
    errorMessage: string | null;

    // Hook error
    hookError: Error | null;

    // Manifest actions
    handleNewManifest: () => Promise<void>;
    handleOpenManifest: () => Promise<void>;
    handleSaveManifest: () => Promise<void>;
    handleCloseManifest: () => Promise<void>;
    handleOpenQleanyManifest: () => Promise<void>;

    // System actions
    handleExit: () => Promise<void>;
}

// Create the context with a default value
const HomeContext = createContext<HomeContextValue | undefined>(undefined);

/**
 * Props for the HomeProvider component
 */
interface HomeProviderProps {
    children: React.ReactNode;
}

/**
 * HomeProvider component that provides home-related data to its children
 */
export function HomeProvider({children}: HomeProviderProps) {
    // Add state for handling hook errors
    const [hookError, setHookError] = useState<Error | null>(null);

    // Use the custom hooks for data with error handling
    let manifestData = {
        isLoading: false,
        errorMessage: null,
        manifestStatus: {
            loaded: false,
            saved: false
        },
        handleNewManifest: async () => {
        },
        handleOpenManifest: async () => {
        },
        handleSaveManifest: async () => {
        },
        handleCloseManifest: async () => {
        },
        handleOpenQleanyManifest: async () => {
        }
    };

    let systemData = {
        isLoading: false,
        errorMessage: null,
        handleExit: async () => {
        }
    };

    try {
        manifestData = useHandlingManifest();
    } catch (err) {
        const errorMessage = `Error in useHandlingManifest hook: ${err}`;
        logError(errorMessage);
        setHookError(err instanceof Error ? err : new Error('Unknown error in useHandlingManifest hook'));
    }

    try {
        systemData = useSystem();
    } catch (err) {
        const errorMessage = `Error in useSystem hook: ${err}`;
        logError(errorMessage);
        setHookError(err instanceof Error ? err : new Error('Unknown error in useSystem hook'));
    }

    const {
        isLoading: isManifestLoading,
        errorMessage: manifestError,
        manifestStatus,
        handleNewManifest,
        handleOpenManifest,
        handleSaveManifest,
        handleCloseManifest,
        handleOpenQleanyManifest
    } = manifestData;

    const {
        isLoading: isSystemLoading,
        errorMessage: systemError,
        handleExit
    } = systemData;

    // Combine loading states and error messages from both hooks
    const isLoading = isManifestLoading || isSystemLoading;
    const errorMessage = manifestError || systemError;

    // Create the context value
    const contextValue: HomeContextValue = {
        // Manifest handling data
        isManifestLoading,
        manifestError,
        manifestStatus,

        // System data
        isSystemLoading,
        systemError,

        // Combined states
        isLoading,
        errorMessage,

        // Hook error
        hookError,

        // Manifest actions
        handleNewManifest,
        handleOpenManifest,
        handleSaveManifest,
        handleCloseManifest,
        handleOpenQleanyManifest,

        // System actions
        handleExit
    };

    // If there's a hook error, render an error message
    if (hookError) {
        return (
            <Alert color="red" title="Error initializing home data">
                <p>{hookError.message}</p>
                <p>Please try refreshing the page.</p>
            </Alert>
        );
    }

    return (
        <HomeContext.Provider value={contextValue}>
            {children}
        </HomeContext.Provider>
    );
}

/**
 * Custom hook to use the HomeContext
 * @returns The HomeContext value
 * @throws Error if used outside of a HomeProvider
 */
export function useHomeContext() {
    const context = useContext(HomeContext);

    if (context === undefined) {
        throw new Error('useHomeContext must be used within a HomeProvider');
    }

    return context;
}