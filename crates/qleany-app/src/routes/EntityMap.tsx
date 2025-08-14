import {memo, useCallback, useEffect, useMemo, useState} from 'react';
import {Alert, Box, Button, Group, Loader, Paper, Stack, Text, Title} from '@mantine/core';
import {error, error as logError, info} from '@tauri-apps/plugin-log';
import {EntityProvider, useEntityContext} from '@/contexts/EntityContext';
import {EntityDTO} from '@/services/entity-service';
import {FieldDTO, FieldType} from '@/services/field-service';
import {Direction, RelationshipDTO, Strength} from '@/services/relationship-service';
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
import ErrorBoundary from '@/components/ErrorBoundary';
import {rootService} from "#services/root-service.ts";

/**
 * Interface for entity with its fields and relationships loaded
 */
interface EntityWithFields extends EntityDTO {
    fieldObjects: FieldDTO[];
    relationshipObjects: RelationshipDTO[];
}

/**
 * Custom node component for displaying entity details
 */
const EntityNode = memo(({data}: { data: { entity: EntityWithFields, allEntities: EntityWithFields[] } }) => {
    // Defensive check to ensure data and its properties exist
    if (!data || !data.entity || !data.allEntities) {
        return (
            <div style={{padding: 0}}>
                <Paper shadow="xs" p="md" withBorder style={{minWidth: 200}}>
                    <Text color="red">Error: Invalid entity data</Text>
                </Paper>
            </div>
        );
    }

    const {entity, allEntities} = data;

    return (
        <div style={{padding: 0}}>
            {/* Input connection point */}
            <Handle type="target" position={Position.Top}/>

            <Paper shadow="xs" p="md" withBorder style={{minWidth: 200}}>
                <Stack gap="xs">
                    {/* Entity name header */}
                    <Title order={4} ta="center" style={{
                        borderBottom: '1px solid #ccc',
                        paddingBottom: '8px',
                        backgroundColor: entity.only_for_heritage ? '#f0f8ff' : '#fff8f0'
                    }}>
                        {entity.name || 'Unnamed Entity'}
                        {entity.only_for_heritage && <Text size="xs" c="dimmed">(Heritage)</Text>}
                    </Title>

                    {/* Fields section */}
                    {Array.isArray(entity.fieldObjects) && (
                        <Stack gap="xs" style={{
                            borderBottom: entity.fieldObjects.length > 0 ? '1px solid #ccc' : 'none',
                            paddingBottom: '8px'
                        }}>
                            {entity.fieldObjects.map(field => (
                                field ? (
                                    <Box key={field.id} px="sm">
                                        <Text size="sm">
                      <span style={{
                          fontWeight: field.is_primary_key ? 'bold' : 'normal',
                          color: field.field_type === FieldType.Entity ? '#0066cc' : 'inherit'
                      }}>
                        {field.name || 'Unnamed Field'}
                      </span>: {field.field_type === FieldType.Entity && field.entity ?
                                            allEntities.find(e => e && e.id === field.entity)?.name || 'Unknown Entity'
                                            : field.field_type || 'Unknown Type'}
                                            {field.is_primary_key ? ' (PK)' : ''}
                                            {field.is_nullable ? ' (nullable)' : ''}
                                            {field.is_list ? ' (list)' : ''}
                                        </Text>
                                    </Box>
                                ) : null
                            ))}
                        </Stack>
                    )}

                    {/* Relationships section */}
                    {Array.isArray(entity.relationshipObjects) && entity.relationshipObjects.length > 0 && (
                        <Stack gap="xs">
                            {entity.relationshipObjects.map(relationship => {
                                if (!relationship) return null;

                                if (relationship.direction === Direction.Backward) return null;

                                // Find the target entity
                                const targetEntity = relationship.right_entity && Array.isArray(allEntities) ?
                                    allEntities.find(e => e && e.id === relationship.right_entity) : null;

                                return (
                                    <Text key={relationship.id} size="sm" px="sm" style={{
                                        color: relationship.strength === Strength.Strong ? '#cc0000' : '#666666'
                                    }}>
                                        {relationship.field_name || 'Unnamed Relationship'} → {
                                        targetEntity ? targetEntity.name : 'Unknown'
                                    }
                                        {relationship.strength === Strength.Strong ? ' (strong)' : ''}
                                    </Text>
                                );
                            })}
                        </Stack>
                    )}
                </Stack>
            </Paper>

            {/* Output connection point */}
            <Handle type="source" position={Position.Bottom}/>
        </div>
    );
});

