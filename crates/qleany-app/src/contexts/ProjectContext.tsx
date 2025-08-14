import React, {createContext, useContext, useEffect, useRef, useState} from 'react';
import {Alert} from '@mantine/core';
import {error as logError, info as logInfo} from '@tauri-apps/plugin-log';
import {CreateGlobalDTO, GlobalDTO} from '../services/global-service';
import {useGlobal} from '../hooks/useGlobal';
import {useRoot} from '../hooks/useRoot';
import { useUndoRedo } from '../hooks/useUndoRedo';

/**
 * Interface for the ProjectContext value
 */
interface ProjectContextValue {
    // Data
    global: GlobalDTO | null;
    formData: CreateGlobalDTO;
    
    // Loading states
    isLoading: boolean;
    isSaving: boolean;
    
    // Errors
    error: unknown;
    hookError: Error | null;
    
    // Actions
    updateFormData: (data: Partial<CreateGlobalDTO>) => void;
    resetForm: () => void;

    // Undo/Redo state and actions
    canUndo: boolean;
    canRedo: boolean;
    undo: () => Promise<void> | void;
    redo: () => Promise<void> | void;
    
    // Refetch capability
    refetch: () => Promise<void>;
}

// Create the context with a default value
const ProjectContext = createContext<ProjectContextValue | undefined>(undefined);

/**
 * Props for the ProjectProvider component
 */
interface ProjectProviderProps {
    children: React.ReactNode;
}

/**
 * ProjectProvider component that provides project-related data to its children
 */
export function ProjectProvider({children}: ProjectProviderProps) {
    // Add state for handling hook errors
    const [hookError, setHookError] = useState<Error | null>(null);
    
    // Form state
    const [formData, setFormData] = useState<CreateGlobalDTO>({
        language: 'Rust',
        application_name: '',
        organisation_name: '',
        organisation_domain: '',
        prefix_path: '',
    });
    
    const [isSaving, setIsSaving] = useState(false);
    const saveTimeoutRef = useRef<number | null>(null);
    const isLoadingDataRef = useRef(false);

    // Use the hooks with error handling
    let rootData = {
        root: null,
        isLoading: true,
        error: null
    };
    
    let globalData = {
        global: null,
        isLoading: true,
        error: null,
        createGlobal: (_: CreateGlobalDTO) => {},
        updateGlobal: (_: GlobalDTO) => {},
        removeGlobal: (_: number) => {},
        refetch: async () => {}
    };

    try {
        rootData = useRoot();
    } catch (err) {
        const errorMessage = `Error in useRoot hook: ${err}`;
        logError(errorMessage);
        setHookError(err instanceof Error ? err : new Error('Unknown error in useRoot hook'));
    }

    try {
        globalData = useGlobal(rootData.root?.id || 1);
    } catch (err) {
        const errorMessage = `Error in useGlobal hook: ${err}`;
        logError(errorMessage);
        setHookError(err instanceof Error ? err : new Error('Unknown error in useGlobal hook'));
    }

    const {
        root,
        isLoading: isLoadingRoot,
        error: rootError
    } = rootData;

    const {
        global,
        isLoading: isLoadingGlobal,
        error: globalError,
        createGlobal,
        updateGlobal,
        refetch
    } = globalData;

    // Undo/Redo hook (event-driven)
    const { canUndo, canRedo, undo, redo } = useUndoRedo();

    // Update form when global data changes
    useEffect(() => {
        if (global) {
            // Flag that we're loading data from external source
            isLoadingDataRef.current = true;

            const snapshot: CreateGlobalDTO = {
                language: global.language,
                application_name: global.application_name,
                organisation_name: global.organisation_name,
                organisation_domain: global.organisation_domain,
                prefix_path: global.prefix_path,
            };
            setFormData(snapshot);

            // Reset flag after a brief timeout to allow state update to complete
            setTimeout(() => {
                isLoadingDataRef.current = false;
            }, 0);
        }
    }, [global]);

    // Autosave on formData changes (debounced)
    useEffect(() => {
        if (isLoadingGlobal || isLoadingRoot) return;
        if (!global) return; // Skip if global data hasn't loaded yet
        if (isLoadingDataRef.current) return; // Skip if formData change is from external data loading

        if (saveTimeoutRef.current) {
            window.clearTimeout(saveTimeoutRef.current);
        }

        saveTimeoutRef.current = window.setTimeout(() => {
            setIsSaving(true);
            try {
                const updatedGlobal: GlobalDTO = {id: global.id, ...formData};
                updateGlobal(updatedGlobal);
                logInfo("Project settings auto-saved successfully");
            } catch (err) {
                logError(`Error auto-saving project settings: ${err}`);
            } finally {
                setIsSaving(false);
            }
        }, 500);

        return () => {
            if (saveTimeoutRef.current) {
                window.clearTimeout(saveTimeoutRef.current);
            }
        };
    }, [formData, global, isLoadingGlobal, isLoadingRoot, updateGlobal]);

    // Function to update form data
    const updateFormData = (data: Partial<CreateGlobalDTO>) => {
        setFormData(prev => ({...prev, ...data}));
    };

    // Function to reset form to initial state
    const resetForm = () => {
        if (global) {
            const initialData: CreateGlobalDTO = {
                language: global.language,
                application_name: global.application_name,
                organisation_name: global.organisation_name,
                organisation_domain: global.organisation_domain,
                prefix_path: global.prefix_path,
            };
            setFormData(initialData);
        } else {
            setFormData({
                language: 'Rust',
                application_name: '',
                organisation_name: '',
                organisation_domain: '',
                prefix_path: '',
            });
        }
    };

    // Combine loading states and errors
    const isLoading = isLoadingRoot || isLoadingGlobal;
    const combinedError = rootError || globalError;

    // Create the context value
    const contextValue: ProjectContextValue = {
        // Data
        global,
        formData,
        
        // Loading states
        isLoading,
        isSaving,
        
        // Errors
        error: combinedError,
        hookError,
        
        // Actions
        updateFormData,
        resetForm,

        // Undo/Redo state and actions
        canUndo,
        canRedo,
        undo,
        redo,
        
        // Refetch capability
        refetch
    };

    // If there's a hook error, render an error message
    if (hookError) {
        return (
            <Alert color="red" title="Error initializing project data">
                <p>{hookError.message}</p>
                <p>Please try refreshing the page.</p>
            </Alert>
        );
    }

    return (
        <ProjectContext.Provider value={contextValue}>
            {children}
        </ProjectContext.Provider>
    );
}

/**
 * Custom hook to use the ProjectContext
 * @returns The ProjectContext value
 * @throws Error if used outside of a ProjectProvider
 */
export function useProjectContext() {
    const context = useContext(ProjectContext);

    if (context === undefined) {
        throw new Error('useProjectContext must be used within a ProjectProvider');
    }

    return context;
}