import { invokeCommand, createEventSubscription, createUnsubscribeFn, EventCallback } from './tauri-api';
import { UnlistenFn } from '@tauri-apps/api/event';

/**
 * Undo-redo service for interacting with the Tauri backend
 */
export const UndoRedoEvents = {
  UNDONE: 'undo_redo_undone',
  REDONE: 'undo_redo_redone',
  BEGIN_COMPOSITE: 'undo_redo_begincomposite',
  END_COMPOSITE: 'undo_redo_endcomposite',
} as const;

export type UndoRedoEventName = typeof UndoRedoEvents[keyof typeof UndoRedoEvents];

export interface UndoRedoEventPayload {
  ids: number[];
  data?: string;
}

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
  },

  // Event subscriptions
  onUndone: (callback: EventCallback<UndoRedoEventPayload | void>) => {
    return createEventSubscription<UndoRedoEventPayload | void>(UndoRedoEvents.UNDONE, callback);
  },

  onRedone: (callback: EventCallback<UndoRedoEventPayload | void>) => {
    return createEventSubscription<UndoRedoEventPayload | void>(UndoRedoEvents.REDONE, callback);
  },

  onBeginComposite: (callback: EventCallback<UndoRedoEventPayload | void>) => {
    return createEventSubscription<UndoRedoEventPayload | void>(UndoRedoEvents.BEGIN_COMPOSITE, callback);
  },

  onEndComposite: (callback: EventCallback<UndoRedoEventPayload | void>) => {
    return createEventSubscription<UndoRedoEventPayload | void>(UndoRedoEvents.END_COMPOSITE, callback);
  },

  /**
   * Subscribe to multiple undo/redo events at once
   * @param callbacks Object containing callbacks for different events
   * @returns A function that unsubscribes from all events
   */
  subscribeToUndoRedoEvents: (callbacks: {
    onUndone?: EventCallback<UndoRedoEventPayload | void>;
    onRedone?: EventCallback<UndoRedoEventPayload | void>;
    onBeginComposite?: EventCallback<UndoRedoEventPayload | void>;
    onEndComposite?: EventCallback<UndoRedoEventPayload | void>;
  }): () => Promise<void> => {
    const subscriptions: Promise<UnlistenFn>[] = [];

    if (callbacks.onUndone) {
      subscriptions.push(undoRedoService.onUndone(callbacks.onUndone));
    }
    if (callbacks.onRedone) {
      subscriptions.push(undoRedoService.onRedone(callbacks.onRedone));
    }
    if (callbacks.onBeginComposite) {
      subscriptions.push(undoRedoService.onBeginComposite(callbacks.onBeginComposite));
    }
    if (callbacks.onEndComposite) {
      subscriptions.push(undoRedoService.onEndComposite(callbacks.onEndComposite));
    }

    return createUnsubscribeFn(subscriptions);
  }
};