import {useEffect, useState} from 'react';
import {listen} from "@tauri-apps/api/event";
import {error, info} from '@tauri-apps/plugin-log';
import {createFile, FileDto, getFileMulti} from "@/controller/file-controller.ts";
import {getRootRelationship, RootRelationshipField, setRootRelationship} from "@/controller/root-controller.ts";
import {beginComposite, endComposite} from "#controller/undo-redo-controller.ts";
import {useDebouncedCallback} from "@mantine/hooks";

export interface RootFilesListModelProps {
    rootId: number | null;
    onFilesChanged: (files: FileDto[]) => void;
    groupFilter?: string;
}

export function useRootFilesListModel(
    {
        rootId,
        onFilesChanged,
        groupFilter
    }
    : RootFilesListModelProps) {
    const [files, setFiles] = useState<FileDto[]>([]);
    const [currentGroupFilter, setCurrentGroupFilter] = useState<string | undefined>(groupFilter);

    // Function to fetch file data from the backend
    async function fetchFileData() {
        if (!rootId) {
            await error("No root selected");
            return [];
        }
        const fileIds = await getRootRelationship(rootId, RootRelationshipField.Files);
        const files = await getFileMulti(fileIds);
        let filteredFiles = files.filter((file) => file !== null) as FileDto[];

        // Apply group filter if it exists
        if (currentGroupFilter) {
            filteredFiles = filteredFiles.filter(file => file.group === currentGroupFilter);
        }

        setFiles(filteredFiles);
        onFilesChanged(filteredFiles);

        return filteredFiles;
    }

    async function createNewFile() {
        try {
            // Create file
            const dto = {
                name: 'New File',
                group: "New Group",
            };
            await beginComposite()
            const newFile = await createFile(dto);

            if (!rootId) {
                return;
            }

            // Update file relationship
            const rootFiles = await getRootRelationship(rootId, RootRelationshipField.Files) || [];

            await setRootRelationship({
                id: rootId,
                field: RootRelationshipField.Files,
                right_ids: [...rootFiles, newFile.id],
            });
            await endComposite();

            // Update files state
            const updatedFiles = [...files, newFile];
            setFiles(updatedFiles);
            onFilesChanged(updatedFiles);

            await info("File created successfully");
            return newFile;
        } catch (err) {
            await error(`Failed to create file: ${err}`);
            throw err;
        }
    }

    // Function to handle reordering of files
    async function handleReorder(reorderedIds: number[]): Promise<void> {
        try {
            if (!rootId) {
                await error("No root selected for reordering files");
                return;
            }

            // Update the root relationship with the new order
            await setRootRelationship({
                id: rootId,
                field: RootRelationshipField.Files,
                right_ids: reorderedIds,
            });

            info("File order updated successfully");
            await fetchFileData();
        } catch (err) {
            error(`Failed to update file order: ${err}`);
            throw err;
        }
    }

    const rootUpdaterHandler = useDebouncedCallback(async (event) => {
            const payload = event.payload as { ids: number[] };

            if (!rootId) {
                return;
            }

            if (!payload.ids.includes(rootId)) {
                return; // Ignore updates for other entities
            }

            info(`Root updated event received: ${payload.ids}`);
            const updatedEntities = await getRootRelationship(rootId, RootRelationshipField.Files);

            // If the files relationship has changed, fetch the updated files
            const filesIds = files.map(file => file.id);

            if (JSON.stringify(updatedEntities) !== JSON.stringify(filesIds)) {
                info(`Files relationship has changed for root ${rootId}, fetching updated files`);
                await fetchFileData().catch((err) => error(err));
            } else {
                info(`Files relationship has not changed for root ${rootId}`);
            }
        }
        , 1000);

    const fileUpdaterHandler = useDebouncedCallback(async (event) => {
            const payload = event.payload as { ids: number[] };
            info(`File updated event received: ${payload.ids}`);
            const updatedFiles = await getFileMulti(payload.ids);

            for (const updatedFile of updatedFiles) {

                if (!updatedFile) {
                    info(`File not found in the current state.`);
                    continue;
                }
                const index = files.findIndex((file) => file.id === updatedFile.id);
                if (index !== -1) {
                    const updatedFilesList = [...files];
                    updatedFilesList[index] = updatedFile;
                    setFiles(updatedFilesList);
                    onFilesChanged(updatedFilesList);
                } else {
                    info(`File not found in the current state.`);
                }
            }
        }
        , 1000);

    // Setup event listeners
    useEffect(() => {
        fetchFileData().catch((err) => error(err));

        // mounting the event listeners
        const unlisten_direct_access_file_created = listen('direct_access_file_created', (event) => {
            const payload = event.payload as { ids: string[] };
            info(`File created event received: ${payload.ids}`);

            fetchFileData().catch((err) => error(err));
        });

        // Listen for file removal events
        const unlisten_direct_access_file_removed = listen('direct_access_file_removed', async (event) => {
            const payload = event.payload as { ids: number[] };
            info(`File removed event received: ${payload.ids}`);

            // Filter out the removed files from the current state
            const updatedFiles = files.filter(file => !payload.ids.includes(file.id));
            setFiles(updatedFiles);
            onFilesChanged(updatedFiles);
        });

        // Listen for file updates


        const unlisten_direct_access_file_updated = listen('direct_access_file_updated', fileUpdaterHandler);

        // listen to any root update event, filter to the current root, check if the "files" relationship has changed
        // and update the files state accordingly


        const unlisten_direct_access_root_updated = listen('direct_access_root_updated', rootUpdaterHandler);

        const unlisten_direct_access_all_reset = listen('direct_access_all_reset', () => {
            info(`Direct access all reset event received`);
            fetchFileData().then((dtos) => info(`Files data reset successfully: ${JSON.stringify(dtos)}`)
            ).catch((err) => error(err));
        });

        return () => {
            unlisten_direct_access_file_created.then(f => f());
            unlisten_direct_access_file_removed.then(f => f());
            unlisten_direct_access_file_updated.then(f => f());
            unlisten_direct_access_root_updated.then(f => f());
            unlisten_direct_access_all_reset.then(f => f());
        };
    }, [files, rootId, currentGroupFilter]);

    return {
        files,
        createNewFile,
        handleReorder,
        fetchFileData,
        currentGroupFilter,
        setGroupFilter: setCurrentGroupFilter
    };
}
