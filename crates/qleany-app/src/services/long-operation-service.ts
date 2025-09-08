import {createEventSubscription, createUnsubscribeFn, EventCallback, invokeCommand} from './tauri-api';
import {UnlistenFn} from '@tauri-apps/api/event';

export const LongOperationEvents = {
    STARTED: 'long_operation_started',
    PROGRESS: 'long_operation_progress',
    CANCELLED: 'long_operation_cancelled',
    COMPLETED: 'long_operation_completed',
    FAILED: 'long_operation_failed',
} as const;

export type LongOperationEventName = typeof LongOperationEvents[keyof typeof LongOperationEvents];

export type OperationStatus = 'Running' | 'Completed' | 'Cancelled' | { Failed: string };

export interface OperationProgress {
    percentage: number;
    message?: string | null;
}

export interface OperationSummary {
    id: string;
    status: OperationStatus;
    progress: OperationProgress;
}

export interface LongOperationEventPayload {
    ids: number[];
    data?: string; // JSON string with { id, ... }
}

export const longOperationService = {
    // Queries
    getOperationStatus: (operation_id: string): Promise<OperationStatus | null> => {
        return invokeCommand<OperationStatus | null>('get_operation_status', {operationId: operation_id});
    },

    getOperationProgress: (operation_id: string): Promise<OperationProgress | null> => {
        return invokeCommand<OperationProgress | null>('get_operation_progress', {operationId: operation_id});
    },

    cancelOperation: (operation_id: string): Promise<boolean> => {
        return invokeCommand<boolean>('cancel_operation', {operationId: operation_id});
    },

    isOperationFinished: (operation_id: string): Promise<boolean | null> => {
        return invokeCommand<boolean | null>('is_operation_finished', {operationId: operation_id});
    },

    cleanupFinishedOperations: (): Promise<void> => {
        return invokeCommand<void>('cleanup_finished_operations');
    },

    listOperations: (): Promise<string[]> => {
        return invokeCommand<string[]>('list_operations');
    },

    getOperationsSummary: (): Promise<OperationSummary[]> => {
        return invokeCommand<OperationSummary[]>('get_operations_summary');
    },

    getOperationResult: <T = unknown>(operation_id: string): Promise<T | null> => {
        return invokeCommand<string | null>('get_operation_result', {operation_id: operation_id})
            .then((json) => (json ? (JSON.parse(json) as T) : null));
    },

    // Event subscriptions
    onStarted: (callback: EventCallback<LongOperationEventPayload>) =>
        createEventSubscription<LongOperationEventPayload>(LongOperationEvents.STARTED, callback),

    onProgress: (callback: EventCallback<LongOperationEventPayload>) =>
        createEventSubscription<LongOperationEventPayload>(LongOperationEvents.PROGRESS, callback),

    onCancelled: (callback: EventCallback<LongOperationEventPayload>) =>
        createEventSubscription<LongOperationEventPayload>(LongOperationEvents.CANCELLED, callback),

    onCompleted: (callback: EventCallback<LongOperationEventPayload>) =>
        createEventSubscription<LongOperationEventPayload>(LongOperationEvents.COMPLETED, callback),

    onFailed: (callback: EventCallback<LongOperationEventPayload>) =>
        createEventSubscription<LongOperationEventPayload>(LongOperationEvents.FAILED, callback),

    subscribeToLongOperationEvents: (callbacks: {
        onStarted?: EventCallback<LongOperationEventPayload>;
        onProgress?: EventCallback<LongOperationEventPayload>;
        onCancelled?: EventCallback<LongOperationEventPayload>;
        onCompleted?: EventCallback<LongOperationEventPayload>;
        onFailed?: EventCallback<LongOperationEventPayload>;
    }): () => Promise<void> => {
        const subs: Promise<UnlistenFn>[] = [];
        if (callbacks.onStarted) subs.push(longOperationService.onStarted(callbacks.onStarted));
        if (callbacks.onProgress) subs.push(longOperationService.onProgress(callbacks.onProgress));
        if (callbacks.onCancelled) subs.push(longOperationService.onCancelled(callbacks.onCancelled));
        if (callbacks.onCompleted) subs.push(longOperationService.onCompleted(callbacks.onCompleted));
        if (callbacks.onFailed) subs.push(longOperationService.onFailed(callbacks.onFailed));
        return createUnsubscribeFn(subs);
    },
};