// Define nodeTypes outside the component to prevent recreation on each render
const nodeTypes = {
    entityNode: EntityNode
};

// Define edgeTypes outside the component to prevent recreation on each render
const edgeTypes = {};

// Create an ELK instance
const elk = new ELK();

// Define defaultEdgeOptions outside the component to prevent recreation on each render
const defaultEdgeOptions = {
    type: 'smoothstep',
    style: {strokeWidth: 2}
};

// Define miniMapProps outside the component to prevent recreation on each render
const miniMapProps = {
    nodeStrokeColor: "#aaa",
    nodeColor: "#fff",
    nodeBorderRadius: 2
};

// Define backgroundProps outside the component to prevent recreation on each render
const backgroundProps = {
    gap: 12,
    size: 1,
    color: "#f8f8f8"
};

/**
 * Main component for the entity map flow
 */
const EntityMapFlow = () => {
    const {
        entities,
        fields,
        relationships,
        isLoadingEntities,
        isLoadingFields,
        isLoadingRelationships,
        entityError,
        fieldError,
        relationshipError
    } = useEntityContext();

    // Initialize with empty arrays but with explicit types
    const [nodes, setNodes, onNodesChange] = useNodesState<Node[]>([]);
    const [edges, setEdges, onEdgesChange] = useEdgesState<Edge[]>([]);
    const [loading, setLoading] = useState(false);
    const [componentError, setComponentError] = useState<Error | null>(null);

    // Memoize all objects passed to ReactFlow to prevent recreation on each render
    const memoizedNodeTypes = useMemo(() => nodeTypes, []);
    const memoizedEdgeTypes = useMemo(() => edgeTypes, []);
    const memoizedDefaultEdgeOptions = useMemo(() => defaultEdgeOptions, []);
    const memoizedMiniMapProps = useMemo(() => miniMapProps, []);
    const memoizedBackgroundProps = useMemo(() => backgroundProps, []);

    // Monitor edges state changes
    useEffect(() => {
        console.log("Edges state changed:", edges.length, "edges now in state");
    }, [edges]);

    /**
     * Prepare entity data with fields and relationships from context
     */
    useEffect(() => {
        if (isLoadingEntities || isLoadingFields || isLoadingRelationships) {
            return;
        }

        if (entities.length === 0) {
            // No entities to display
            return;
        }

        try {
            // Prepare entities with their fields and relationships from context
            const preparedEntities = entities.map(entity => {
                // Find all fields for this entity
                const entityFields = fields.filter(field => {
                    // Check if this field belongs to the entity
                    return entity && Array.isArray(entity.fields) && entity.fields.includes(field.id);
                });

                // Find all relationships for this entity
                const entityRelationships = relationships.filter(relationship => {
                    // Check if this relationship has this entity as the left entity
                    return relationship && relationship.left_entity === entity.id;
                });

                return {
                    ...entity,
                    fieldObjects: entityFields,
                    relationshipObjects: entityRelationships
                };
            });
            console.log("Prepared entities:", preparedEntities);

            // Debug log before creating nodes and edges
            console.log("Calling createNodesAndEdges with prepared entities:", preparedEntities);

            // Log context data for debugging
            console.log("Context data - Total entities:", entities.length);
            console.log("Context data - Total fields:", fields.length);
            console.log("Context data - Total relationships:", relationships.length);

            // Log detailed relationship data
            console.log("All relationships from context:", relationships);

            // Log relationship data for debugging
            preparedEntities.forEach(entity => {
                console.log(`Entity ${entity.name} (ID: ${entity.id}) has ${entity.fieldObjects?.length || 0} fields and ${entity.relationshipObjects?.length || 0} relationships:`);

                if (Array.isArray(entity.fieldObjects)) {
                    entity.fieldObjects.forEach(field => {
                        if (field.field_type === FieldType.Entity && field.strong === true) {
                            console.log(`  - Strong entity field: ${field.name} (ID: ${field.id}) pointing to entity ID: ${field.entity}`);
                        }
                    });
                }

                if (Array.isArray(entity.relationshipObjects)) {
                    entity.relationshipObjects.forEach(rel => {
                        console.log(`  - Relationship ID ${rel.id}: ${rel.field_name} from ${rel.left_entity} to ${rel.right_entity} (${rel.strength})`);
                    });
                }
            });

            // Create nodes and edges for ReactFlow using ELK for layout
            createNodesAndEdges(preparedEntities);
        } catch (err) {
            // Log the error and set the component error state
            logError(`Error preparing entity data: ${err}`);
            setComponentError(err instanceof Error ? err : new Error(`${err}`));
        }
    }, [entities, fields, relationships, isLoadingEntities, isLoadingFields, isLoadingRelationships]);

    /**
     * Create nodes and edges for ReactFlow using ELK for layout
     */
    const createNodesAndEdges = useCallback(async (entities: EntityWithFields[]) => {
        if (!entities.length) return;

        setLoading(true);

        // Debug log to check entities and their relationships
        //console.log("Creating nodes and edges for entities:", entities);

        // Debug log to check the relationships array in each entity
        entities.forEach(entity => {
            console.log(`Entity ${entity.name} (ID: ${entity.id}) relationships:`, entity.relationships);
            console.log(`Entity ${entity.name} (ID: ${entity.id}) fieldObjects:`, entity.fieldObjects);
        });

        try {
            const newNodes: Node[] = [];
            const newEdges: Edge[] = [];

            // Separate heritage entities from regular entities
            const heritageEntities = entities.filter(e => e && e.only_for_heritage === true);
            const regularEntities = entities.filter(e => e && e.only_for_heritage !== true);

            // Create initial nodes for all entities (without positions yet)
            const elkNodes = regularEntities.map(entity => ({
                id: `entity-${entity.id}`,
                width: 250,  // Approximate width of the node
                height: 200, // Approximate height of the node
            }));

            // Create edges for parent-child relationships
            const parentChildEdges: any[] = [];
            regularEntities.forEach(entity => {
                // Defensive check to ensure entity.parent exists and is not null
                if (entity && entity.parent !== null && entity.parent !== undefined) {
                    const parentEntity = entities.find(e => e && e.id === entity.parent);

                    // Only create edge if parent is not a heritage entity
                    if (parentEntity && parentEntity.only_for_heritage === false) {
                        parentChildEdges.push({
                            id: `edge-parent-${entity.parent}-${entity.id}`,
                            source: `entity-${entity.parent}`,
                            target: `entity-${entity.id}`,
                            // Keep parent-child relationships as smoothstep (now used for strong relationships)
                            type: 'smoothstep'
                        });
                    }
                }
            });

            // Create edges for entity relationships from relationship objects
            const relationshipEdges: any[] = [];
            regularEntities.forEach(entity => {
                // Defensive check to ensure entity.relationshipObjects exists and is an array
                if (!Array.isArray(entity.relationshipObjects)) {
                    console.log(`Entity ${entity.name} has no relationshipObjects array`);
                    return;
                }

                console.log(`Processing relationships for entity ${entity.name}:`, entity.relationshipObjects);

                // Create edges from explicit relationship objects
                entity.relationshipObjects.forEach(relationship => {
                    try {
                        if (!relationship) {
                            console.log(`Skipping null relationship for entity ${entity.name}`);
                            return;
                        }

                        if (!relationship.id) {
                            console.log(`Skipping relationship without ID for entity ${entity.name}`);
                            return;
                        }

                        const sourceEntityId = relationship.left_entity;
                        const targetEntityId = relationship.right_entity;

                        if (!sourceEntityId || !targetEntityId) {
                            console.log(`Skipping relationship ${relationship.id} with missing source or target entity`);
                            return;
                        }

                        // Verify that source entity matches current entity
                        if (sourceEntityId !== entity.id) {
                            console.log(`Warning: Relationship ${relationship.id} source entity ${sourceEntityId} doesn't match current entity ${entity.id}`);
                            // Continue anyway as this might be intentional
                        }

                        const targetEntity = entities.find(e => e && e.id === targetEntityId);

                        if (!targetEntity) {
                            console.log(`Skipping relationship ${relationship.id}: target entity ${targetEntityId} not found`);
                            return;
                        }

                        // Only create edge if target is not a heritage entity
                        if (targetEntity.only_for_heritage === false) {
                            console.log(`Creating relationship edge from ${sourceEntityId} to ${targetEntityId} with label ${relationship.field_name}`);

                            relationshipEdges.push({
                                id: `edge-rel-${sourceEntityId}-${relationship.id}-${targetEntityId}`,
                                source: `entity-${sourceEntityId}`,
                                target: `entity-${targetEntityId}`,
                                // Use smoothstep for strong relationships to visually differentiate them
                                type: relationship.strength === Strength.Strong ? 'smoothstep' : 'default',
                                label: relationship.field_name,
                                // Add visual distinction for strong relationships
                                animated: relationship.strength === Strength.Strong,
                                style: {
                                    stroke: relationship.strength === Strength.Strong ? '#cc0000' : '#666666',
                                    strokeWidth: 2
                                }
                            });
                        } else {
                            console.log(`Skipping relationship ${relationship.id}: target entity ${targetEntityId} is heritage-only`);
                        }
                    } catch (err) {
                        console.log(`Error processing relationship for entity ${entity.name}: ${err}`);
                    }
                });

                // Also check fields for strong entity relationships (as a fallback)
                if (Array.isArray(entity.fieldObjects)) {
                    entity.fieldObjects.forEach(field => {
                        if (field && field.field_type === FieldType.Entity && field.strong === true && field.entity) {
                            const targetEntityId = field.entity;
                            const targetEntity = entities.find(e => e && e.id === targetEntityId);

                            // Only create edge if target is not a heritage entity and we don't already have this edge
                            if (targetEntity && targetEntity.only_for_heritage === false) {
                                const edgeExists = relationshipEdges.some(edge =>
                                    edge.source === `entity-${entity.id}` && edge.target === `entity-${targetEntityId}`
                                );

                                if (!edgeExists) {
                                    console.log(`Creating field-based edge from ${entity.id} to ${targetEntityId} with label ${field.name}`);

                                    relationshipEdges.push({
                                        id: `edge-field-${entity.id}-${field.id}-${targetEntityId}`,
                                        source: `entity-${entity.id}`,
                                        target: `entity-${targetEntityId}`,
                                        // Use smoothstep for field-based edges as they represent strong relationships
                                        type: 'smoothstep',
                                        label: field.name
                                    });
                                }
                            }
                        }
                    });
                }
            });

            // Combine all edges for ELK
            const elkEdges = [...parentChildEdges, ...relationshipEdges];

            // Prepare the graph for ELK layout
            const elkGraph = {
                id: 'root',
                layoutOptions: {
                    'elk.algorithm': 'layered',
                    'elk.direction': 'DOWN',
                    'elk.spacing.nodeNode': '100',
                    'elk.layered.spacing.nodeNodeBetweenLayers': '120',
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
                // Defensive check to ensure entity exists and has an id
                if (!entity || entity.id === undefined) {
                    return;
                }

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

            // Position heritage entities in a separate area
            // Find the maximum x and y coordinates from the regular entities
            let maxX = 0;
            let maxY = 0;

            newNodes.forEach(node => {
                maxX = Math.max(maxX, node.position.x);
                maxY = Math.max(maxY, node.position.y);
            });

            // Add heritage entities to the right side
            heritageEntities.forEach((entity, index) => {
                if (!entity || entity.id === undefined) {
                    return;
                }

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
                // Defensive check to ensure entity exists and has an id
                if (!entity || entity.id === undefined) {
                    return;
                }

                if (entity.parent !== null && entity.parent !== undefined) {
                    const parentEntity = entities.find(e => e && e.id === entity.parent);

                    // Only create edge if parent is not a heritage entity
                    if (parentEntity && parentEntity.only_for_heritage === false) {
                        newEdges.push({
                            id: `edge-parent-${entity.parent}-${entity.id}`,
                            source: `entity-${entity.parent}`,
                            target: `entity-${entity.id}`,
                            // Keep parent-child relationships as smoothstep (now used for strong relationships)
                            type: 'smoothstep',
                            animated: false,
                            style: {stroke: '#555', strokeWidth: 2}
                        });
                    }
                }
            });

            // Relationship edges from relationship objects
            regularEntities.forEach(entity => {
                // Defensive check to ensure entity exists and has an id
                if (!entity || entity.id === undefined) {
                    return;
                }

                // Defensive check to ensure entity.relationshipObjects exists and is an array
                if (!Array.isArray(entity.relationshipObjects)) {
                    return;
                }

                // Create edges from explicit relationship objects
                entity.relationshipObjects.forEach(relationship => {
                    try {
                        if (!relationship) {
                            console.log(`Skipping null relationship for entity ${entity.name} in ReactFlow edges`);
                            return;
                        }

                        if (!relationship.id) {
                            console.log(`Skipping relationship without ID for entity ${entity.name} in ReactFlow edges`);
                            return;
                        }

                        const sourceEntityId = relationship.left_entity;
                        const targetEntityId = relationship.right_entity;

                        if (!sourceEntityId || !targetEntityId) {
                            console.log(`Skipping relationship ${relationship.id} with missing source or target entity in ReactFlow edges`);
                            return;
                        }

                        // Verify that source entity matches current entity
                        if (sourceEntityId !== entity.id) {
                            console.log(`Warning: Relationship ${relationship.id} source entity ${sourceEntityId} doesn't match current entity ${entity.id} in ReactFlow edges`);
                            // Continue anyway as this might be intentional
                        }

                        const targetEntity = entities.find(e => e && e.id === targetEntityId);

                        if (!targetEntity) {
                            console.log(`Skipping relationship ${relationship.id}: target entity ${targetEntityId} not found in ReactFlow edges`);
                            return;
                        }

                        // Only create edge if target is not a heritage entity
                        if (targetEntity.only_for_heritage === false) {
                            console.log(`Creating ReactFlow edge from ${sourceEntityId} to ${targetEntityId}`);

                            newEdges.push({
                                id: `edge-rel-${sourceEntityId}-${relationship.id}-${targetEntityId}`,
                                source: `entity-${sourceEntityId}`,
                                target: `entity-${targetEntityId}`,
                                // Use smoothstep for strong relationships to visually differentiate them
                                type: relationship.strength === Strength.Strong ? 'smoothstep' : 'default',
                                animated: relationship.strength === Strength.Strong,
                                style: {
                                    stroke: relationship.strength === Strength.Strong ? '#cc0000' : '#666666',
                                    strokeWidth: 2
                                },
                                label: relationship.field_name || '',
                                labelStyle: {
                                    fill: relationship.strength === Strength.Strong ? '#cc0000' : '#666666',
                                    fontWeight: 500
                                },
                                labelBgStyle: {fill: '#ffffff', fillOpacity: 0.8}
                            });
                        } else {
                            console.log(`Skipping relationship ${relationship.id}: target entity ${targetEntityId} is heritage-only in ReactFlow edges`);
                        }
                    } catch (err) {
                        console.log(`Error processing relationship for entity ${entity.name} in ReactFlow edges: ${err}`);
                    }
                });

                // Also check fields for strong entity relationships (as a fallback)
                if (Array.isArray(entity.fieldObjects)) {
                    entity.fieldObjects.forEach(field => {
                        try {
                            // Defensive check to ensure field exists and has valid properties
                            if (!field || field.id === undefined) {
                                return;
                            }

                            if (field.field_type === FieldType.Entity && field.strong === true && field.entity) {
                                const targetEntityId = field.entity;

                                if (!targetEntityId) {
                                    console.log(`Field ${field.name} (ID: ${field.id}) has entity type but no target entity ID`);
                                    return;
                                }

                                const targetEntity = entities.find(e => e && e.id === targetEntityId);

                                if (!targetEntity) {
                                    console.log(`Field ${field.name} (ID: ${field.id}) references non-existent entity ID: ${targetEntityId}`);
                                    return;
                                }

                                // Only create edge if target is not a heritage entity and we don't already have this edge
                                if (targetEntity.only_for_heritage === false) {
                                    const edgeExists = newEdges.some(edge =>
                                        edge.source === `entity-${entity.id}` && edge.target === `entity-${targetEntityId}`
                                    );

                                    if (!edgeExists) {
                                        console.log(`Creating field-based ReactFlow edge from ${entity.id} to ${targetEntityId}`);

                                        newEdges.push({
                                            id: `edge-field-${entity.id}-${field.id}-${targetEntityId}`,
                                            source: `entity-${entity.id}`,
                                            target: `entity-${targetEntityId}`,
                                            // Use smoothstep for field-based edges as they represent strong relationships
                                            type: 'smoothstep',
                                            animated: true,
                                            style: {
                                                stroke: '#cc0000',
                                                strokeWidth: 2
                                            },
                                            label: field.name || '',
                                            labelStyle: {fill: '#cc0000', fontWeight: 500},
                                            labelBgStyle: {fill: '#ffffff', fillOpacity: 0.8}
                                        });
                                    } else {
                                        console.log(`Skipping field-based edge from ${entity.id} to ${targetEntityId} as it already exists`);
                                    }
                                } else {
                                    console.log(`Skipping field ${field.name} (ID: ${field.id}): target entity ${targetEntityId} is heritage-only`);
                                }
                            }
                        } catch (err) {
                            console.log(`Error processing field ${field?.name || field?.id || 'unknown'} for entity ${entity.name}: ${err}`);
                        }
                    });
                }
            });

            // Debug log to check the edges being created
            console.log("Created edges:", newEdges);
            console.log("Edge count:", newEdges.length);
            console.log("Edge properties:", newEdges.map(edge => ({
                id: edge.id,
                source: edge.source,
                target: edge.target,
                type: edge.type
            })));

            // Log relationship edges specifically
            const relationshipEdgesCount = newEdges.filter(edge => edge.id.includes('edge-rel-')).length;
            console.log("Relationship edges count:", relationshipEdgesCount);
            console.log("Relationship edges:", newEdges.filter(edge => edge.id.includes('edge-rel-')));

            // Log before setting state
            console.log("Setting nodes:", newNodes.length, "and edges:", newEdges.length);

            setNodes(newNodes);
            setEdges(newEdges);

            // Log after setting state to verify
            console.log("After setEdges, current edges state:", edges.length);
        } catch (err) {
            logError(`Failed to layout diagram: ${err}`);
            setComponentError(err instanceof Error ? err : new Error(`${err}`));
        } finally {
            setLoading(false);
        }
    }, [setNodes, setEdges, setLoading, setComponentError]);

    // Custom fallback component for error state
    const errorFallback = (
        <Alert color="yellow" title="Entity map could not be loaded">
            There was an issue loading the entity map. Please try again later.
        </Alert>
    );

    // Loading state
    if (isLoadingEntities || isLoadingFields) {
        return (
            <Alert color="blue" title="Loading entity data">
                <Group>
                    <Loader size="sm"/>
                    <Text>Please wait while we load the entity data...</Text>
                </Group>
            </Alert>
        );
    }

    // Error state
    if (entityError || fieldError || relationshipError) {
        return (
            <Alert color="red" title="Error loading entity data">
                {entityError instanceof Error ? entityError.message :
                    fieldError instanceof Error ? fieldError.message :
                        relationshipError instanceof Error ? relationshipError.message : 'An unknown error occurred'}
            </Alert>
        );
    }

    // No entities state
    if (entities.length === 0) {
        return (
            <Alert color="gray" title="No entities found">
                No entities have been created yet. Please create some entities first.
            </Alert>
        );
    }

    // If there's a component error, display an error message
    if (componentError) {
        return (
            <Alert color="red" title="Error in entity map">
                <p>An error occurred while creating the entity map:</p>
                <p>{componentError.message}</p>
                <Group justify="flex-end" mt="md">
                    <Button
                        onClick={() => setComponentError(null)}
                        color="red"
                        variant="light"
                    >
                        Try Again
                    </Button>
                </Group>
            </Alert>
        );
    }

    return (
        <ErrorBoundary fallback={errorFallback}>
            <div style={{
                width: '100%',
                height: '80vh',
                position: 'relative',
                border: '1px solid #eee',
                borderRadius: '4px'
            }}>
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
                        <Stack align="center">
                            <Loader size="md"/>
                            <Text size="xl" fw={700}>Calculating layout...</Text>
                        </Stack>
                    </div>
                )}
                {/* Debug log to check the edges being passed to ReactFlow */}
                {/* console.log("Rendering ReactFlow with edges:", edges) */}

                <ReactFlow
                    nodes={nodes}
                    edges={edges}
                    onNodesChange={onNodesChange}
                    onEdgesChange={onEdgesChange}
                    nodeTypes={memoizedNodeTypes}
                    edgeTypes={memoizedEdgeTypes}
                    fitView
                    attributionPosition="bottom-left"
                    minZoom={0.2}
                    maxZoom={1.5}
                    defaultEdgeOptions={memoizedDefaultEdgeOptions}
                >
                    <Controls/>
                    <MiniMap
                        {...memoizedMiniMapProps}
                    />
                    <Background {...memoizedBackgroundProps}/>
                </ReactFlow>
            </div>
        </ErrorBoundary>
    );
};

