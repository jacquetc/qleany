import {useCallback} from 'react';
import {Alert, Box, Text, Title} from '@mantine/core';
import CheckableList from '../CheckableList';
import ErrorBoundary from '../ErrorBoundary';
import {useFileContext} from '@/contexts/FileContext';

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
        groupError
    } = useFileContext();

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

    return (
        <ErrorBoundary fallback={errorFallback}>
            <CheckableList
                items={groups}
                selectedItemId={selectedGroup ? groups.findIndex(g => g.name === selectedGroup) : null}
                checkedItemIds={groups
                    .filter(group => checkedGroups.includes(group.name))
                    .map(group => group.id)}
                onSelectItem={(id) => {
                    const group = groups[id];
                    if (group) {
                        selectGroup(group.name);
                    }
                }}
                onCheckItem={(id, checked) => {
                    const group = groups[id];
                    if (group) {
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
