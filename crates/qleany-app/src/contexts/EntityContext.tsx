import React, {createContext, useContext, useState} from 'react';
import {Alert} from '@mantine/core';
import {error as logError} from '@tauri-apps/plugin-log';
import {useEntities} from '../hooks/useEntities';
import {useFields} from '../hooks/useFields';
import {useRelationships} from '../hooks/useRelationships';
import {EntityDTO} from '../services/entity-service';
import {FieldDTO, FieldType} from '../services/field-service';
import {
    Cardinality,
    Direction,
    Order,
    RelationshipDTO,
    RelationshipType,
    Strength
} from '../services/relationship-service';

/**
 * Interface for the EntityContext value
 */
interface EntityContextValue {
    // Entities data
    entities: EntityDTO[];
    selectedEntityId: number | null;
    isLoadingEntities: boolean;
    entityError: unknown;

    // Fields data
    fields: FieldDTO[];
    selectedFieldId: number | null;
    isLoadingFields: boolean;
    fieldError: unknown;

    // Relationships data
    relationships: RelationshipDTO[];
    selectedRelationshipId: number | null;
    isLoadingRelationships: boolean;
    relationshipError: unknown;

    // Hook error
    hookError: Error | null;

    // Actions
    selectEntity: (entityId: number | null) => void;
    createEntity: () => void;
    updateEntity: (entity: EntityDTO) => void;
    removeEntity: (entityId: number) => void;

    selectField: (fieldId: number | null) => void;
    createField: (fieldData?: {
        name: string;
        fieldType: FieldType;
        entity?: number | null;
        isNullable: boolean;
        isPrimaryKey: boolean;
        isList: boolean;
        single: boolean;
        strong: boolean;
        ordered: boolean;
        listModel: boolean;
        listModelDisplayedField?: string | null;
    }) => void;
    updateField: (field: FieldDTO) => void;
    removeField: (fieldId: number) => void;

    selectRelationship: (relationshipId: number | null) => void;
    createRelationship: (relationshipData?: {
        fieldName: string;
        relationshipType: RelationshipType;
        rightEntityId: number;
        strength: Strength;
        direction: Direction;
        cardinality: Cardinality;
        order: Order | null;
    }) => void;
    updateRelationship: (relationship: RelationshipDTO) => void;
    removeRelationship: (relationshipId: number) => void;

    refetchAll: () => Promise<void>;
}

// Create the context with a default value
const EntityContext = createContext<EntityContextValue | undefined>(undefined);

/**
 * Props for the EntityProvider component
 */
interface EntityProviderProps {
    rootId: number | null;
    children: React.ReactNode;
}

/**
 * EntityProvider component that provides entity-related data to its children
 */
