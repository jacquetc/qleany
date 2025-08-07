import { invokeCommand } from './tauri-api';

/**
 * Enum for DTO relationship fields
 */
export enum DtoRelationshipField {
  Fields = "Fields",
}

/**
 * Types for DTO-related DTOs
 */
export interface CreateDtoDTO {
  name: string;
  fields: number[];
}

export interface DtoDTO {
  id: number;
  name: string;
  fields: number[];
}

export interface DtoRelationshipDTO {
  id: number;
  field: DtoRelationshipField;
  right_ids: number[];
}

/**
 * DTO service for interacting with the Tauri backend
 */
export const dtoService = {
  /**
   * Create a new DTO
   * @param dto The DTO data to create
   * @returns The created DTO
   */
  createDto: (dto: CreateDtoDTO): Promise<DtoDTO> => {
    return invokeCommand<DtoDTO>("create_dto", { dto });
  },

  /**
   * Create multiple DTOs
   * @param dtos The DTO data to create
   * @returns The created DTOs
   */
  createDtoMulti: (dtos: CreateDtoDTO[]): Promise<DtoDTO[]> => {
    return invokeCommand<DtoDTO[]>("create_dto_multi", { dtos });
  },

  /**
   * Get a DTO by ID
   * @param id The DTO ID
   * @returns The DTO or null if not found
   */
  getDto: (id: number): Promise<DtoDTO | null> => {
    return invokeCommand<DtoDTO | null>("get_dto", { id });
  },

  /**
   * Get multiple DTOs by IDs
   * @param ids The DTO IDs
   * @returns Array of DTOs (null for DTOs not found)
   */
  getDtoMulti: (ids: number[]): Promise<(DtoDTO | null)[]> => {
    return invokeCommand<(DtoDTO | null)[]>("get_dto_multi", { ids });
  },

  /**
   * Update a DTO
   * @param dto The DTO data to update
   * @returns The updated DTO
   */
  updateDto: (dto: DtoDTO): Promise<DtoDTO> => {
    return invokeCommand<DtoDTO>("update_dto", { dto });
  },

  /**
   * Update multiple DTOs
   * @param dtos The DTO data to update
   * @returns The updated DTOs
   */
  updateDtoMulti: (dtos: DtoDTO[]): Promise<DtoDTO[]> => {
    return invokeCommand<DtoDTO[]>("update_dto_multi", { dtos });
  },

  /**
   * Remove a DTO
   * @param id The DTO ID to remove
   */
  removeDto: (id: number): Promise<void> => {
    return invokeCommand<void>("remove_dto", { id });
  },

  /**
   * Remove multiple DTOs
   * @param ids The DTO IDs to remove
   */
  removeDtoMulti: (ids: number[]): Promise<void> => {
    return invokeCommand<void>("remove_dto_multi", { ids });
  },

  /**
   * Get a relationship for a DTO
   * @param id The DTO ID
   * @param field The relationship field
   * @returns Array of related entity IDs
   */
  getDtoRelationship: (id: number, field: DtoRelationshipField): Promise<number[]> => {
    return invokeCommand<number[]>("get_dto_relationship", { id, field });
  },

  /**
   * Set a relationship for a DTO
   * @param dto The relationship data
   */
  setDtoRelationship: (dto: DtoRelationshipDTO): Promise<void> => {
    return invokeCommand<void>("set_dto_relationship", { dto });
  }
};