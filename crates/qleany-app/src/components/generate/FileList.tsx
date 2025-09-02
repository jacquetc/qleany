import {useCallback} from 'react';
import {Alert, Box, Text, Title} from '@mantine/core';
import {error} from '@tauri-apps/plugin-log';
import CheckableList from '../CheckableList';
import {FileDTO} from '@/services/file-service';
import ErrorBoundary from '../ErrorBoundary';
import {useFileContext} from '@/contexts/FileContext';

interface FileListProps {
    rootId: number | null;
}

const FileList = ({rootId}: FileListProps) => {
    // Use the FileContext instead of directly using the RootFilesListModel
    const {
        files,
        selectedFileId,
        checkedFileIds,
        selectedGroup,
        selectFile,
        checkFile,
        isLoadingFiles,
        fileError
    } = useFileContext();

    // Wrapped selectFile with logging
    const selectFileWithLogging = useCallback((fileId: number | null) => {
        error(`FileList.selectFile: User clicked file with ID ${fileId}`);
        selectFile(fileId);
    }, [selectFile]);

    // Header component for the list
    const header = (
        <Box mb={10}>
            <Title order={4}>
                {selectedGroup ? `Files in ${selectedGroup}` : 'Select a group to view files'}
            </Title>
        </Box>
    );

    // Custom filter function for files
    const filterFile = useCallback((file: FileDTO, query: string) => {
        const searchLower = query.toLowerCase();
        const nameLower = file.name.toLowerCase();
        const pathLower = file.relative_path.toLowerCase();

        return nameLower.includes(searchLower) || pathLower.includes(searchLower);
    }, []);

    // Sort files by name
    const sortFiles = useCallback((a: FileDTO, b: FileDTO) => {
        const fileNameA = a.relative_path + a.name;
        const fileNameB = b.relative_path + b.name;
        return fileNameA.localeCompare(fileNameB);
    }, []);

    // Custom fallback component for error state
    const errorFallback = (
        <Alert color="yellow" title="Files could not be loaded">
            There was an issue loading the file list. Please try again later.
        </Alert>
    );

    // Loading state
    if (isLoadingFiles) {
        return (
            <Alert color="blue" title="Loading files">
                Please wait...
            </Alert>
        );
    }

    // Error state
    if (fileError) {
        return (
            <Alert color="red" title="Error loading files">
                {fileError instanceof Error ? fileError.message : 'An unknown error occurred'}
            </Alert>
        );
    }

    return (
        <ErrorBoundary fallback={errorFallback}>
            <CheckableList
                items={files}
                selectedItemId={selectedFileId}
                checkedItemIds={checkedFileIds}
                onSelectItem={selectFileWithLogging}
                onCheckItem={checkFile}
                getItemId={(item) => item.id}
                renderItemContent={(item) => (
                    <Text 
                        style={{ 
                            direction: 'rtl',
                            textAlign: 'left',
                            overflow: 'hidden',
                            whiteSpace: 'nowrap',
                            textOverflow: 'ellipsis'
                        }}
                        title={`${item.relative_path}${item.name}`}
                    >
                        <span style={{ direction: 'ltr' }}>
                            {item.relative_path}<strong>{item.name}</strong>
                        </span>
                    </Text>
                )}
                sortItems={sortFiles}
                filterItem={filterFile}
                itemType="file"
                header={header}
            />
        </ErrorBoundary>
    );
};

export default FileList;
