import {ReactNode} from 'react';
import {FeatureDto} from "../../controller/feature_controller.ts";
import {ActionIcon, Group, Title} from '@mantine/core';
import {error, info} from '@tauri-apps/plugin-log';
import {RootRelationshipField, setRootRelationship} from "../../controller/root_controller.ts";
import ReorderableList from '../ReorderableList.tsx';

interface FeatureListProps {
    features: FeatureDto[];
    selectedFeature: number | null;
    onSelectFeature: (featureId: number) => void;
    onCreateFeature: () => void;
    onFeaturesReordered: () => void;
}

const FeatureList = ({
                         features,
                         selectedFeature,
                         onSelectFeature,
                         onCreateFeature,
                         onFeaturesReordered
                     }: FeatureListProps) => {

    // Create header component for ReorderableList
    const header = (
        <Group>
            <Title order={2} id="featuresListHeading">Features</Title>
            <ActionIcon
                variant="filled"
                aria-label="Add new feature"
                onClick={onCreateFeature}
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

    // Define onReorder function to handle reordering
    const handleReorder = async (reorderedIds: number[]): Promise<void> => {
        try {
            // Update the root relationship with the new order
            await setRootRelationship({
                id: 1,
                field: RootRelationshipField.Features,
                right_ids: reorderedIds,
            });

            info("Feature order updated successfully");
            onFeaturesReordered();
        } catch (err) {
            error(`Failed to update feature order: ${err}`);
        }
    };

    return (
        <ReorderableList
            items={features}
            selectedItemId={selectedFeature}
            onSelectItem={onSelectFeature}
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