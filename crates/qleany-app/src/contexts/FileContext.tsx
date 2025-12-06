import React, {createContext, useContext, useEffect, useState} from 'react';
import {Alert} from '@mantine/core';
import {error as logError} from '@tauri-apps/plugin-log';
import {useFiles} from '../hooks/useFiles';
import {useRoot} from '../hooks/useRoot';
import {GroupItem, useGroups} from '../hooks/useGroups';
import {FileDTO} from '../services/file-service';
import {QueryObserverResult, RefetchOptions} from '@tanstack/react-query';

/**
 * Interface for the FileContext value
 */
interface FileContextValue {
    // Files data
    files: FileDTO[];
    selectedFileId: number | null;
    checkedFileIds: number[];
    isLoadingFiles: boolean;
    fileError: unknown;

    // Groups data
    groups: GroupItem[];
    selectedGroup: string | null;
    checkedGroups: string[];
    isLoadingGroups: boolean;
    groupError: unknown;

    // common data
    rootPath: string | null;

    // Hook error
    hookError: Error | null;

    // Actions
    selectFile: (fileId: number | null) => void;
    checkFile: (fileId: number, checked: boolean) => void;
    selectGroup: (groupName: string | null) => void;
    checkGroup: (groupName: string, checked: boolean) => void;
    createFile: () => void;
    refetchFiles: () => Promise<void>;
    getFilesInGroup: (groupName: string) => FileDTO[];
}

// Create the context with a default value
const FileContext = createContext<FileContextValue | undefined>(undefined);

/**
 * Props for the FileProvider component
 */
interface FileProviderProps {
    rootId: number | null;
    children: React.ReactNode;
}

/**
 * FileProvider component that provides file and group data to its children
 */
