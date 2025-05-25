import {invoke} from "@tauri-apps/api/core";

/**
 * Undoes the most recent command.
 * @returns A promise that resolves when the undo operation is complete
 */
export async function undo(): Promise<void> {
    return await invoke("undo");
}

/**
 * Redoes the most recently undone command.
 * @returns A promise that resolves when the redo operation is complete
 */
export async function redo(): Promise<void> {
    return await invoke("redo");
}

/**
 * Checks if there are commands that can be undone.
 * @returns A promise that resolves to true if there are commands that can be undone
 */
export async function canUndo(): Promise<boolean> {
    return await invoke("can_undo");
}

/**
 * Checks if there are commands that can be redone.
 * @returns A promise that resolves to true if there are commands that can be redone
 */
export async function canRedo(): Promise<boolean> {
    return await invoke("can_redo");
}

/**
 * Begins a composite command group.
 * All commands added between beginComposite and endComposite will be treated as a single command.
 * @returns A promise that resolves when the operation is complete
 */
export async function beginComposite(): Promise<void> {
    return await invoke("begin_composite");
}

/**
 * Ends the current composite command group.
 * @returns A promise that resolves when the operation is complete
 */
export async function endComposite(): Promise<void> {
    return await invoke("end_composite");
}