/**
 * Main EntityMap component with ReactFlowProvider
 */
const EntityMap = () => {
    const [rootId, setRootId] = useState<number | null>(null);


    // Function to get the root ID
    async function getRootId() {
        try {
            const roots = await rootService.getRootMulti([]);
            info(`Root ID initialized: ${JSON.stringify(roots)}`);
            if (roots.length > 0 && roots[0] !== null) {
                setRootId(roots[0]!.id);
                return roots[0]!.id;
            }
            return null;
        } catch (err) {
            error(`Error getting root ID: ${err}`);
            throw err;
        }
    }

    // Initialize root ID on component mount
    useEffect(() => {
        const fetchData = async () => {
            try {
                const rootId = await getRootId(); // Initialize rootId
                if (!rootId) {
                    error("No root found. Please create a root first.");
                    return;
                }
                setRootId(rootId);
            } catch (err) {
                const errorMessage = `Failed to fetch root ID: ${err}`;
                error(errorMessage);
            }
        };

        fetchData().catch((err) => {
            const errorMessage = `Unexpected error: ${err}`;
            error(errorMessage);
        });
    }, []);


    return (
        <div className="p-10">
            <Title order={1} mb="xl">Entity Relationship Diagram</Title>
            <Text mb="lg" c="dimmed">
                This diagram shows the relationships between entities in your data model.
                Heritage entities are shown on the right side. Strong relationships are shown in red.
            </Text>
            <EntityProvider rootId={rootId}>
                <ReactFlowProvider>
                    <EntityMapFlow/>
                </ReactFlowProvider>
            </EntityProvider>
        </div>
    );
};

export default EntityMap;