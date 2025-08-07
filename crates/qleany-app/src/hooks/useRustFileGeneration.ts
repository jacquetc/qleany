import { useCallback, useState } from 'react';
import { 
  ListRustFilesDTO, 
  GenerateRustFilesDTO, 
  GenerateRustFilesResultDTO,
  rustFileGenerationService 
} from '../services/rust-file-generation-service';
import { error, info } from '@tauri-apps/plugin-log';

/**
 * Custom hook for rust file generation functionality
 * 
 * This hook provides functions for listing and generating Rust files
 */
export function useRustFileGeneration() {
  const [isLoading, setIsLoading] = useState(false);
  const [operationError, setOperationError] = useState<Error | null>(null);
  const [operationId, setOperationId] = useState<string | null>(null);
  const [generationResult, setGenerationResult] = useState<GenerateRustFilesResultDTO | null>(null);

  /**
   * List available rust files
   * @param onlyExisting Whether to only list existing files
   */
  const listRustFiles = useCallback(async (onlyExisting: boolean = false) => {
    setIsLoading(true);
    setOperationError(null);
    
    try {
      const dto: ListRustFilesDTO = {
        only_existing: onlyExisting
      };
      
      await rustFileGenerationService.listRustFiles(dto);
      info("Rust files listed successfully");
    } catch (err) {
      const errorMessage = `Error listing rust files: ${err}`;
      error(errorMessage);
      setOperationError(err instanceof Error ? err : new Error(errorMessage));
      throw err;
    } finally {
      setIsLoading(false);
    }
  }, []);

  /**
   * Generate rust files
   * @param fileIds IDs of files to generate
   * @param rootPath Root path for generation
   * @param prefix Prefix for generated files
   */
  const generateRustFiles = useCallback(async (fileIds: number[], rootPath: string, prefix: string) => {
    setIsLoading(true);
    setOperationError(null);
    setOperationId(null);
    setGenerationResult(null);
    
    try {
      const dto: GenerateRustFilesDTO = {
        file_ids: fileIds,
        root_path: rootPath,
        prefix: prefix
      };
      
      const id = await rustFileGenerationService.generateRustFiles(dto);
      setOperationId(id);
      info(`Rust file generation started with operation ID: ${id}`);
      return id;
    } catch (err) {
      const errorMessage = `Error generating rust files: ${err}`;
      error(errorMessage);
      setOperationError(err instanceof Error ? err : new Error(errorMessage));
      throw err;
    } finally {
      setIsLoading(false);
    }
  }, []);

  /**
   * Check the result of a generate operation
   * @param id Operation ID to check
   */
  const checkGenerationResult = useCallback(async (id: string) => {
    setIsLoading(true);
    setOperationError(null);
    
    try {
      const result = await rustFileGenerationService.getGenerateRustFilesResult(id);
      setGenerationResult(result);
      return result;
    } catch (err) {
      const errorMessage = `Error checking generation result: ${err}`;
      error(errorMessage);
      setOperationError(err instanceof Error ? err : new Error(errorMessage));
      throw err;
    } finally {
      setIsLoading(false);
    }
  }, []);

  return {
    isLoading,
    operationError,
    operationId,
    generationResult,
    listRustFiles,
    generateRustFiles,
    checkGenerationResult
  };
}