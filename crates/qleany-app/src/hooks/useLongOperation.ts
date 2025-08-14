import {useEffect, useMemo, useState, useCallback} from 'react';
import { longOperationService, OperationStatus, OperationProgress, LongOperationEventPayload } from '../services/long-operation-service';
import { error } from '@tauri-apps/plugin-log';

interface OperationState {
  status: OperationStatus;
  progress: OperationProgress;
}

function parseData<T = any>(payload?: LongOperationEventPayload): T | undefined {
  if (!payload?.data) return undefined;
  try {
    return JSON.parse(payload.data) as T;
  } catch (e) {
    console.error('Failed to parse long operation event payload', e, payload.data);
    return undefined;
  }
}

export function useLongOperation() {
  const [operations, setOperations] = useState<Record<string, OperationState>>({});

  // Initialize from summaries and subscribe to events
  useEffect(() => {
    let unsubAll: (() => Promise<void>) | undefined;

    const init = async () => {
      try {
        const summaries = await longOperationService.getOperationsSummary();
        const initial: Record<string, OperationState> = {};
        for (const s of summaries) {
          initial[s.id] = { status: s.status, progress: s.progress };
        }
        setOperations(initial);
      } catch (e) {
        error(`Failed to load long operations summary: ${e}`);
      }

      try {
        unsubAll = longOperationService.subscribeToLongOperationEvents({
          onStarted: (payload) => {
            const id = payload?.data; // data is the id string
            if (!id) return;
            setOperations((prev) => ({
              ...prev,
              [id]: { status: 'Running', progress: { percentage: 0, message: 'Started' } },
            }));
          },
          onProgress: (payload) => {
            const data = parseData<{ id: string; percentage: number; message?: string | null }>(payload);
            if (!data) return;
            setOperations((prev) => {
              const current = prev[data.id] ?? { status: 'Running', progress: { percentage: 0, message: undefined } };
              return {
                ...prev,
                [data.id]: { status: current.status, progress: { percentage: data.percentage, message: data.message } },
              };
            });
          },
          onCancelled: (payload) => {
            const data = parseData<{ id: string }>(payload);
            if (!data) return;
            setOperations((prev) => {
              const current = prev[data.id] ?? { status: 'Running', progress: { percentage: 0, message: undefined } };
              return {
                ...prev,
                [data.id]: { status: 'Cancelled', progress: current.progress },
              };
            });
          },
          onCompleted: (payload) => {
            const data = parseData<{ id: string }>(payload);
            if (!data) return;
            setOperations((prev) => {
              const current = prev[data.id] ?? { status: 'Running', progress: { percentage: 100, message: 'Completed' } };
              return {
                ...prev,
                [data.id]: { status: 'Completed', progress: { ...current.progress, percentage: 100, message: 'Completed' } },
              };
            });
          },
          onFailed: (payload) => {
            const data = parseData<{ id: string; error?: string }>(payload);
            if (!data) return;
            setOperations((prev) => {
              const current = prev[data.id] ?? { status: 'Running', progress: { percentage: 0, message: undefined } };
              return {
                ...prev,
                [data.id]: { status: { Failed: data.error ?? 'Unknown error' }, progress: current.progress },
              };
            });
          },
        });
      } catch (e) {
        error(`Failed to subscribe to long operation events: ${e}`);
      }
    };

    void init();

    return () => {
      if (unsubAll) {
        unsubAll().catch((e) => error(`Error unsubscribing from long operation events: ${e}`));
      }
    };
  }, []);

  const cancelOperation = useCallback(async (id: string) => {
    try {
      await longOperationService.cancelOperation(id);
    } catch (e) {
      error(`Failed to cancel operation ${id}: ${e}`);
    }
  }, []);

  const getResult = useCallback(async <T = unknown>(id: string): Promise<T | null> => {
    try {
      return await longOperationService.getOperationResult<T>(id);
    } catch (e) {
      error(`Failed to get operation result for ${id}: ${e}`);
      return null;
    }
  }, []);

  const list = useMemo(() => Object.keys(operations), [operations]);

  return {
    operations,
    list,
    cancelOperation,
    getResult,
  } as const;
}
