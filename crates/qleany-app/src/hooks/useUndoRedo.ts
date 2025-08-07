import { useCallback, useEffect, useState } from 'react';
import { undoRedoService } from '../services/undo-redo-service';
import { error } from '@tauri-apps/plugin-log';

/**
 * Custom hook for undo/redo functionality
 * 
 * This hook provides state and functions for undo/redo operations
 * and automatically checks for undo/redo availability.
 */
export function useUndoRedo() {
  const [canUndoState, setCanUndoState] = useState(false);
  const [canRedoState, setCanRedoState] = useState(false);

  // Check undo/redo availability
  useEffect(() => {
    // Function to check if undo is available
    const checkUndoAvailability = async () => {
      try {
        const undoAvailable = await undoRedoService.canUndo();
        setCanUndoState(undoAvailable);
      } catch (err) {
        error(`Error checking undo availability: ${err}`);
        console.error("Error checking undo availability:", err);
      }
    };

    // Function to check if redo is available
    const checkRedoAvailability = async () => {
      try {
        const redoAvailable = await undoRedoService.canRedo();
        setCanRedoState(redoAvailable);
      } catch (err) {
        error(`Error checking redo availability: ${err}`);
        console.error("Error checking redo availability:", err);
      }
    };

    // Check initially
    checkUndoAvailability();
    checkRedoAvailability();

    // Set up interval to check periodically
    const undoIntervalId = setInterval(checkUndoAvailability, 100);
    const redoIntervalId = setInterval(checkRedoAvailability, 100);

    // Clean up intervals on unmount
    return () => {
      clearInterval(undoIntervalId);
      clearInterval(redoIntervalId);
    };
  }, []);

  // Function to perform undo
  const handleUndo = useCallback(async () => {
    try {
      await undoRedoService.undo();
      
      // Update undo/redo availability after performing undo
      const undoAvailable = await undoRedoService.canUndo();
      setCanUndoState(undoAvailable);
      
      const redoAvailable = await undoRedoService.canRedo();
      setCanRedoState(redoAvailable);
    } catch (err) {
      error(`Error performing undo: ${err}`);
      console.error("Error performing undo:", err);
    }
  }, []);

  // Function to perform redo
  const handleRedo = useCallback(async () => {
    try {
      await undoRedoService.redo();
      
      // Update undo/redo availability after performing redo
      const undoAvailable = await undoRedoService.canUndo();
      setCanUndoState(undoAvailable);
      
      const redoAvailable = await undoRedoService.canRedo();
      setCanRedoState(redoAvailable);
    } catch (err) {
      error(`Error performing redo: ${err}`);
      console.error("Error performing redo:", err);
    }
  }, []);

  // Return state and functions
  return {
    canUndo: canUndoState,
    canRedo: canRedoState,
    undo: handleUndo,
    redo: handleRedo
  };
}