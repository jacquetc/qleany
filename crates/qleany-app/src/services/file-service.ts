import { invokeCommand } from './tauri-api';

/**
 * Types for file-related DTOs
 */
export interface CreateFileDTO {
  name: string;
  relative_path: string;
  group: string;
}

export interface FileDTO {
  id: number;
  name: string;
  relative_path: string;
  group: string;
}

/**
 * File service for interacting with the Tauri backend
 */
export const fileService = {
  /**
   * Create a new file
   * @param dto The file data to create
   * @returns The created file
   */
  createFile: (dto: CreateFileDTO): Promise<FileDTO> => {
    return invokeCommand<FileDTO>("create_file", { dto });
  },

  /**
   * Create multiple files
   * @param dtos The file data to create
   * @returns The created files
   */
  createFileMulti: (dtos: CreateFileDTO[]): Promise<FileDTO[]> => {
    return invokeCommand<FileDTO[]>("create_file_multi", { dtos });
  },

  /**
   * Get a file by ID
   * @param id The file ID
   * @returns The file or null if not found
   */
  getFile: (id: number): Promise<FileDTO | null> => {
    return invokeCommand<FileDTO | null>("get_file", { id });
  },

  /**
   * Get multiple files by IDs
   * @param ids The file IDs
   * @returns Array of files (null for files not found)
   */
  getFileMulti: (ids: number[]): Promise<(FileDTO | null)[]> => {
    return invokeCommand<(FileDTO | null)[]>("get_file_multi", { ids });
  },

  /**
   * Update a file
   * @param dto The file data to update
   * @returns The updated file
   */
  updateFile: (dto: FileDTO): Promise<FileDTO> => {
    return invokeCommand<FileDTO>("update_file", { dto });
  },

  /**
   * Update multiple files
   * @param dtos The file data to update
   * @returns The updated files
   */
  updateFileMulti: (dtos: FileDTO[]): Promise<FileDTO[]> => {
    return invokeCommand<FileDTO[]>("update_file_multi", { dtos });
  },

  /**
   * Remove a file
   * @param id The file ID to remove
   */
  removeFile: (id: number): Promise<void> => {
    return invokeCommand<void>("remove_file", { id });
  },

  /**
   * Remove multiple files
   * @param ids The file IDs to remove
   */
  removeFileMulti: (ids: number[]): Promise<void> => {
    return invokeCommand<void>("remove_file_multi", { ids });
  }
};