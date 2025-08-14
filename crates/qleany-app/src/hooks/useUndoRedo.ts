import { useCallback, useEffect, useState } from 'react';
import { undoRedoService } from '../services/undo-redo-service';
import { error } from '@tauri-apps/plugin-log';

/**
 * Custom hook for undo/redo functionality
 * 
 * This hook provides state and functions for undo/redo operations
 * and automatically stays in sync with backend events.
 */
export function useUndoRedo() {
  const [canUndoState, setCanUndoState] = useState(false);
  const [canRedoState, setCanRedoState] = useState(false);

  // Keep canUndo/canRedo in sync using events
  useEffect(() => {
    let unsubAll: (() => Promise<void>) | undefined;

    const refreshStates = async () => {
      try {
        const [undoAvailable, redoAvailable] = await Promise.all([
          undoRedoService.canUndo(),
          undoRedoService.canRedo(),
        ]);
        setCanUndoState(undoAvailable);
        setCanRedoState(redoAvailable);
      } catch (err) {
        error(`Error refreshing undo/redo availability: ${err}`);
        console.error('Error refreshing undo/redo availability:', err);
      }
    };

    // Initial fetch
    refreshStates();

    // Subscribe to undo/redo related events and refresh on each
    try {
      unsubAll = undoRedoService.subscribeToUndoRedoEvents({
        onUndone: () => void refreshStates(),
        onRedone: () => void refreshStates(),
        onBeginComposite: () => void refreshStates(),
        onEndComposite: () => void refreshStates(),
      });
    } catch (err) {
      error(`Error subscribing to undo/redo events: ${err}`);
      console.error('Error subscribing to undo/redo events:', err);
    }

    return () => {
      if (unsubAll) {
        unsubAll().catch((e) => {
          error(`Error unsubscribing from undo/redo events: ${e}`);
          console.error('Error unsubscribing from undo/redo events:', e);
        });
      }
    };
  }, []);

  // Function to perform undo
  const handleUndo = useCallback(async () => {
    try {
      await undoRedoService.undo();
      // State will be updated by the emitted event, but refresh defensively
      const [undoAvailable, redoAvailable] = await Promise.all([
        undoRedoService.canUndo(),
        undoRedoService.canRedo(),
      ]);
      setCanUndoState(undoAvailable);
      setCanRedoState(redoAvailable);
    } catch (err) {
      error(`Error performing undo: ${err}`);
      console.error('Error performing undo:', err);
    }
  }, []);

  // Function to perform redo
  const handleRedo = useCallback(async () => {
    try {
      await undoRedoService.redo();
      // State will be updated by the emitted event, but refresh defensively
      const [undoAvailable, redoAvailable] = await Promise.all([
        undoRedoService.canUndo(),
        undoRedoService.canRedo(),
      ]);
      setCanUndoState(undoAvailable);
      setCanRedoState(redoAvailable);
    } catch (err) {
      error(`Error performing redo: ${err}`);
      console.error('Error performing redo:', err);
    }
  }, []);

  // Return state and functions
  return {
    canUndo: canUndoState,
    canRedo: canRedoState,
    undo: handleUndo,
    redo: handleRedo,
  };
}