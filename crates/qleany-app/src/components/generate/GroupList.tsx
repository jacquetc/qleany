import { useState, useEffect } from 'react';
import { Text, Box, Title } from '@mantine/core';
import CheckableList from '../CheckableList';
import { useRootFilesListModel } from '@/models/RootFilesListModel';
import { FileDto } from '@/controller/file-controller';

interface GroupListProps {
  rootId: number | null;
  selectedGroup: string | null;
  checkedGroups: string[];
  onSelectGroup: (group: string) => void;
  onCheckGroup: (group: string, checked: boolean) => void;
  onFilesChanged: (files: FileDto[]) => void;
}

interface GroupItem {
  id: number;
  name: string;
  fileCount: number;
}

const GroupList = ({
  rootId,
  selectedGroup,
  checkedGroups,
  onSelectGroup,
  onCheckGroup,
  onFilesChanged
}: GroupListProps) => {
  const [groups, setGroups] = useState<GroupItem[]>([]);
  
  // Use the RootFilesListModel to get all files
  const { files } = useRootFilesListModel({
    rootId,
    onFilesChanged
  });

  // Extract unique groups from files and count files per group
  useEffect(() => {
    const groupMap = new Map<string, number>();
    
    files.forEach(file => {
      if (file.group) {
        const count = groupMap.get(file.group) || 0;
        groupMap.set(file.group, count + 1);
      }
    });
    
    const groupItems: GroupItem[] = Array.from(groupMap.entries()).map(([name, fileCount], index) => ({
      id: index,
      name,
      fileCount
    }));
    
    setGroups(groupItems);
  }, [files]);

  // Header component for the list
  const header = (
    <Box mb={10}>
      <Title order={4}>Groups</Title>
    </Box>
  );

  return (
    <CheckableList
      items={groups}
      selectedItemId={selectedGroup ? groups.findIndex(g => g.name === selectedGroup) : null}
      checkedItemIds={groups
        .filter(group => checkedGroups.includes(group.name))
        .map(group => group.id)}
      onSelectItem={(id) => {
        const group = groups[id];
        if (group) {
          onSelectGroup(group.name);
        }
      }}
      onCheckItem={(id, checked) => {
        const group = groups[id];
        if (group) {
          onCheckGroup(group.name, checked);
        }
      }}
      getItemId={(item) => item.id}
      renderItemContent={(item) => (
        <Text>
          {item.name} ({item.fileCount})
        </Text>
      )}
      itemType="group"
      header={header}
    />
  );
};

export default GroupList;