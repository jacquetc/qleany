import {invokeCommand} from './tauri-api';

/**
 * Enum for root relationship fields
 */
export enum RootRelationshipField {
    Global = "Global",
    Entities = "Entities",
    Features = "Features",
    Files = "Files",
}

/**
 * Types for root-related DTOs
 */
export interface CreateRootDTO {
    global: number;
    manifest_absolute_path: string;
    entities: number[];
    features: number[];
    files: number[];
    path: string;
}

export interface RootDTO {
    id: number;
    manifest_absolute_path: string;
    global: number;
    entities: number[];
    features: number[];
    files: number[];
}

export interface RootRelationshipDTO {
    id: number;
    field: RootRelationshipField;
    right_ids: number[];
}

/**
 * Root service for interacting with the Tauri backend
 */
export const rootService = {
    /**
     * Create a new root
     * @param dto The root data to create
     * @returns The created root
     */
    createRoot: (dto: CreateRootDTO): Promise<RootDTO> => {
        return invokeCommand<RootDTO>("create_root", {dto});
    },

    /**
     * Create multiple roots
     * @param dtos The root data to create
     * @returns The created roots
     */
    createRootMulti: (dtos: CreateRootDTO[]): Promise<RootDTO[]> => {
        return invokeCommand<RootDTO[]>("create_root_multi", {dtos});
    },

    /**
     * Get a root by ID
     * @param id The root ID
     * @returns The root or null if not found
     */
    getRoot: (id: number): Promise<RootDTO | null> => {
        return invokeCommand<RootDTO | null>("get_root", {id});
    },

    /**
     * Get multiple roots by IDs
     * @param ids The root IDs
     * @returns Array of roots (null for roots not found)
     */
    getRootMulti: (ids: number[]): Promise<(RootDTO | null)[]> => {
        return invokeCommand<(RootDTO | null)[]>("get_root_multi", {ids});
    },

    /**
     * Update a root
     * @param dto The root data to update
     * @returns The updated root
     */
    updateRoot: (dto: RootDTO): Promise<RootDTO> => {
        return invokeCommand<RootDTO>("update_root", {dto});
    },

    /**
     * Update multiple roots
     * @param dtos The root data to update
     * @returns The updated roots
     */
    updateRootMulti: (dtos: RootDTO[]): Promise<RootDTO[]> => {
        return invokeCommand<RootDTO[]>("update_root_multi", {dtos});
    },

    /**
     * Remove a root
     * @param id The root ID to remove
     */
    removeRoot: (id: number): Promise<void> => {
        return invokeCommand<void>("remove_root", {id});
    },

    /**
     * Remove multiple roots
     * @param ids The root IDs to remove
     */
    removeRootMulti: (ids: number[]): Promise<void> => {
        return invokeCommand<void>("remove_root_multi", {ids});
    },

    /**
     * Get a relationship for a root
     * @param id The root ID
     * @param field The relationship field
     * @returns Array of related entity IDs
     */
    getRootRelationship: (id: number, field: RootRelationshipField): Promise<number[]> => {
        return invokeCommand<number[]>("get_root_relationship", {id, field});
    },

    /**
     * Set a relationship for a root
     * @param dto The relationship data
     */
    setRootRelationship: (dto: RootRelationshipDTO): Promise<void> => {
        return invokeCommand<void>("set_root_relationship", {dto});
    }
};