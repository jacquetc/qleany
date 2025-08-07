import { invokeCommand } from './tauri-api';

/**
 * Enum for use case relationship fields
 */
export enum UseCaseRelationshipField {
  Entities = "Entities",
  DtoIn = "DtoIn",
  DtoOut = "DtoOut"
}

/**
 * Types for use case-related DTOs
 */
export interface CreateUseCaseDTO {
  name: string;
  validator: boolean;
  entities: number[];
  undoable: boolean;
  dto_in: number | null;
  dto_out: number | null;
}

export interface UseCaseDTO {
  id: number;
  name: string;
  validator: boolean;
  entities: number[];
  undoable: boolean;
  dto_in: number | null;
  dto_out: number | null;
}

export interface UseCaseRelationshipDTO {
  id: number;
  field: UseCaseRelationshipField;
  right_ids: number[];
}

/**
 * Use Case service for interacting with the Tauri backend
 */
export const useCaseService = {
  /**
   * Create a new use case
   * @param dto The use case data to create
   * @returns The created use case
   */
  createUseCase: (dto: CreateUseCaseDTO): Promise<UseCaseDTO> => {
    return invokeCommand<UseCaseDTO>("create_use_case", { dto });
  },

  /**
   * Create multiple use cases
   * @param dtos The use case data to create
   * @returns The created use cases
   */
  createUseCaseMulti: (dtos: CreateUseCaseDTO[]): Promise<UseCaseDTO[]> => {
    return invokeCommand<UseCaseDTO[]>("create_use_case_multi", { dtos });
  },

  /**
   * Get a use case by ID
   * @param id The use case ID
   * @returns The use case or null if not found
   */
  getUseCase: (id: number): Promise<UseCaseDTO | null> => {
    return invokeCommand<UseCaseDTO | null>("get_use_case", { id });
  },

  /**
   * Get multiple use cases by IDs
   * @param ids The use case IDs
   * @returns Array of use cases (null for use cases not found)
   */
  getUseCaseMulti: (ids: number[]): Promise<(UseCaseDTO | null)[]> => {
    return invokeCommand<(UseCaseDTO | null)[]>("get_use_case_multi", { ids });
  },

  /**
   * Update a use case
   * @param dto The use case data to update
   * @returns The updated use case
   */
  updateUseCase: (dto: UseCaseDTO): Promise<UseCaseDTO> => {
    return invokeCommand<UseCaseDTO>("update_use_case", { dto });
  },

  /**
   * Update multiple use cases
   * @param dtos The use case data to update
   * @returns The updated use cases
   */
  updateUseCaseMulti: (dtos: UseCaseDTO[]): Promise<UseCaseDTO[]> => {
    return invokeCommand<UseCaseDTO[]>("update_use_case_multi", { dtos });
  },

  /**
   * Remove a use case
   * @param id The use case ID to remove
   */
  removeUseCase: (id: number): Promise<void> => {
    return invokeCommand<void>("remove_use_case", { id });
  },

  /**
   * Remove multiple use cases
   * @param ids The use case IDs to remove
   */
  removeUseCaseMulti: (ids: number[]): Promise<void> => {
    return invokeCommand<void>("remove_use_case_multi", { ids });
  },

  /**
   * Get a relationship for a use case
   * @param id The use case ID
   * @param field The relationship field
   * @returns Array of related entity IDs
   */
  getUseCaseRelationship: (id: number, field: UseCaseRelationshipField): Promise<number[]> => {
    return invokeCommand<number[]>("get_use_case_relationship", { id, field });
  },

  /**
   * Set a relationship for a use case
   * @param dto The relationship data
   */
  setUseCaseRelationship: (dto: UseCaseRelationshipDTO): Promise<void> => {
    return invokeCommand<void>("set_use_case_relationship", { dto });
  }
};