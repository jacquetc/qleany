import {createEventSubscription, createUnsubscribeFn, EventCallback} from './tauri-api';
import {UnlistenFn} from '@tauri-apps/api/event';

/**
 * Event names for feature-specific events
 */
export const FeatureEvents = {

    // Handling Manifest events
    HANDLING_MANIFEST_LOADED: 'handling_manifest_loaded',
    HANDLING_MANIFEST_SAVED: 'handling_manifest_saved',

};

/**
 * Type for entity event payloads
 */
export interface EntityEventPayload {
    ids: number[];
    data?: string;
}

/**
 * Event service for subscribing to Tauri events
 */
export const featureEventService = {
    /**
     * Subscribe to a manifest loaded event
     * @param callback The callback to call when the event is emitted
     * @returns A promise that resolves to an unsubscribe function
     */
    onHandlingManifestLoaded: (callback: EventCallback<void>): Promise<UnlistenFn> => {
        return createEventSubscription<void>(FeatureEvents.HANDLING_MANIFEST_LOADED, callback);
    },

    /**
     * Subscribe to a manifest saved event
     * @param callback The callback to call when the event is emitted
     * @returns A promise that resolves to an unsubscribe function
     */
    onHandlingManifestSaved: (callback: EventCallback<void>): Promise<UnlistenFn> => {
        return createEventSubscription<void>(FeatureEvents.HANDLING_MANIFEST_SAVED, callback);
    },

    /**
     * Subscribe to multiple manifest-related events
     * @param callbacks Object containing callbacks for different events
     * @returns A function that unsubscribes from all events
     */
    subscribeToHandlingManifestEvents: (callbacks: {
        onLoaded?: EventCallback<void>;
        onSaved?: EventCallback<void>;
    }): () => Promise<void> => {
        const subscriptions: Promise<UnlistenFn>[] = [];

        if (callbacks.onLoaded) {
            subscriptions.push(featureEventService.onHandlingManifestLoaded(callbacks.onLoaded));
        }

        if (callbacks.onSaved) {
            subscriptions.push(featureEventService.onHandlingManifestSaved(callbacks.onSaved));
        }

        return createUnsubscribeFn(subscriptions);
    }

};