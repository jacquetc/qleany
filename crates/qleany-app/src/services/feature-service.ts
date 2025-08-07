import { invokeCommand } from './tauri-api';

/**
 * Enum for feature relationship fields
 */
export enum FeatureRelationshipField {
  UseCases = "UseCases",
}

/**
 * Types for feature-related DTOs
 */
export interface CreateFeatureDTO {
  name: string;
  use_cases: number[];
}

export interface FeatureDTO {
  id: number;
  name: string;
  use_cases: number[];
}

export interface FeatureRelationshipDTO {
  id: number;
  field: FeatureRelationshipField;
  right_ids: number[];
}

/**
 * Feature service for interacting with the Tauri backend
 */
export const featureService = {
  /**
   * Create a new feature
   * @param dto The feature data to create
   * @returns The created feature
   */
  createFeature: (dto: CreateFeatureDTO): Promise<FeatureDTO> => {
    return invokeCommand<FeatureDTO>("create_feature", { dto });
  },

  /**
   * Create multiple features
   * @param dtos The feature data to create
   * @returns The created features
   */
  createFeatureMulti: (dtos: CreateFeatureDTO[]): Promise<FeatureDTO[]> => {
    return invokeCommand<FeatureDTO[]>("create_feature_multi", { dtos });
  },

  /**
   * Get a feature by ID
   * @param id The feature ID
   * @returns The feature or null if not found
   */
  getFeature: (id: number): Promise<FeatureDTO | null> => {
    return invokeCommand<FeatureDTO | null>("get_feature", { id });
  },

  /**
   * Get multiple features by IDs
   * @param ids The feature IDs
   * @returns Array of features (null for features not found)
   */
  getFeatureMulti: (ids: number[]): Promise<(FeatureDTO | null)[]> => {
    return invokeCommand<(FeatureDTO | null)[]>("get_feature_multi", { ids });
  },

  /**
   * Update a feature
   * @param dto The feature data to update
   * @returns The updated feature
   */
  updateFeature: (dto: FeatureDTO): Promise<FeatureDTO> => {
    return invokeCommand<FeatureDTO>("update_feature", { dto });
  },

  /**
   * Update multiple features
   * @param dtos The feature data to update
   * @returns The updated features
   */
  updateFeatureMulti: (dtos: FeatureDTO[]): Promise<FeatureDTO[]> => {
    return invokeCommand<FeatureDTO[]>("update_feature_multi", { dtos });
  },

  /**
   * Remove a feature
   * @param id The feature ID to remove
   */
  removeFeature: (id: number): Promise<void> => {
    return invokeCommand<void>("remove_feature", { id });
  },

  /**
   * Remove multiple features
   * @param ids The feature IDs to remove
   */
  removeFeatureMulti: (ids: number[]): Promise<void> => {
    return invokeCommand<void>("remove_feature_multi", { ids });
  },

  /**
   * Get a relationship for a feature
   * @param id The feature ID
   * @param field The relationship field
   * @returns Array of related entity IDs
   */
  getFeatureRelationship: (id: number, field: FeatureRelationshipField): Promise<number[]> => {
    return invokeCommand<number[]>("get_feature_relationship", { id, field });
  },

  /**
   * Set a relationship for a feature
   * @param dto The relationship data
   */
  setFeatureRelationship: (dto: FeatureRelationshipDTO): Promise<void> => {
    return invokeCommand<void>("set_feature_relationship", { dto });
  }
};