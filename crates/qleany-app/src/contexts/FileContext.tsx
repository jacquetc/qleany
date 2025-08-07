import React, {createContext, useContext, useState} from 'react';
import {Alert} from '@mantine/core';
import {error as logError} from '@tauri-apps/plugin-log';
import {useFiles} from '../hooks/useFiles';
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
    // State for selected and checked items
    const [selectedFileId, setSelectedFileId] = useState<number | null>(null);
    const [checkedFileIds, setCheckedFileIds] = useState<number[]>([]);
    const [selectedGroup, setSelectedGroup] = useState<string | null>(null);
    const [checkedGroups, setCheckedGroups] = useState<string[]>([]);

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

    // Function to select a file
    const selectFile = (fileId: number | null) => {
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
        setSelectedFileId(null);
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