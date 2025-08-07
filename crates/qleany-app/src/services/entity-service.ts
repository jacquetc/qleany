import {invokeCommand} from './tauri-api';

/**
 * Enum for entity relationship fields
 */
export enum EntityRelationshipField {
    Fields = "Fields",
    Relationships = "Relationships",
}

/**
 * Types for entity-related DTOs
 */
export interface CreateEntityDTO {
    name: string;
    only_for_heritage: boolean;
    parent: number | null;
    allow_direct_access: boolean;
    fields: number[];
    relationships: number[];
}

export interface EntityDTO {
    id: number;
    name: string;
    only_for_heritage: boolean;
    parent: number | null;
    allow_direct_access: boolean;
    fields: number[];
    relationships: number[];
}

export interface EntityRelationshipDTO {
    id: number;
    field: EntityRelationshipField;
    right_ids: number[];
}

/**
 * Entity service for interacting with the Tauri backend
 */
export const entityService = {
    /**
     * Create a new entity
     * @param dto The entity data to create
     * @returns The created entity
     */
    createEntity: (dto: CreateEntityDTO): Promise<EntityDTO> => {
        return invokeCommand<EntityDTO>("create_entity", {dto});
    },

    /**
     * Create multiple entities
     * @param dtos The entity data to create
     * @returns The created entities
     */
    createEntityMulti: (dtos: CreateEntityDTO[]): Promise<EntityDTO[]> => {
        return invokeCommand<EntityDTO[]>("create_entity_multi", {dtos});
    },

    /**
     * Get an entity by ID
     * @param id The entity ID
     * @returns The entity or null if not found
     */
    getEntity: (id: number): Promise<EntityDTO | null> => {
        return invokeCommand<EntityDTO | null>("get_entity", {id});
    },

    /**
     * Get multiple entities by IDs
     * @param ids The entity IDs
     * @returns Array of entities (null for entities not found)
     */
    getEntityMulti: (ids: number[]): Promise<(EntityDTO | null)[]> => {
        return invokeCommand<(EntityDTO | null)[]>("get_entity_multi", {ids});
    },

    /**
     * Update an entity
     * @param dto The entity data to update
     * @returns The updated entity
     */
    updateEntity: (dto: EntityDTO): Promise<EntityDTO> => {
        return invokeCommand<EntityDTO>("update_entity", {dto});
    },

    /**
     * Update multiple entities
     * @param dtos The entity data to update
     * @returns The updated entities
     */
    updateEntityMulti: (dtos: EntityDTO[]): Promise<EntityDTO[]> => {
        return invokeCommand<EntityDTO[]>("update_entity_multi", {dtos});
    },

    /**
     * Remove an entity
     * @param id The entity ID to remove
     */
    removeEntity: (id: number): Promise<void> => {
        return invokeCommand<void>("remove_entity", {id});
    },

    /**
     * Remove multiple entities
     * @param ids The entity IDs to remove
     */
    removeEntityMulti: (ids: number[]): Promise<void> => {
        return invokeCommand<void>("remove_entity_multi", {ids});
    },

    /**
     * Get a relationship for an entity
     * @param id The entity ID
     * @param field The relationship field
     * @returns Array of related entity IDs
     */
    getEntityRelationship: (id: number, field: EntityRelationshipField): Promise<number[]> => {
        return invokeCommand<number[]>("get_entity_relationship", {id, field});
    },

    /**
     * Set a relationship for an entity
     * @param dto The relationship data
     */
    setEntityRelationship: (dto: EntityRelationshipDTO): Promise<void> => {
        return invokeCommand<void>("set_entity_relationship", {dto});
    }
};