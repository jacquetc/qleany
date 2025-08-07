import { invoke } from "@tauri-apps/api/core";
import { listen, UnlistenFn } from "@tauri-apps/api/event";

/**
 * Type-safe wrapper for Tauri invoke calls
 * @param command The Tauri command to invoke
 * @param args The arguments to pass to the command
 * @returns The result of the command
 */
export async function invokeCommand<T>(command: string, args?: Record<string, unknown>): Promise<T> {
  try {
    return await invoke<T>(command, args);
  } catch (error) {
    console.error(`Error invoking command ${command}:`, error);
    throw error;
  }
}

/**
 * Type for event callback functions
 */
export type EventCallback<T = unknown> = (data: T) => void;

/**
 * Subscribe to a Tauri event
 * @param event The event name to subscribe to
 * @param callback The callback to call when the event is emitted
 * @returns A promise that resolves to an unsubscribe function
 */
export async function subscribeToEvent<T = unknown>(
  event: string, 
  callback: EventCallback<T>
): Promise<UnlistenFn> {
  try {
    return await listen<T>(event, (event) => {
      try {
        callback(event.payload);
      } catch (error) {
        console.error(`Error in event handler for ${event}:`, error);
      }
    });
  } catch (error) {
    console.error(`Error subscribing to event ${event}:`, error);
    throw error;
  }
}

/**
 * Helper to create a typed event subscription
 * @param event The event name to subscribe to
 * @param callback The callback to call when the event is emitted
 * @returns A promise that resolves to an unsubscribe function
 */
export function createEventSubscription<T = unknown>(
  event: string,
  callback: EventCallback<T>
): Promise<UnlistenFn> {
  return subscribeToEvent<T>(event, callback);
}

/**
 * Helper to handle multiple event subscriptions
 * @param subscriptions Array of promises that resolve to unsubscribe functions
 * @returns A function that unsubscribes from all events
 */
export function createUnsubscribeFn(subscriptions: Promise<UnlistenFn>[]): () => Promise<void> {
  return async () => {
    try {
      const unsubFns = await Promise.all(subscriptions);
      await Promise.all(unsubFns.map(fn => fn()));
    } catch (error) {
      console.error("Error unsubscribing from events:", error);
    }
  };
}