export function FileProvider({rootId, children}: FileProviderProps) {
    // Log component mount/unmount and rootId changes
    useEffect(() => {
        logError(`FileProvider: Component mounted with rootId=${rootId}`);
        return () => {
            logError(`FileProvider: Component unmounting`);
        };
    }, []);

    useEffect(() => {
        logError(`FileProvider: rootId changed to ${rootId}`);
    }, [rootId]);

    // State for selected and checked items - use a ref to persist across remounts
    const [selectedFileIdState, setSelectedFileIdState] = useState<number | null>(() => {
        // Try to restore from sessionStorage on mount
        try {
            const stored = sessionStorage.getItem(`selectedFileId_${rootId}`);
            return stored ? parseInt(stored, 10) : null;
        } catch {
            return null;
        }
    });
    const [checkedFileIds, setCheckedFileIds] = useState<number[]>([]);
    const [selectedGroup, setSelectedGroup] = useState<string | null>(null);
    const [checkedGroups, setCheckedGroups] = useState<string[]>([]);

    // Wrapped setSelectedFileId with logging
    const setSelectedFileId = (fileId: number | null) => {
        logError(`FileContext.setSelectedFileId: Changing selectedFileId from ${selectedFileIdState} to ${fileId}`);
        const stack = new Error().stack;
        logError(`FileContext.setSelectedFileId: Call stack: ${stack}`);
        setSelectedFileIdState(fileId);
    };

    const selectedFileId = selectedFileIdState;

    // Effect to persist selectedFileId to sessionStorage
    useEffect(() => {
        try {
            if (selectedFileIdState !== null) {
                sessionStorage.setItem(`selectedFileId_${rootId}`, selectedFileIdState.toString());
                logError(`FileContext: Saved selectedFileId ${selectedFileIdState} to sessionStorage`);
            } else {
                sessionStorage.removeItem(`selectedFileId_${rootId}`);
                logError(`FileContext: Removed selectedFileId from sessionStorage`);
            }
        } catch (err) {
            logError(`FileContext: Failed to save selectedFileId to sessionStorage: ${err}`);
        }
    }, [selectedFileIdState, rootId]);

    // Add state for handling hook errors
    const [hookError, setHookError] = useState<Error | null>(null);

    // Use the custom hooks for files and groups data with error handling
    let filesData: {
        createFile: () => void;
        error: Error | null;
        files: FileDTO[];
        isLoading: boolean;
        refetch: { (options?: RefetchOptions | undefined): Promise<QueryObserverResult<FileDTO[], Error>> }
    } = {
        files: [], isLoading: true, error: null, createFile: () => {
        }, refetch: async () => {
        }
    };
    let groupsData = {
        groups: [], isLoading: true, error: null, refetch: async () => {
        }, getFilesInGroup: (_: string) => []
    };

    try {
        filesData = useFiles(rootId, selectedGroup || undefined);
    } catch (err) {
        const errorMessage = `Error in useFiles hook: ${err}`;
        logError(errorMessage);
        setHookError(err instanceof Error ? err : new Error('Unknown error in useFiles hook'));
    }

    try {
        groupsData = useGroups(rootId);
    } catch (err) {
        const errorMessage = `Error in useGroups hook: ${err}`;
        logError(errorMessage);
        setHookError(err instanceof Error ? err : new Error('Unknown error in useGroups hook'));
    }
    const {
        files,
        isLoading: isLoadingFiles,
        queryError: fileError,
        createFile,
        refetch: refetchFilesData
    } = filesData;

    const {
        groups,
        isLoading: isLoadingGroups,
        error: groupError,
        getFilesInGroup
    } = groupsData;

    // Resolve rootPath from the current root entity
    const { root } = useRoot(rootId);
    const rootPath = root?.manifest_absolute_path ?? null;


    // Effect to validate selectedFileId when files change
    useEffect(() => {
        logError(`FileContext validation effect: selectedFileId=${selectedFileId}, selectedGroup=${selectedGroup}, files.length=${files.length}`);

        // Only validate and clear selection when:
        // 1. No group is selected (showing all files), AND
        // 2. Files have been loaded (not empty), AND
        // 3. The selected file doesn't exist in the complete files list
        if (selectedFileId !== null && !selectedGroup && files.length > 0) {
            const selectedFileExists = files.some(file => file.id === selectedFileId);
            logError(`FileContext validation: selectedFileExists=${selectedFileExists} for fileId=${selectedFileId}`);

            if (!selectedFileExists) {
                // Clear the selection if the file no longer exists in the complete list
                logError(`FileContext validation: Clearing selectedFileId ${selectedFileId} because file not found in files list`);
                setSelectedFileId(null);
            }
        }
        // When a group is selected, don't validate against the filtered list
        // Let the user keep their selected file even if it's not in the current group
    }, [files, selectedFileId, selectedGroup]);

    // Function to select a file
    const selectFile = (fileId: number | null) => {
        logError(`FileContext.selectFile: Setting selectedFileId to ${fileId}`);
        logError(`FileContext.selectFile: Current selectedFileId before change: ${selectedFileId}`);
        setSelectedFileId(fileId);
    };

    // Function to check/uncheck a file
    const checkFile = (fileId: number, checked: boolean) => {
        if (checked) {
            // Add file to checked files
            setCheckedFileIds(prev => [...prev, fileId]);

            // Check if all files in the group are now checked
            const file = files.find(f => f.id === fileId);
            if (file && file.group) {
                const filesInGroup = files.filter(f => f.group === file.group);
                const fileIdsInGroup = filesInGroup.map(f => f.id);
                const allChecked = fileIdsInGroup.every(id =>
                    checkedFileIds.includes(id) || id === fileId
                );

                if (allChecked && !checkedGroups.includes(file.group)) {
                    setCheckedGroups(prev => [...prev, file.group]);
                }
            }
        } else {
            // Remove file from checked files
            setCheckedFileIds(prev => prev.filter(id => id !== fileId));

            // Uncheck the group if any file in the group is unchecked
            const file = files.find(f => f.id === fileId);
            if (file && file.group && checkedGroups.includes(file.group)) {
                setCheckedGroups(prev => prev.filter(g => g !== file.group));
            }
        }
    };

    // Function to select a group
    const selectGroup = (groupName: string | null) => {
        setSelectedGroup(groupName);
        // Don't clear selected file when selecting a group - let users keep their code displayed
    };

    // Function to check/uncheck a group
    const checkGroup = (groupName: string, checked: boolean) => {
        if (checked) {
            // Add group to checked groups
            setCheckedGroups(prev => [...prev, groupName]);

            // Check all files in this group
            const filesInGroup = files.filter(file => file.group === groupName);
            const fileIdsInGroup = filesInGroup.map(file => file.id);
            setCheckedFileIds(prev => [...prev, ...fileIdsInGroup.filter(id => !prev.includes(id))]);
        } else {
            // Remove group from checked groups
            setCheckedGroups(prev => prev.filter(g => g !== groupName));

            // Uncheck all files in this group
            const filesInGroup = files.filter(file => file.group === groupName);
            const fileIdsInGroup = filesInGroup.map(file => file.id);
            setCheckedFileIds(prev => prev.filter(id => !fileIdsInGroup.includes(id)));
        }
    };

    // Function to refetch files data
    const refetchFiles = async () => {
        await refetchFilesData();
    };

    // Create the context value
    const contextValue: FileContextValue = {
        // Files data
        files,
        selectedFileId,
        checkedFileIds,
        isLoadingFiles,
        fileError,

        // Groups data
        groups,
        selectedGroup,
        checkedGroups,
        isLoadingGroups,
        groupError,

        // Common data
        rootPath,

        // Hook error
        hookError,

        // Actions
        selectFile,
        checkFile,
        selectGroup,
        checkGroup,
        createFile,
        refetchFiles,
        getFilesInGroup
    };

    // If there's a hook error, render an error message
    if (hookError) {
        return (
            <Alert color="red" title="Error initializing file data">
                <p>{hookError.message}</p>
                <p>Please try refreshing the page.</p>
            </Alert>
        );
    }

    return (
        <FileContext.Provider value={contextValue}>
            {children}
        </FileContext.Provider>
    );
}

/**
 * Custom hook to use the FileContext
 * @returns The FileContext value
 * @throws Error if used outside of a FileProvider
 */
export function useFileContext() {
    const context = useContext(FileContext);

    if (context === undefined) {
        throw new Error('useFileContext must be used within a FileProvider');
    }

    return context;
}