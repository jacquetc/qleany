import {useMemo} from 'react';
import {useFiles} from './useFiles';
import {FileDTO} from '../services/file-service';
import {error} from '@tauri-apps/plugin-log';

/**
 * Group item interface
 */
export interface GroupItem {
    id: number;
    name: string;
    fileCount: number;
}

/**
 * Custom hook for extracting and managing groups data from files
 *
 * This hook uses the useFiles hook to get the files data and then
 * extracts and organizes the group information.
 *
 * @param rootId The ID of the root entity
 */
export function useGroups(rootId: number | null) {
    // Use the useFiles hook to get all files (without group filter)
    const {files, isLoading, queryError, refetch} = useFiles(rootId);

    // Extract groups from files data
    const groups = useMemo(() => {
        if (!files || files.length === 0) {
            return [];
        }

        try {
            // Create a map to count files per group
            const groupMap = new Map<string, number>();

            // Count files per group
            files.forEach(file => {
                if (file && file.group) {
                    const count = groupMap.get(file.group) || 0;
                    groupMap.set(file.group, count + 1);
                }
            });

            // Convert map to array of GroupItem objects
            const groupItems: GroupItem[] = Array.from(groupMap.entries())
                .map(([name, fileCount], index) => ({
                    id: index,
                    name,
                    fileCount
                }));

            return groupItems;
        } catch (err) {
            error(`Error processing files for groups: ${err}`);
            return [];
        }
    }, [files]);

    /**
     * Get all files in a specific group
     * @param groupName The name of the group
     * @returns Array of files in the group
     */
    const getFilesInGroup = (groupName: string): FileDTO[] => {
        try {
            if (!files || files.length === 0) {
                return [];
            }

            return files.filter(file => file.group === groupName);
        } catch (err) {
            error(`Error getting files in group ${groupName}: ${err}`);
            return [];
        }
    };

    return {
        groups,
        isLoading,
        error: queryError,
        refetch,
        getFilesInGroup
    };
}