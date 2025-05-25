import {ReactNode, useState} from 'react';
import {FeatureDto} from "#controller/feature-controller.ts";
import {ActionIcon, Group, Title} from '@mantine/core';
import ReorderableList from '../ReorderableList.tsx';
import {useFeatureListModel} from "#models/RootFeaturesListModel.ts";

interface FeatureListProps {
    onSelectFeature: (featureId: number) => void;
}

const FeatureList = ({
                         onSelectFeature
                     }: FeatureListProps) => {
    const [selectedFeature, setSelectedFeature] = useState<number | null>(null);

    // Use the entity list model
    const {features, createNewFeature, handleReorder} = useFeatureListModel({
        onFeaturesChanged: (__newFeatures) => {
            // If needed, you can perform additional actions when entities change
        }

    });

    // Create header component for ReorderableList
    const header = (
        <Group>
            <Title order={2} id="featuresListHeading">Features</Title>
            <ActionIcon
                variant="filled"
                aria-label="Add new feature"
                onClick={createNewFeature}
            >
                +
            </ActionIcon>
        </Group>
    );

    // Define renderItemContent function for feature items
    const renderFeatureContent = (feature: FeatureDto): ReactNode => (
        <div>
            <div>{feature.name}</div>
            <div style={{color: 'dimmed', fontSize: 'small'}}>
                {feature.use_cases.length > 0 ? `${feature.use_cases.length} use cases` : 'No use cases'}
            </div>
        </div>
    );

    // Handle entity selection
    const handleSelectFeature = (featureId: number) => {
        setSelectedFeature(featureId);
        onSelectFeature(featureId);
    };

    return (
        <ReorderableList
            items={features}
            selectedItemId={selectedFeature}
            onSelectItem={handleSelectFeature}
            onReorder={handleReorder}
            getItemId={(feature) => feature.id}
            renderItemContent={renderFeatureContent}
            droppableId="features-list"
            draggableIdPrefix="feature"
            itemType="feature"
            header={header}
        />
    );
};

export default FeatureList;