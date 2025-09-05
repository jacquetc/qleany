import { useCallback } from 'react';
import { RootDTO, CreateRootDTO, RootRelationshipDTO, RootRelationshipField, rootService } from '../services/root-service';
import { error, info } from '@tauri-apps/plugin-log';

import { useMutation, useQuery, useQueryClient } from '@tanstack/react-query';
import {undoRedoService} from "#services/undo-redo-service.ts";

/**
 * Custom hook for fetching and managing root entity data
 *
 * This hook uses React Query to fetch and cache root data,
 * and provides functions for creating, updating, and removing root entities.
 *
 * @param rootId The ID of the root entity to fetch (defaults to 1)
 */
export function useRoot(rootId: number = 1) {
  const queryClient = useQueryClient();

  // Query for fetching root entity
  const rootQuery = useQuery({
    queryKey: ['root', rootId],
    queryFn: async () => {
      try {
        // Get root entity
        const rootData = await rootService.getRoot(rootId);
        return rootData;
      } catch (err) {
        error(`Error fetching root entity: ${err}`);
        throw err;
      }
    },
    staleTime: 1000 * 60 * 5, // 5 minutes
    retry: 1
  });

  // Mutation for creating a new root entity
  const createRootMutation = useMutation({
    mutationFn: async (dto: CreateRootDTO) => {
      try {
        return await rootService.createRoot(dto);
      } catch (err) {
        error(`Error creating root entity: ${err}`);
        throw err;
      }
    },
    onSuccess: () => {
      // Invalidate queries to refetch data
      queryClient.invalidateQueries({ queryKey: ['root'] });
      info("Root entity created successfully");
    }
  });

  // Mutation for updating a root entity
  const updateRootMutation = useMutation({
    mutationFn: async (root: RootDTO) => {
      try {
        return await rootService.updateRoot(root);
      } catch (err) {
        error(`Error updating root entity: ${err}`);
        throw err;
      }
    },
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['root'] });
      info("Root entity updated successfully");
    }
  });

  // Mutation for removing a root entity
  const removeRootMutation = useMutation({
    mutationFn: async (id: number) => {
      try {
        await rootService.removeRoot(id);
      } catch (err) {
        error(`Error removing root entity: ${err}`);
        throw err;
      }
    },
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['root'] });
      info("Root entity removed successfully");
    }
  });

  // Mutation for setting a relationship on a root entity
  const setRelationshipMutation = useMutation({
    mutationFn: async (relationshipDto: RootRelationshipDTO) => {
      try {
        await rootService.setRootRelationship(relationshipDto);
      } catch (err) {
        error(`Error setting root relationship: ${err}`);
        throw err;
      }
    },
    onSuccess: (_, variables) => {
      queryClient.invalidateQueries({ queryKey: ['root', variables.id] });
      info("Root relationship set successfully");
    }
  });

  // Function to create a new root entity
  const createRoot = useCallback((dto: CreateRootDTO) => {
    createRootMutation.mutate(dto);
  }, [createRootMutation]);

  // Function to update a root entity
  const updateRoot = useCallback((root: RootDTO) => {
    updateRootMutation.mutate(root);
  }, [updateRootMutation]);

  // Function to remove a root entity
  const removeRoot = useCallback((id: number) => {
    removeRootMutation.mutate(id);
  }, [removeRootMutation]);

  // Function to set a relationship on a root entity
  const setRelationship = useCallback((relationshipDto: RootRelationshipDTO) => {
    setRelationshipMutation.mutate(relationshipDto);
  }, [setRelationshipMutation]);

  // Function to get a relationship from a root entity
  const getRelationship = useCallback(async (field: RootRelationshipField): Promise<number[]> => {
    if (!rootId) {
      return [];
    }
    
    try {
      return await rootService.getRootRelationship(rootId, field);
    } catch (err) {
      error(`Error getting root relationship: ${err}`);
      throw err;
    }
  }, [rootId]);

  return {
    root: rootQuery.data,
    isLoading: rootQuery.isLoading,
    error: rootQuery.error,
    createRoot,
    updateRoot,
    removeRoot,
    setRelationship,
    getRelationship,
    refetch: rootQuery.refetch
  };
}