import {invokeCommand} from './tauri-api';

/**
 * Types for handling manifest DTOs
 */
export interface LoadManifestDTO {
    manifest_path: string;
}

export interface LoadReturnDTO {
    root_id: number;
}

export interface SaveManifestDTO {
    manifest_path: string;
}

/**
 * Handling manifest service for interacting with the Tauri backend
 */
export const handlingManifestService = {
    /**
     * Load a manifest from the specified path
     * @param dto The manifest data with path
     * @returns A promise that resolves when the manifest is loaded
     */
    loadManifest: (dto: LoadManifestDTO): Promise<LoadReturnDTO> => {
        return invokeCommand<LoadReturnDTO>("load_manifest", {dto});
    },

    /**
     * Save a manifest to the specified path
     * @param dto The manifest data with path
     * @returns A promise that resolves when the manifest is saved
     */
    saveManifest: (dto: SaveManifestDTO): Promise<void> => {
        return invokeCommand<void>("save_manifest", {dto});
    }
};