import { invokeCommand } from './tauri-api';

/**
 * Types for global-related DTOs
 */
export interface CreateGlobalDTO {
  language: string;
  application_name: string;
  organisation_name: string;
  organisation_domain: string;
  prefix_path: string;
}

export interface GlobalDTO {
  id: number;
  language: string;
  application_name: string;
  organisation_name: string;
  organisation_domain: string;
  prefix_path: string;
}

/**
 * Global service for interacting with the Tauri backend
 */
export const globalService = {
  /**
   * Create a new global configuration
   * @param dto The global data to create
   * @returns The created global configuration
   */
  createGlobal: (dto: CreateGlobalDTO): Promise<GlobalDTO> => {
    return invokeCommand<GlobalDTO>("create_global", { dto });
  },

  /**
   * Create multiple global configurations
   * @param dtos The global data to create
   * @returns The created global configurations
   */
  createGlobalMulti: (dtos: CreateGlobalDTO[]): Promise<GlobalDTO[]> => {
    return invokeCommand<GlobalDTO[]>("create_global_multi", { dtos });
  },

  /**
   * Get a global configuration by ID
   * @param id The global ID
   * @returns The global configuration or null if not found
   */
  getGlobal: (id: number): Promise<GlobalDTO | null> => {
    return invokeCommand<GlobalDTO | null>("get_global", { id });
  },

  /**
   * Get multiple global configurations by IDs
   * @param ids The global IDs
   * @returns Array of global configurations (null for configurations not found)
   */
  getGlobalMulti: (ids: number[]): Promise<(GlobalDTO | null)[]> => {
    return invokeCommand<(GlobalDTO | null)[]>("get_global_multi", { ids });
  },

  /**
   * Update a global configuration
   * @param dto The global data to update
   * @returns The updated global configuration
   */
  updateGlobal: (dto: GlobalDTO): Promise<GlobalDTO> => {
    return invokeCommand<GlobalDTO>("update_global", { dto });
  },

  /**
   * Update multiple global configurations
   * @param dtos The global data to update
   * @returns The updated global configurations
   */
  updateGlobalMulti: (dtos: GlobalDTO[]): Promise<GlobalDTO[]> => {
    return invokeCommand<GlobalDTO[]>("update_global_multi", { dtos });
  },

  /**
   * Remove a global configuration
   * @param id The global ID to remove
   */
  removeGlobal: (id: number): Promise<void> => {
    return invokeCommand<void>("remove_global", { id });
  },

  /**
   * Remove multiple global configurations
   * @param ids The global IDs to remove
   */
  removeGlobalMulti: (ids: number[]): Promise<void> => {
    return invokeCommand<void>("remove_global_multi", { ids });
  }
};