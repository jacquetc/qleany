import {memo, useEffect, useState} from 'react';
import {Box, Paper, Stack, Text, Title} from '@mantine/core';
import {getRootMulti, getRootRelationship, RootRelationshipField} from "../controller/root_controller";
import {EntityDto, getEntityMulti} from "../controller/entity_controller";
import {FieldDto, FieldType, getFieldMulti} from "../controller/field_controller";
import {error} from '@tauri-apps/plugin-log';
import ReactFlow, {
    Background,
    Controls,
    Edge,
    Handle,
    MiniMap,
    Node,
    Position,
    ReactFlowProvider,
    useEdgesState,
    useNodesState
} from 'reactflow';
import 'reactflow/dist/style.css';
import ELK from 'elkjs/lib/elk.bundled.js';

interface EntityWithFields extends EntityDto {
    fieldObjects: FieldDto[];
}

// Custom node component for displaying entity details
const EntityNode = memo(({data}: { data: { entity: EntityWithFields, allEntities: EntityWithFields[] } }) => {
    const {entity, allEntities} = data;

    return (
        <div style={{padding: 0}}>
            <Handle type="target" position={Position.Top}/>

            <Paper shadow="xs" p="md" withBorder style={{minWidth: 200}}>
                <Stack gap="xs">
                    <Title order={4} ta="center" style={{borderBottom: '1px solid #ccc', paddingBottom: '8px'}}>
                        {entity.name}
                    </Title>

                    {/* Fields */}
                    <Stack gap="xs" style={{
                        borderBottom: entity.fieldObjects.length > 0 ? '1px solid #ccc' : 'none',
                        paddingBottom: '8px'
                    }}>
                        {entity.fieldObjects.map(field => (
                            <Box key={field.id} px="sm">
                                <Text size="sm">
                                    {field.name}: {field.field_type}
                                    {field.is_primary_key ? ' (PK)' : ''}
                                    {field.is_nullable ? ' (nullable)' : ''}
                                </Text>
                            </Box>
                        ))}
                    </Stack>

                    {/* Relationships */}
                    {entity.relationships.length > 0 && (
                        <Stack gap="xs">
                            {entity.relationships.map(relationshipId => {
                                // Find the field that represents this relationship
                                const relationshipField = allEntities
                                    .flatMap(e => e.fieldObjects)
                                    .find(f => f.id === relationshipId);

                                if (!relationshipField) return null;

                                return (
                                    <Text key={relationshipId} size="sm" px="sm">
                                        {relationshipField.name} → {relationshipField.entity ?
                                        allEntities.find(e => e.id === relationshipField.entity)?.name || 'Unknown'
                                        : 'Unknown'}
                                        {relationshipField.strong ? ' (strong)' : ''}
                                    </Text>
                                );
                            })}
                        </Stack>
                    )}
                </Stack>
            </Paper>

            <Handle type="source" position={Position.Bottom}/>
        </div>
    );
});

// Define nodeTypes outside the component to prevent recreation on each render
const nodeTypes = {
    entityNode: EntityNode
};

// Create an ELK instance
const elk = new ELK();

