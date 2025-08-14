import {invokeCommand} from './tauri-api';

/**
 * Types for rust file generation DTOs
 */
export interface ListRustFilesDTO {
    only_existing: boolean;
}

export interface GenerateRustFilesDTO {
    file_ids: number[];
    root_path: string;
    prefix: string;
}

export interface GenerateRustFilesResultDTO {
    files: string[];
    timestamp: string;
}

export interface GenerateRustCodeDTO {
    file_id: number;
}

export interface GenerateRustCodeReturnDTO {
    generated_code: string;
    timestamp: string;
}

/**
 * Rust file generation service for interacting with the Tauri backend
 */
export const rustFileGenerationService = {
    /**
     * List available rust files
     * @param dto The list options
     * @returns A promise that resolves when the list operation is complete
     */
    listRustFiles: (dto: ListRustFilesDTO): Promise<void> => {
        return invokeCommand<void>("list_rust_files", {dto});
    },

    /**
     * Generate rust code for a file
     * @param dto The file ID
     * @returns A promise that resolves to the generated code
     */
    generateRustCode: (dto: GenerateRustCodeDTO): Promise<GenerateRustCodeReturnDTO> => {
        return invokeCommand<GenerateRustCodeReturnDTO>("generate_rust_code", {dto});
    },

    /**
     * Generate rust files
     * @param dto The generation options
     * @returns A promise that resolves to the operation ID
     */
    generateRustFiles: (dto: GenerateRustFilesDTO): Promise<string> => {
        return invokeCommand<string>("generate_rust_files", {dto});
    },

    /**
     * Get the result of a generate rust files operation
     * @param operationId The operation ID
     * @returns A promise that resolves to the generation result or null if not complete
     */
    getGenerateRustFilesResult: (operationId: string): Promise<GenerateRustFilesResultDTO | null> => {
        return invokeCommand<GenerateRustFilesResultDTO | null>("get_generate_rust_files_result", {operationId});
    }
};