export function EntityProvider({rootId, children}: EntityProviderProps) {
    // State for selected items
    const [selectedEntityId, setSelectedEntityId] = useState<number | null>(null);
    const [selectedFieldId, setSelectedFieldId] = useState<number | null>(null);
    const [selectedRelationshipId, setSelectedRelationshipId] = useState<number | null>(null);

    // Add state for handling hook errors
    const [hookError, setHookError] = useState<Error | null>(null);

    // Use the custom hooks for data with error handling
    let entitiesData = {
        entities: [],
        isLoading: true,
        error: null,
        createEntity: () => {
        },
        updateEntity: (_: EntityDTO) => {
        },
        removeEntity: (_: number) => {
        },
        refetch: async () => {
        }
    };

    let fieldsData = {
        fields: [],
        isLoading: true,
        error: null,
        createField: (_?: any) => {
        },
        updateField: (_: FieldDTO) => {
        },
        removeField: (_: number) => {
        },
        refetch: async () => {
        }
    };

    let relationshipsData = {
        relationships: [],
        isLoading: true,
        error: null,
        createRelationship: (_?: any) => {
        },
        updateRelationship: (_: RelationshipDTO) => {
        },
        removeRelationship: (_: number) => {
        },
        refetch: async () => {
        }
    };

    try {
        entitiesData = useEntities(rootId);
    } catch (err) {
        const errorMessage = `Error in useEntities hook: ${err}`;
        logError(errorMessage);
        setHookError(err instanceof Error ? err : new Error('Unknown error in useEntities hook'));
    }

    // For fields and relationships, we need to fetch data for all entities, not just the selected one
    // We'll use null to fetch all fields and relationships
    try {
        fieldsData = useFields(null);
    } catch (err) {
        const errorMessage = `Error in useFields hook: ${err}`;
        logError(errorMessage);
        setHookError(err instanceof Error ? err : new Error('Unknown error in useFields hook'));
    }

    try {
        relationshipsData = useRelationships(null);
    } catch (err) {
        const errorMessage = `Error in useRelationships hook: ${err}`;
        logError(errorMessage);
        setHookError(err instanceof Error ? err : new Error('Unknown error in useRelationships hook'));
    }

    const {
        entities,
        isLoading: isLoadingEntities,
        error: entityError,
        createEntity,
        updateEntity,
        removeEntity,
        refetch: refetchEntities
    } = entitiesData;

    const {
        fields,
        isLoading: isLoadingFields,
        error: fieldError,
        createField,
        updateField,
        removeField,
        refetch: refetchFields
    } = fieldsData;

    const {
        relationships,
        isLoading: isLoadingRelationships,
        error: relationshipError,
        createRelationship,
        updateRelationship,
        removeRelationship,
        refetch: refetchRelationships
    } = relationshipsData;

    // Function to select an entity
    const selectEntity = (entityId: number | null) => {
        setSelectedEntityId(entityId);
        setSelectedFieldId(null); // Reset selected field when entity changes
        setSelectedRelationshipId(null); // Reset selected relationship when entity changes
    };

    // Function to select a field
    const selectField = (fieldId: number | null) => {
        setSelectedFieldId(fieldId);
        setSelectedRelationshipId(null); // Reset selected relationship when field changes
    };

    // Function to select a relationship
    const selectRelationship = (relationshipId: number | null) => {
        setSelectedRelationshipId(relationshipId);
        setSelectedFieldId(null); // Reset selected field when relationship changes
    };

    // Function to refetch all data
    const refetchAll = async () => {
        await Promise.all([
            refetchEntities(),
            refetchFields(),
            refetchRelationships()
        ]);
    };

    // Log data availability for debugging
    console.log(`EntityContext - Entities: ${entities.length}, Fields: ${fields.length}, Relationships: ${relationships.length}`);
    console.log(`EntityContext - Loading states - Entities: ${isLoadingEntities}, Fields: ${isLoadingFields}, Relationships: ${isLoadingRelationships}`);

    if (entities.length > 0 && fields.length > 0) {
        // Log a sample of field data to verify it's being loaded correctly
        console.log(`EntityContext - Sample field data:`, fields.slice(0, 3));
    }

    if (entities.length > 0 && relationships.length > 0) {
        // Log a sample of relationship data to verify it's being loaded correctly
        console.log(`EntityContext - Sample relationship data:`, relationships.slice(0, 3));
    }

    // Create the context value
    const contextValue: EntityContextValue = {
        // Entities data
        entities,
        selectedEntityId,
        isLoadingEntities,
        entityError,

        // Fields data
        fields,
        selectedFieldId,
        isLoadingFields,
        fieldError,

        // Relationships data
        relationships,
        selectedRelationshipId,
        isLoadingRelationships,
        relationshipError,

        // Hook error
        hookError,

        // Actions
        selectEntity,
        createEntity,
        updateEntity,
        removeEntity,

        selectField,
        createField,
        updateField,
        removeField,

        selectRelationship,
        createRelationship,
        updateRelationship,
        removeRelationship,

        refetchAll
    };

    // If there's a hook error, render an error message
    if (hookError) {
        return (
            <Alert color="red" title="Error initializing entity data">
                <p>{hookError.message}</p>
                <p>Please try refreshing the page.</p>
            </Alert>
        );
    }

    return (
        <EntityContext.Provider value={contextValue}>
            {children}
        </EntityContext.Provider>
    );
}

/**
 * Custom hook to use the EntityContext
 * @returns The EntityContext value
 * @throws Error if used outside of a EntityProvider
 */
export function useEntityContext() {
    const context = useContext(EntityContext);

    if (context === undefined) {
        throw new Error('useEntityContext must be used within a EntityProvider');
    }

    return context;
}