// Main component for the entity map
const EntityMapFlow = () => {
    const [_, setEntities] = useState<EntityWithFields[]>([]);
    const [nodes, setNodes, onNodesChange] = useNodesState([]);
    const [edges, setEdges, onEdgesChange] = useEdgesState([]);
    const [loading, setLoading] = useState(false);
    const [rootId, setRootId] = useState<number>(1);

    // Function to get the root ID
    async function getRootId() {
        const roots = await getRootMulti([]);
        if (roots.length > 0 && roots[0] !== null) {
            setRootId(roots[0]!.id);
            return roots[0]!.id;
        }
        return 1; // Fallback to default
    }

    // Function to fetch entity data from the backend
    async function fetchEntityData() {
        try {
            setLoading(true);

            // Get all entity IDs from the root
            const currentRootId = await getRootId();
            const entityIds = await getRootRelationship(currentRootId, RootRelationshipField.Entities);

            // Fetch all entities
            const entitiesData = await getEntityMulti(entityIds);
            const validEntities = entitiesData.filter((entity): entity is EntityDto => entity !== null);

            // For each entity, fetch its fields
            const entitiesWithFields: EntityWithFields[] = [];

            for (const entity of validEntities) {
                const fieldIds = entity.fields;
                const fieldsData = await getFieldMulti(fieldIds);
                const validFields = fieldsData.filter((field): field is FieldDto => field !== null);

                entitiesWithFields.push({
                    ...entity,
                    fieldObjects: validFields
                });
            }

            // Find the root entity (en
            setEntities(entitiesWithFields);

            // Create nodes and edges for ReactFlow using ELK for layout
            await createNodesAndEdges(entitiesWithFields);
        } catch (err) {
            error(`Failed to fetch entity data: ${err}`);
        } finally {
            setLoading(false);
        }
    }

    // Create nodes and edges for ReactFlow using ELK for layout
    const createNodesAndEdges = async (entities: EntityWithFields[]) => {
        if (!entities.length) return;

        setLoading(true);

        try {
            const newNodes: Node[] = [];
            const newEdges: Edge[] = [];

            // Separate heritage entities from regular entities
            const heritageEntities = entities.filter(e => e.only_for_heritage);
            const regularEntities = entities.filter(e => !e.only_for_heritage);

            // Create initial nodes for all entities (without positions yet)
            const elkNodes = regularEntities.map(entity => ({
                id: `entity-${entity.id}`,
                width: 250,  // Approximate width of the node
                height: 200, // Approximate height of the node
            }));

            // Create edges for parent-child relationships
            const parentChildEdges: any[] = [];
            regularEntities.forEach(entity => {
                if (entity.parent !== null) {
                    const parentEntity = entities.find(e => e.id === entity.parent);

                    // Only create edge if parent is not a heritage entity
                    if (parentEntity && !parentEntity.only_for_heritage) {
                        parentChildEdges.push({
                            id: `edge-parent-${entity.parent}-${entity.id}`,
                            source: `entity-${entity.parent}`,
                            target: `entity-${entity.id}`,
                            type: 'straight'
                        });
                    }
                }
            });

            // Create edges for strong entity relationships
            const relationshipEdges: any[] = [];
            regularEntities.forEach(entity => {
                // Check all fields of the entity, not just those in the relationships array
                entity.fieldObjects.forEach(field => {
                    if (field.field_type === FieldType.Entity && field.strong && field.entity) {
                        const targetEntityId = field.entity;
                        const targetEntity = entities.find(e => e.id === targetEntityId);

                        // Only create edge if target is not a heritage entity
                        if (targetEntity && !targetEntity.only_for_heritage) {
                            relationshipEdges.push({
                                id: `edge-rel-${entity.id}-${field.id}-${targetEntityId}`,
                                source: `entity-${entity.id}`,
                                target: `entity-${targetEntityId}`,
                                type: 'straight',
                                label: field.name
                            });
                        }
                    }
                });
            });

            // Combine all edges for ELK
            const elkEdges = [...parentChildEdges, ...relationshipEdges];

            // Prepare the graph for ELK layout
            const elkGraph = {
                id: 'root',
                layoutOptions: {
                    'elk.algorithm': 'layered',
                    'elk.direction': 'DOWN',
                    'elk.spacing.nodeNode': '80',
                    'elk.layered.spacing.nodeNodeBetweenLayers': '100',
                    'elk.edgeRouting': 'ORTHOGONAL'
                },
                children: elkNodes,
                edges: elkEdges
            };

            // Apply ELK layout
            const layoutedGraph = await elk.layout(elkGraph);

            // Create a map to store positions from ELK
            const nodePositions = new Map<string, { x: number, y: number }>();

            // Extract positions from ELK layout
            if (layoutedGraph.children) {
                layoutedGraph.children.forEach(node => {
                    if (node.x !== undefined && node.y !== undefined) {
                        nodePositions.set(node.id, {x: node.x, y: node.y});
                    }
                });
            }

            // Create ReactFlow nodes with positions from ELK
            regularEntities.forEach(entity => {
                const nodeId = `entity-${entity.id}`;
                const position = nodePositions.get(nodeId) || {x: 0, y: 0};

                newNodes.push({
                    id: nodeId,
                    type: 'entityNode',
                    position,
                    data: {
                        entity,
                        allEntities: entities
                    }
                });
            });

            // Position heritage entities in the bottom-right corner
            // Find the maximum x and y coordinates from the regular entities
            let maxX = 0;
            let maxY = 0;

            newNodes.forEach(node => {
                maxX = Math.max(maxX, node.position.x);
                maxY = Math.max(maxY, node.position.y);
            });

            // Add heritage entities to the bottom-right corner
            heritageEntities.forEach((entity, index) => {
                newNodes.push({
                    id: `entity-${entity.id}`,
                    type: 'entityNode',
                    position: {
                        x: maxX + 400, // Position to the right of the regular entities
                        y: 100 + (index * 250) // Stack them vertically with some spacing
                    },
                    data: {
                        entity,
                        allEntities: entities
                    }
                });
            });

            // Create ReactFlow edges
            // Parent-child edges
            regularEntities.forEach(entity => {
                if (entity.parent !== null) {
                    const parentEntity = entities.find(e => e.id === entity.parent);

                    // Only create edge if parent is not a heritage entity
                    if (parentEntity && !parentEntity.only_for_heritage) {
                        newEdges.push({
                            id: `edge-parent-${entity.parent}-${entity.id}`,
                            source: `entity-${entity.parent}`,
                            target: `entity-${entity.id}`,
                            type: 'smoothstep',
                            animated: false,
                            style: {stroke: '#555'}
                        });
                    }
                }
            });

            // Strong relationship edges
            regularEntities.forEach(entity => {
                // Check all fields of the entity, not just those in the relationships array
                entity.fieldObjects.forEach(field => {
                    if (field.field_type === FieldType.Entity && field.strong && field.entity) {
                        const targetEntityId = field.entity;
                        const targetEntity = entities.find(e => e.id === targetEntityId);

                        // Only create edge if target is not a heritage entity
                        if (targetEntity && !targetEntity.only_for_heritage) {
                            newEdges.push({
                                id: `edge-rel-${entity.id}-${field.id}-${targetEntityId}`,
                                source: `entity-${entity.id}`,
                                target: `entity-${targetEntityId}`,
                                type: 'smoothstep',
                                animated: true,
                                style: {
                                    stroke: '#ff0000'
                                },
                                label: field.name
                            });
                        }
                    }
                });
            });

            setNodes(newNodes);
            setEdges(newEdges);
        } catch (err) {
            error(`Failed to layout diagram: ${err}`);
        } finally {
            setLoading(false);
        }
    };

    useEffect(() => {
        fetchEntityData();
    }, []);

    return (
        <div style={{width: '100%', height: '80vh', position: 'relative'}}>
            {loading && (
                <div style={{
                    position: 'absolute',
                    top: 0,
                    left: 0,
                    right: 0,
                    bottom: 0,
                    backgroundColor: 'rgba(255, 255, 255, 0.7)',
                    display: 'flex',
                    justifyContent: 'center',
                    alignItems: 'center',
                    zIndex: 10
                }}>
                    <div>
                        <Text size="xl" fw={700}>Calculating layout...</Text>
                    </div>
                </div>
            )}
            <ReactFlow
                nodes={nodes}
                edges={edges}
                onNodesChange={onNodesChange}
                onEdgesChange={onEdgesChange}
                nodeTypes={nodeTypes}
                fitView
                attributionPosition="bottom-left"
            >
                <Controls/>
                <MiniMap/>
                <Background gap={12} size={1}/>
            </ReactFlow>
        </div>
    );
};

// Wrapper component with ReactFlowProvider
const EntityMap = () => {
    return (
        <div className="p-10">
            <Title order={1} mb="xl">Entity Relationship Diagram</Title>
            <ReactFlowProvider>
                <EntityMapFlow/>
            </ReactFlowProvider>
        </div>
    );
}

export default EntityMap;
