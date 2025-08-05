import {invoke} from "@tauri-apps/api/core";

export enum OperationStatus {
    Running = "Running",
    Completed = "Completed",
    Cancelled = "Cancelled",
    Failed = "Failed",
}

export type OperationProgress = {
    percentage: number;
    message?: string;
}

export type OperationSummary = {
    id: string;
    status: OperationStatus;
    progress: OperationProgress;
}

export async function getOperationStatus(operationId: string): Promise<OperationStatus | null> {
    return await invoke("get_operation_status", {operationId});
}

export async function getOperationProgress(operationId: string): Promise<OperationProgress | null> {
    return await invoke("get_operation_progress", {operationId});
}

export async function cancelOperation(operationId: string): Promise<boolean> {
    return await invoke("cancel_operation", {operationId});
}

export async function isOperationFinished(operationId: string): Promise<boolean | null> {
    return await invoke("is_operation_finished", {operationId});
}

export async function cleanupFinishedOperations(): Promise<void> {
    return await invoke("cleanup_finished_operations");
}

export async function listOperations(): Promise<string[]> {
    return await invoke("list_operations");
}

export async function getOperationsSummary(): Promise<OperationSummary[]> {
    return await invoke("get_operations_summary");
}

export async function getOperationResult(operationId: string): Promise<any | null> {
    const resultStr = await invoke("get_operation_result", {operationId});
    if (resultStr) {
        try {
            return JSON.parse(resultStr as string);
        } catch (e) {
            console.error("Failed to parse operation result:", e);
            return null;
        }
    }
    return null;
}