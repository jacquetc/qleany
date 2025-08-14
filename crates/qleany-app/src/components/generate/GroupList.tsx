import {useCallback, useMemo} from 'react';
import {Alert, Box, Text, Title} from '@mantine/core';
import CheckableList from '../CheckableList';
import ErrorBoundary from '../ErrorBoundary';
import {useFileContext} from '@/contexts/FileContext';
import {GroupItem} from '@/hooks/useGroups';

interface GroupListProps {
    rootId: number | null;
}

const GroupList = ({rootId}: GroupListProps) => {
    // Use the FileContext instead of directly using the RootFilesListModel
    const {
        groups,
        selectedGroup,
        checkedGroups,
        selectGroup,
        checkGroup,
        isLoadingGroups,
        groupError,
    } = useFileContext();

    // Create a list with an "All" group at the top
    const groupsWithAll = useMemo(() => {
        // Calculate total file count across all groups
        const totalFileCount = groups.reduce((total, group) => total + group.fileCount, 0);

        // Create the "All" group with id -1 to ensure it doesn't conflict with other groups
        const allGroup: GroupItem = {
            id: -1,
            name: "All",
            fileCount: totalFileCount
        };

        return [allGroup, ...groups];
    }, [groups]);

    // Header component for the list
    const header = (
        <Box mb={10}>
            <Title order={4}>Groups</Title>
        </Box>
    );

    // Custom filter function for groups
    const filterGroup = useCallback((group: { name: string }, query: string) => {
        const searchLower = query.toLowerCase();
        const nameLower = group.name.toLowerCase();

        return nameLower.includes(searchLower);
    }, []);

    // Custom fallback component for error state
    const errorFallback = (
        <Alert color="yellow" title="Groups could not be loaded">
            There was an issue loading the group list. Please try again later.
        </Alert>
    );

    // Loading state
    if (isLoadingGroups) {
        return (
            <Alert color="blue" title="Loading groups">
                Please wait...
            </Alert>
        );
    }

    // Error state
    if (groupError) {
        return (
            <Alert color="red" title="Error loading groups">
                {groupError instanceof Error ? groupError.message : 'An unknown error occurred'}
            </Alert>
        );
    }

    // Find the selected item ID
    // If selectedGroup is null, select the "All" group (id -1)
    // Otherwise, find the group with the matching name and use its ID
    const selectedItemId = selectedGroup === null
        ? -1
        : groupsWithAll.find(g => g.name === selectedGroup)?.id ?? -1;

    return (
        <ErrorBoundary fallback={errorFallback}>
            <CheckableList
                items={groupsWithAll}
                selectedItemId={selectedItemId}
                checkedItemIds={groupsWithAll
                    .filter(group => group.id !== -1 && checkedGroups.includes(group.name))
                    .map(group => group.id)}
                onSelectItem={(id) => {
                    // Find the group by its ID
                    const group = groupsWithAll.find(g => g.id === id);
                    if (group) {
                        // If "All" group is selected, pass null to selectGroup
                        if (group.id === -1) {
                            selectGroup(null);
                        } else {
                            selectGroup(group.name);
                        }
                    }
                }}
                onCheckItem={(id, checked) => {
                    // Find the group by its ID
                    const group = groupsWithAll.find(g => g.id === id);
                    // Don't allow checking/unchecking the "All" group
                    if (group && group.id !== -1) {
                        checkGroup(group.name, checked);
                    }
                }}
                getItemId={(item) => item.id}
                renderItemContent={(item) => (
                    <Text>
                        {item.name} ({item.fileCount})
                    </Text>
                )}
                filterItem={filterGroup}
                itemType="group"
                header={header}
            />
        </ErrorBoundary>
    );
};

export default GroupList;
