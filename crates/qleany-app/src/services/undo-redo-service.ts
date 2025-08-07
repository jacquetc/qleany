import { invokeCommand } from './tauri-api';

/**
 * Undo-redo service for interacting with the Tauri backend
 */
export const undoRedoService = {
  /**
   * Undoes the most recent command
   * @returns A promise that resolves when the undo operation is complete
   */
  undo: (): Promise<void> => {
    return invokeCommand<void>("undo");
  },

  /**
   * Redoes the most recently undone command
   * @returns A promise that resolves when the redo operation is complete
   */
  redo: (): Promise<void> => {
    return invokeCommand<void>("redo");
  },

  /**
   * Checks if there are commands that can be undone
   * @returns A promise that resolves to true if there are commands that can be undone
   */
  canUndo: (): Promise<boolean> => {
    return invokeCommand<boolean>("can_undo");
  },

  /**
   * Checks if there are commands that can be redone
   * @returns A promise that resolves to true if there are commands that can be redone
   */
  canRedo: (): Promise<boolean> => {
    return invokeCommand<boolean>("can_redo");
  },

  /**
   * Begins a composite command group
   * All commands added between beginComposite and endComposite will be treated as a single command
   * @returns A promise that resolves when the operation is complete
   */
  beginComposite: (): Promise<void> => {
    return invokeCommand<void>("begin_composite");
  },

  /**
   * Ends the current composite command group
   * @returns A promise that resolves when the operation is complete
   */
  endComposite: (): Promise<void> => {
    return invokeCommand<void>("end_composite");
  }
};