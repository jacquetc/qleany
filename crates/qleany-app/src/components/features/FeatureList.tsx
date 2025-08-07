import { ReactNode } from 'react';
import { ActionIcon, Group, Title, Alert } from '@mantine/core';
import ReorderableList from '../ReorderableList.tsx';
import { useFeatureContext } from '@/contexts/FeatureContext';
import { FeatureDTO } from '@/services/feature-service';
import ErrorBoundary from '@/components/ErrorBoundary';

const FeatureList = () => {
    // Use the feature context instead of the model
    const {
        features,
        selectedFeatureId,
        isLoadingFeatures,
        featureError,
        selectFeature,
        createFeature,
        reorderFeatures
    } = useFeatureContext();

    // Create header component for ReorderableList
    const header = (
        <Group>
            <Title order={2} id="featuresListHeading">Features</Title>
            <ActionIcon
                variant="filled"
                aria-label="Add new feature"
                onClick={createFeature}
            >
                +
            </ActionIcon>
        </Group>
    );

    // Define renderItemContent function for feature items
    const renderFeatureContent = (feature: FeatureDTO): ReactNode => (
        <div>
            <div>{feature.name}</div>
            <div style={{color: 'dimmed', fontSize: 'small'}}>
                {feature.use_cases.length > 0 ? `${feature.use_cases.length} use cases` : 'No use cases'}
            </div>
        </div>
    );

    // Custom fallback component for error state
    const errorFallback = (
        <Alert color="yellow" title="Features could not be loaded">
            There was an issue loading the feature list. Please try again later.
        </Alert>
    );

    // Loading state
    if (isLoadingFeatures) {
        return (
            <Alert color="blue" title="Loading features">
                Please wait...
            </Alert>
        );
    }

    // Error state
    if (featureError) {
        return (
            <Alert color="red" title="Error loading features">
                {featureError instanceof Error ? featureError.message : 'An unknown error occurred'}
            </Alert>
        );
    }

    return (
        <ErrorBoundary fallback={errorFallback}>
            <ReorderableList
                items={features}
                selectedItemId={selectedFeatureId}
                onSelectItem={selectFeature}
                onReorder={reorderFeatures}
                getItemId={(feature) => feature.id}
                renderItemContent={renderFeatureContent}
                droppableId="features-list"
                draggableIdPrefix="feature"
                itemType="feature"
                header={header}
            />
        </ErrorBoundary>
    );
};

export default FeatureList;