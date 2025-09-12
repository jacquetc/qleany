import {invokeCommand} from './tauri-api';

/**
 * Enum for DTO field types
 */
export enum DtoFieldType {
    Boolean = "Boolean",
    Integer = "Integer",
    UInteger = "UInteger",
    Float = "Float",
    String = "String",
    Uuid = "Uuid",
    DateTime = "DateTime",
}

/**
 * Types for DTO field-related DTOs
 */
export interface CreateDtoFieldDTO {
    name: string;
    field_type: DtoFieldType;
    is_nullable: boolean;
    is_list: boolean;
    enum_name: string | null;
    enum_values: string[] | null;
}

export interface DtoFieldDTO {
    id: number;
    name: string;
    field_type: DtoFieldType;
    is_nullable: boolean;
    is_list: boolean;
    enum_name: string | null;
    enum_values: string[] | null;
}

/**
 * DTO Field service for interacting with the Tauri backend
 */
export const dtoFieldService = {
    /**
     * Create a new DTO field
     * @param dto The DTO field data to create
     * @returns The created DTO field
     */
    createDtoField: (dto: CreateDtoFieldDTO): Promise<DtoFieldDTO> => {
        return invokeCommand<DtoFieldDTO>("create_dto_field", {dto});
    },

    /**
     * Create multiple DTO fields
     * @param dtos The DTO field data to create
     * @returns The created DTO fields
     */
    createDtoFieldMulti: (dtos: CreateDtoFieldDTO[]): Promise<DtoFieldDTO[]> => {
        return invokeCommand<DtoFieldDTO[]>("create_dto_field_multi", {dtos});
    },

    /**
     * Get a DTO field by ID
     * @param id The DTO field ID
     * @returns The DTO field or null if not found
     */
    getDtoField: (id: number): Promise<DtoFieldDTO | null> => {
        return invokeCommand<DtoFieldDTO | null>("get_dto_field", {id});
    },

    /**
     * Get multiple DTO fields by IDs
     * @param ids The DTO field IDs
     * @returns Array of DTO fields (null for DTO fields not found)
     */
    getDtoFieldMulti: (ids: number[]): Promise<(DtoFieldDTO | null)[]> => {
        return invokeCommand<(DtoFieldDTO | null)[]>("get_dto_field_multi", {ids});
    },

    /**
     * Update a DTO field
     * @param dto The DTO field data to update
     * @returns The updated DTO field
     */
    updateDtoField: (dto: DtoFieldDTO): Promise<DtoFieldDTO> => {
        return invokeCommand<DtoFieldDTO>("update_dto_field", {dto});
    },

    /**
     * Update multiple DTO fields
     * @param dtos The DTO field data to update
     * @returns The updated DTO fields
     */
    updateDtoFieldMulti: (dtos: DtoFieldDTO[]): Promise<DtoFieldDTO[]> => {
        return invokeCommand<DtoFieldDTO[]>("update_dto_field_multi", {dtos});
    },

    /**
     * Remove a DTO field
     * @param id The DTO field ID to remove
     */
    removeDtoField: (id: number): Promise<void> => {
        return invokeCommand<void>("remove_dto_field", {id});
    },

    /**
     * Remove multiple DTO fields
     * @param ids The DTO field IDs to remove
     */
    removeDtoFieldMulti: (ids: number[]): Promise<void> => {
        return invokeCommand<void>("remove_dto_field_multi", {ids});
    }
};