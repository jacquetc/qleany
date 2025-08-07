import {invokeCommand} from './tauri-api';

/**
 * Enum for relationship types
 */
export enum RelationshipType {
    OneToOne = "OneToOne",
    OneToMany = "OneToMany",
    ManyToOne = "ManyToOne",
    ManyToMany = "ManyToMany",
}

export enum Strength {
    Weak = "Weak",
    Strong = "Strong"
}

export enum Direction {
    Forward = "Forward",
    Backward = "Backward"
}

export enum Cardinality {
    ZeroOrOne = "ZeroOrOne",
    One = "One",
    ZeroOrMore = "ZeroOrMore",
    OneOrMore = "OneOrMore"
}

export enum Order {
    Ordered = "Ordered",
    Unordered = "Unordered"
}

export enum RelationshipRelationshipField {
    LeftEntity = "LeftEntity",
    RightEntity = "RightEntity"
}

/**
 * Types for relationship-related DTOs
 */
export interface CreateRelationshipDTO {
    left_entity: number;
    right_entity: number;
    field_name: string;
    relationship_type: RelationshipType;
    strength: Strength;
    direction: Direction;
    cardinality: Cardinality;
    order: Order | null;
}

export interface RelationshipDTO {
    id: number;
    left_entity: number;
    right_entity: number;
    field_name: string;
    relationship_type: RelationshipType;
    strength: Strength;
    direction: Direction;
    cardinality: Cardinality;
    order: Order | null;
}

/**
 * Relationship service for interacting with the Tauri backend
 */
export const relationshipService = {
    /**
     * Create a new relationship
     * @param dto The relationship data to create
     * @returns The created relationship
     */
    createRelationship: (dto: CreateRelationshipDTO): Promise<RelationshipDTO> => {
        return invokeCommand<RelationshipDTO>("create_relationship", {dto});
    },

    /**
     * Create multiple relationships
     * @param dtos The relationship data to create
     * @returns The created relationships
     */
    createRelationshipMulti: (dtos: CreateRelationshipDTO[]): Promise<RelationshipDTO[]> => {
        return invokeCommand<RelationshipDTO[]>("create_relationship_multi", {dtos});
    },

    /**
     * Get a relationship by ID
     * @param id The relationship ID
     * @returns The relationship or null if not found
     */
    getRelationship: (id: number): Promise<RelationshipDTO | null> => {
        return invokeCommand<RelationshipDTO | null>("get_relationship", {id});
    },

    /**
     * Get multiple relationships by IDs
     * @param ids The relationship IDs
     * @returns Array of relationships (null for relationships not found)
     */
    getRelationshipMulti: (ids: number[]): Promise<(RelationshipDTO | null)[]> => {
        return invokeCommand<(RelationshipDTO | null)[]>("get_relationship_multi", {ids});
    },

    /**
     * Update a relationship
     * @param dto The relationship data to update
     * @returns The updated relationship
     */
    updateRelationship: (dto: RelationshipDTO): Promise<RelationshipDTO> => {
        return invokeCommand<RelationshipDTO>("update_relationship", {dto});
    },

    /**
     * Update multiple relationships
     * @param dtos The relationship data to update
     * @returns The updated relationships
     */
    updateRelationshipMulti: (dtos: RelationshipDTO[]): Promise<RelationshipDTO[]> => {
        return invokeCommand<RelationshipDTO[]>("update_relationship_multi", {dtos});
    },

    /**
     * Remove a relationship
     * @param id The relationship ID to remove
     */
    removeRelationship: (id: number): Promise<void> => {
        return invokeCommand<void>("remove_relationship", {id});
    },

    /**
     * Remove multiple relationships
     * @param ids The relationship IDs to remove
     */
    removeRelationshipMulti: (ids: number[]): Promise<void> => {
        return invokeCommand<void>("remove_relationship_multi", {ids});
    },

    /**
     * Get a relationship for a relationship
     * @param id The relationship ID
     * @param field The relationship field
     * @returns Array of related entity IDs
     */
    getRelationshipRelationship: (id: number, field: string): Promise<number[]> => {
        return invokeCommand<number[]>("get_relationship_relationship", {id, field});
    },

    /**
     * Set a relationship for a relationship
     * @param dto The relationship data
     */
    setRelationshipRelationship: (dto: { id: number, field: string, right_ids: number[] }): Promise<void> => {
        return invokeCommand<void>("set_relationship_relationship", {dto});
    }
};