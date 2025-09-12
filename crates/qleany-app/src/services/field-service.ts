import {invokeCommand} from './tauri-api';

/**
 * Enum for field types
 */
export enum FieldType {
    Boolean = "Boolean",
    Integer = "Integer",
    UInteger = "UInteger",
    Float = "Float",
    String = "String",
    Uuid = "Uuid",
    DateTime = "DateTime",
    Enum = "Enum",
    Entity = "Entity",
}

/**
 * Types for field-related DTOs
 */
export interface CreateFieldDTO {
    name: string;
    field_type: FieldType;
    entity: number | null;
    is_nullable: boolean;
    is_primary_key: boolean;
    is_list: boolean;
    single: boolean;
    strong: boolean;
    ordered: boolean;
    list_model: boolean;
    list_model_displayed_field: string | null;
    enum_name: string | null;
    enum_values: string[] | null;
}

export interface FieldDTO {
    id: number;
    name: string;
    field_type: FieldType;
    entity: number | null;
    is_nullable: boolean;
    is_primary_key: boolean;
    is_list: boolean;
    single: boolean;
    strong: boolean;
    ordered: boolean;
    list_model: boolean;
    list_model_displayed_field: string | null;
    enum_name: string | null;
    enum_values: string[] | null;
}

/**
 * Field service for interacting with the Tauri backend
 */
export const fieldService = {
    /**
     * Create a new field
     * @param dto The field data to create
     * @returns The created field
     */
    createField: (dto: CreateFieldDTO): Promise<FieldDTO> => {
        return invokeCommand<FieldDTO>("create_field", {dto});
    },

    /**
     * Create multiple fields
     * @param dtos The field data to create
     * @returns The created fields
     */
    createFieldMulti: (dtos: CreateFieldDTO[]): Promise<FieldDTO[]> => {
        return invokeCommand<FieldDTO[]>("create_field_multi", {dtos});
    },

    /**
     * Get a field by ID
     * @param id The field ID
     * @returns The field or null if not found
     */
    getField: (id: number): Promise<FieldDTO | null> => {
        return invokeCommand<FieldDTO | null>("get_field", {id});
    },

    /**
     * Get multiple fields by IDs
     * @param ids The field IDs
     * @returns Array of fields (null for fields not found)
     */
    getFieldMulti: (ids: number[]): Promise<(FieldDTO | null)[]> => {
        return invokeCommand<(FieldDTO | null)[]>("get_field_multi", {ids});
    },

    /**
     * Update a field
     * @param dto The field data to update
     * @returns The updated field
     */
    updateField: (dto: FieldDTO): Promise<FieldDTO> => {
        return invokeCommand<FieldDTO>("update_field", {dto});
    },

    /**
     * Update multiple fields
     * @param dtos The field data to update
     * @returns The updated fields
     */
    updateFieldMulti: (dtos: FieldDTO[]): Promise<FieldDTO[]> => {
        return invokeCommand<FieldDTO[]>("update_field_multi", {dtos});
    },

    /**
     * Remove a field
     * @param id The field ID to remove
     */
    removeField: (id: number): Promise<void> => {
        return invokeCommand<void>("remove_field", {id});
    },

    /**
     * Remove multiple fields
     * @param ids The field IDs to remove
     */
    removeFieldMulti: (ids: number[]): Promise<void> => {
        return invokeCommand<void>("remove_field_multi", {ids});
    },

    /**
     * Get a relationship for a field
     * @param id The field ID
     * @param field The relationship field
     * @returns Array of related entity IDs
     */
    getFieldRelationship: (id: number, field: string): Promise<number[]> => {
        return invokeCommand<number[]>("get_field_relationship", {id, field});
    },

    /**
     * Set a relationship for a field
     * @param dto The relationship data
     */
    setFieldRelationship: (dto: { id: number, field: string, right_ids: number[] }): Promise<void> => {
        return invokeCommand<void>("set_field_relationship", {dto});
    }
};