import { useState, useEffect } from 'react';
import { Text, Box, Title } from '@mantine/core';
import CheckableList from '../CheckableList';
import { useRootFilesListModel } from '@/models/RootFilesListModel';
import { FileDto } from '@/controller/file-controller';

interface FileListProps {
  rootId: number | null;
  selectedGroup: string | null;
  selectedFileId: number | null;
  checkedFileIds: number[];
  onSelectFile: (fileId: number) => void;
  onCheckFile: (fileId: number, checked: boolean) => void;
  onFilesChanged: (files: FileDto[]) => void;
}

const FileList = ({
  rootId,
  selectedGroup,
  selectedFileId,
  checkedFileIds,
  onSelectFile,
  onCheckFile,
  onFilesChanged
}: FileListProps) => {
  const [filteredFiles, setFilteredFiles] = useState<FileDto[]>([]);
  
  // Use the RootFilesListModel with the group filter
  const { files } = useRootFilesListModel({
    rootId,
    onFilesChanged,
    groupFilter: selectedGroup || undefined
  });

  // Update filtered files when files or selectedGroup changes
  useEffect(() => {
    setFilteredFiles(files);
  }, [files, selectedGroup]);

  // Header component for the list
  const header = (
    <Box mb={10}>
      <Title order={4}>
        {selectedGroup ? `Files in ${selectedGroup}` : 'Select a group to view files'}
      </Title>
    </Box>
  );

  return (
    <CheckableList
      items={filteredFiles}
      selectedItemId={selectedFileId}
      checkedItemIds={checkedFileIds}
      onSelectItem={(id) => onSelectFile(id)}
      onCheckItem={(id, checked) => onCheckFile(id, checked)}
      getItemId={(item) => item.id}
      renderItemContent={(item) => (
        <Text>
          {item.name}
        </Text>
      )}
      itemType="file"
      header={header}
    />
  );
};

export default FileList;