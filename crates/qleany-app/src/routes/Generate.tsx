import {useEffect, useState} from 'react';
import {Grid, Title} from '@mantine/core';
import {error, info} from '@tauri-apps/plugin-log';
import {getRootMulti} from '#controller/root-controller.ts';
import {listRustFiles, ListRustFilesDto} from '#controller/rust-file-generation-controller.ts';
import {FileDto} from '@/controller/file-controller';
import GroupList from '@/components/generate/GroupList';
import FileList from '@/components/generate/FileList';

const Generate = () => {
    const [rootId, setRootId] = useState<number | null>(null);
    const [selectedGroup, setSelectedGroup] = useState<string | null>(null);
    const [checkedGroups, setCheckedGroups] = useState<string[]>([]);
    const [selectedFileId, setSelectedFileId] = useState<number | null>(null);
    const [checkedFileIds, setCheckedFileIds] = useState<number[]>([]);
    const [allFiles, setAllFiles] = useState<FileDto[]>([]);

    // Function to get the root ID
    async function getRootId() {
        const roots = await getRootMulti([]);
        info(`Root ID initialized: JSON.stringify(roots)`);
        if (roots.length > 0 && roots[0] !== null) {
            setRootId(roots[0]!.id);
            return roots[0]!.id;
        }
        return null;
    }

    // Initialize root ID on component mount
    useEffect(() => {
        getRootId().catch((err) => error(err));

        const dto: ListRustFilesDto = {
            only_existing: false,
        }

        listRustFiles(dto).catch((err) => error(err));
    }, []);

    // Handle file changes
    const handleFilesChanged = (files: FileDto[]) => {
        setAllFiles(files);
    };

    // Handle group selection
    const handleSelectGroup = (group: string) => {
        setSelectedGroup(group);
        setSelectedFileId(null);
    };

    // Handle group checking/unchecking
    const handleCheckGroup = (group: string, checked: boolean) => {
        if (checked) {
            // Add group to checked groups
            setCheckedGroups(prev => [...prev, group]);

            // Check all files in this group
            const filesInGroup = allFiles.filter(file => file.group === group);
            const fileIdsInGroup = filesInGroup.map(file => file.id);
            setCheckedFileIds(prev => [...prev, ...fileIdsInGroup.filter(id => !prev.includes(id))]);
        } else {
            // Remove group from checked groups
            setCheckedGroups(prev => prev.filter(g => g !== group));

            // Uncheck all files in this group
            const filesInGroup = allFiles.filter(file => file.group === group);
            const fileIdsInGroup = filesInGroup.map(file => file.id);
            setCheckedFileIds(prev => prev.filter(id => !fileIdsInGroup.includes(id)));
        }
    };

    // Handle file selection
    const handleSelectFile = (fileId: number) => {
        setSelectedFileId(fileId);
    };

    // Handle file checking/unchecking
    const handleCheckFile = (fileId: number, checked: boolean) => {
        if (checked) {
            // Add file to checked files
            setCheckedFileIds(prev => [...prev, fileId]);

            // Check if all files in the group are now checked
            const file = allFiles.find(f => f.id === fileId);
            if (file && file.group) {
                const filesInGroup = allFiles.filter(f => f.group === file.group);
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
            const file = allFiles.find(f => f.id === fileId);
            if (file && file.group && checkedGroups.includes(file.group)) {
                setCheckedGroups(prev => prev.filter(g => g !== file.group));
            }
        }
    };

    return (
        <div className="p-10">
            <Title order={1} mb="xl">Generate</Title>

            <Grid>
                <Grid.Col span={4}>
                    <GroupList
                        rootId={rootId}
                        selectedGroup={selectedGroup}
                        checkedGroups={checkedGroups}
                        onSelectGroup={handleSelectGroup}
                        onCheckGroup={handleCheckGroup}
                        onFilesChanged={handleFilesChanged}
                    />
                </Grid.Col>
                <Grid.Col span={8}>
                    <FileList
                        rootId={rootId}
                        selectedGroup={selectedGroup}
                        selectedFileId={selectedFileId}
                        checkedFileIds={checkedFileIds}
                        onSelectFile={handleSelectFile}
                        onCheckFile={handleCheckFile}
                        onFilesChanged={handleFilesChanged}
                    />
                </Grid.Col>
            </Grid>
        </div>
    );
}

export default Generate;
