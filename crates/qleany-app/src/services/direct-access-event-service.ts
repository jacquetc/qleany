import {createEventSubscription, EventCallback, createUnsubscribeFn} from './tauri-api';
import {UnlistenFn} from '@tauri-apps/api/event';

/**
 * Event names for entity-specific events
 */
export const EntityEvents = {
    // File events
    FILE_CREATED: 'direct_access_file_created',
    FILE_UPDATED: 'direct_access_file_updated',
    FILE_REMOVED: 'direct_access_file_removed',

    // Root events
    ROOT_CREATED: 'direct_access_root_created',
    ROOT_UPDATED: 'direct_access_root_updated',
    ROOT_REMOVED: 'direct_access_root_removed',

    // Entity events
    ENTITY_CREATED: 'direct_access_entity_created',
    ENTITY_UPDATED: 'direct_access_entity_updated',
    ENTITY_REMOVED: 'direct_access_entity_removed',

    // Feature events
    FEATURE_CREATED: 'direct_access_feature_created',
    FEATURE_UPDATED: 'direct_access_feature_updated',
    FEATURE_REMOVED: 'direct_access_feature_removed',

    // Use case events
    USE_CASE_CREATED: 'direct_access_useCase_created',
    USE_CASE_UPDATED: 'direct_access_useCase_updated',
    USE_CASE_REMOVED: 'direct_access_useCase_removed',

    // Field events
    FIELD_CREATED: 'direct_access_field_created',
    FIELD_UPDATED: 'direct_access_field_updated',
    FIELD_REMOVED: 'direct_access_field_removed',

    // DTO events
    DTO_CREATED: 'direct_access_dto_created',
    DTO_UPDATED: 'direct_access_dto_updated',
    DTO_REMOVED: 'direct_access_dto_removed',

    // DTO field events
    DTO_FIELD_CREATED: 'direct_access_dto_field_created',
    DTO_FIELD_UPDATED: 'direct_access_dto_field_updated',
    DTO_FIELD_REMOVED: 'direct_access_dto_field_removed',

    // Global events
    GLOBAL_CREATED: 'direct_access_global_created',
    GLOBAL_UPDATED: 'direct_access_global_updated',
    GLOBAL_REMOVED: 'direct_access_global_removed',

    // Relationship events
    RELATIONSHIP_CREATED: 'direct_access_relationship_created',
    RELATIONSHIP_UPDATED: 'direct_access_relationship_updated',
    RELATIONSHIP_REMOVED: 'direct_access_relationship_removed',

    // Reset event
    ALL_RESET: 'direct_access_all_reset',
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
export const directAccessEventService = {
    /**
     * Subscribe to a file created event
     * @param callback The callback to call when the event is emitted
     * @returns A promise that resolves to an unsubscribe function
     */
    onFileCreated: (callback: EventCallback<EntityEventPayload>): Promise<UnlistenFn> => {
        return createEventSubscription<EntityEventPayload>(EntityEvents.FILE_CREATED, callback);
    },

    /**
     * Subscribe to a file updated event
     * @param callback The callback to call when the event is emitted
     * @returns A promise that resolves to an unsubscribe function
     */
    onFileUpdated: (callback: EventCallback<EntityEventPayload>): Promise<UnlistenFn> => {
        return createEventSubscription<EntityEventPayload>(EntityEvents.FILE_UPDATED, callback);
    },

    /**
     * Subscribe to a file removed event
     * @param callback The callback to call when the event is emitted
     * @returns A promise that resolves to an unsubscribe function
     */
    onFileRemoved: (callback: EventCallback<EntityEventPayload>): Promise<UnlistenFn> => {
        return createEventSubscription<EntityEventPayload>(EntityEvents.FILE_REMOVED, callback);
    },

    /**
     * Subscribe to a root created event
     * @param callback The callback to call when the event is emitted
     * @returns A promise that resolves to an unsubscribe function
     */
    onRootCreated: (callback: EventCallback<EntityEventPayload>): Promise<UnlistenFn> => {
        return createEventSubscription<EntityEventPayload>(EntityEvents.ROOT_CREATED, callback);
    },

    /**
     * Subscribe to a root updated event
     * @param callback The callback to call when the event is emitted
     * @returns A promise that resolves to an unsubscribe function
     */
    onRootUpdated: (callback: EventCallback<EntityEventPayload>): Promise<UnlistenFn> => {
        return createEventSubscription<EntityEventPayload>(EntityEvents.ROOT_UPDATED, callback);
    },

    /**
     * Subscribe to a root removed event
     * @param callback The callback to call when the event is emitted
     * @returns A promise that resolves to an unsubscribe function
     */
    onRootRemoved: (callback: EventCallback<EntityEventPayload>): Promise<UnlistenFn> => {
        return createEventSubscription<EntityEventPayload>(EntityEvents.ROOT_REMOVED, callback);
    },

    /**
     * Subscribe to an entity created event
     * @param callback The callback to call when the event is emitted
     * @returns A promise that resolves to an unsubscribe function
     */
    onEntityCreated: (callback: EventCallback<EntityEventPayload>): Promise<UnlistenFn> => {
        return createEventSubscription<EntityEventPayload>(EntityEvents.ENTITY_CREATED, callback);
    },

    /**
     * Subscribe to an entity updated event
     * @param callback The callback to call when the event is emitted
     * @returns A promise that resolves to an unsubscribe function
     */
    onEntityUpdated: (callback: EventCallback<EntityEventPayload>): Promise<UnlistenFn> => {
        return createEventSubscription<EntityEventPayload>(EntityEvents.ENTITY_UPDATED, callback);
    },

    /**
     * Subscribe to an entity removed event
     * @param callback The callback to call when the event is emitted
     * @returns A promise that resolves to an unsubscribe function
     */
    onEntityRemoved: (callback: EventCallback<EntityEventPayload>): Promise<UnlistenFn> => {
        return createEventSubscription<EntityEventPayload>(EntityEvents.ENTITY_REMOVED, callback);
    },

    /**
     * Subscribe to a feature created event
     * @param callback The callback to call when the event is emitted
     * @returns A promise that resolves to an unsubscribe function
     */
    onFeatureCreated: (callback: EventCallback<EntityEventPayload>): Promise<UnlistenFn> => {
        return createEventSubscription<EntityEventPayload>(EntityEvents.FEATURE_CREATED, callback);
    },

    /**
     * Subscribe to a feature updated event
     * @param callback The callback to call when the event is emitted
     * @returns A promise that resolves to an unsubscribe function
     */
    onFeatureUpdated: (callback: EventCallback<EntityEventPayload>): Promise<UnlistenFn> => {
        return createEventSubscription<EntityEventPayload>(EntityEvents.FEATURE_UPDATED, callback);
    },

    /**
     * Subscribe to a feature removed event
     * @param callback The callback to call when the event is emitted
     * @returns A promise that resolves to an unsubscribe function
     */
    onFeatureRemoved: (callback: EventCallback<EntityEventPayload>): Promise<UnlistenFn> => {
        return createEventSubscription<EntityEventPayload>(EntityEvents.FEATURE_REMOVED, callback);
    },

    /**
     * Subscribe to a use case created event
     * @param callback The callback to call when the event is emitted
     * @returns A promise that resolves to an unsubscribe function
     */
    onUseCaseCreated: (callback: EventCallback<EntityEventPayload>): Promise<UnlistenFn> => {
        return createEventSubscription<EntityEventPayload>(EntityEvents.USE_CASE_CREATED, callback);
    },

    /**
     * Subscribe to a use case updated event
     * @param callback The callback to call when the event is emitted
     * @returns A promise that resolves to an unsubscribe function
     */
    onUseCaseUpdated: (callback: EventCallback<EntityEventPayload>): Promise<UnlistenFn> => {
        return createEventSubscription<EntityEventPayload>(EntityEvents.USE_CASE_UPDATED, callback);
    },

    /**
     * Subscribe to a use case removed event
     * @param callback The callback to call when the event is emitted
     * @returns A promise that resolves to an unsubscribe function
     */
    onUseCaseRemoved: (callback: EventCallback<EntityEventPayload>): Promise<UnlistenFn> => {
        return createEventSubscription<EntityEventPayload>(EntityEvents.USE_CASE_REMOVED, callback);
    },

    /**
     * Subscribe to a field created event
     * @param callback The callback to call when the event is emitted
     * @returns A promise that resolves to an unsubscribe function
     */
    onFieldCreated: (callback: EventCallback<EntityEventPayload>): Promise<UnlistenFn> => {
        return createEventSubscription<EntityEventPayload>(EntityEvents.FIELD_CREATED, callback);
    },

    /**
     * Subscribe to a field updated event
     * @param callback The callback to call when the event is emitted
     * @returns A promise that resolves to an unsubscribe function
     */
    onFieldUpdated: (callback: EventCallback<EntityEventPayload>): Promise<UnlistenFn> => {
        return createEventSubscription<EntityEventPayload>(EntityEvents.FIELD_UPDATED, callback);
    },

    /**
     * Subscribe to a field removed event
     * @param callback The callback to call when the event is emitted
     * @returns A promise that resolves to an unsubscribe function
     */
    onFieldRemoved: (callback: EventCallback<EntityEventPayload>): Promise<UnlistenFn> => {
        return createEventSubscription<EntityEventPayload>(EntityEvents.FIELD_REMOVED, callback);
    },

    /**
     * Subscribe to a DTO created event
     * @param callback The callback to call when the event is emitted
     * @returns A promise that resolves to an unsubscribe function
     */
    onDtoCreated: (callback: EventCallback<EntityEventPayload>): Promise<UnlistenFn> => {
        return createEventSubscription<EntityEventPayload>(EntityEvents.DTO_CREATED, callback);
    },

    /**
     * Subscribe to a DTO updated event
     * @param callback The callback to call when the event is emitted
     * @returns A promise that resolves to an unsubscribe function
     */
    onDtoUpdated: (callback: EventCallback<EntityEventPayload>): Promise<UnlistenFn> => {
        return createEventSubscription<EntityEventPayload>(EntityEvents.DTO_UPDATED, callback);
    },

    /**
     * Subscribe to a DTO removed event
     * @param callback The callback to call when the event is emitted
     * @returns A promise that resolves to an unsubscribe function
     */
    onDtoRemoved: (callback: EventCallback<EntityEventPayload>): Promise<UnlistenFn> => {
        return createEventSubscription<EntityEventPayload>(EntityEvents.DTO_REMOVED, callback);
    },

    /**
     * Subscribe to a DTO field created event
     * @param callback The callback to call when the event is emitted
     * @returns A promise that resolves to an unsubscribe function
     */
    onDtoFieldCreated: (callback: EventCallback<EntityEventPayload>): Promise<UnlistenFn> => {
        return createEventSubscription<EntityEventPayload>(EntityEvents.DTO_FIELD_CREATED, callback);
    },

    /**
     * Subscribe to a DTO field updated event
     * @param callback The callback to call when the event is emitted
     * @returns A promise that resolves to an unsubscribe function
     */
    onDtoFieldUpdated: (callback: EventCallback<EntityEventPayload>): Promise<UnlistenFn> => {
        return createEventSubscription<EntityEventPayload>(EntityEvents.DTO_FIELD_UPDATED, callback);
    },

    /**
     * Subscribe to a DTO field removed event
     * @param callback The callback to call when the event is emitted
     * @returns A promise that resolves to an unsubscribe function
     */
    onDtoFieldRemoved: (callback: EventCallback<EntityEventPayload>): Promise<UnlistenFn> => {
        return createEventSubscription<EntityEventPayload>(EntityEvents.DTO_FIELD_REMOVED, callback);
    },

    /**
     * Subscribe to a global created event
     * @param callback The callback to call when the event is emitted
     * @returns A promise that resolves to an unsubscribe function
     */
    onGlobalCreated: (callback: EventCallback<EntityEventPayload>): Promise<UnlistenFn> => {
        return createEventSubscription<EntityEventPayload>(EntityEvents.GLOBAL_CREATED, callback);
    },

    /**
     * Subscribe to a global updated event
     * @param callback The callback to call when the event is emitted
     * @returns A promise that resolves to an unsubscribe function
     */
    onGlobalUpdated: (callback: EventCallback<EntityEventPayload>): Promise<UnlistenFn> => {
        return createEventSubscription<EntityEventPayload>(EntityEvents.GLOBAL_UPDATED, callback);
    },

    /**
     * Subscribe to a global removed event
     * @param callback The callback to call when the event is emitted
     * @returns A promise that resolves to an unsubscribe function
     */
    onGlobalRemoved: (callback: EventCallback<EntityEventPayload>): Promise<UnlistenFn> => {
        return createEventSubscription<EntityEventPayload>(EntityEvents.GLOBAL_REMOVED, callback);
    },

    /**
     * Subscribe to a relationship created event
     * @param callback The callback to call when the event is emitted
     * @returns A promise that resolves to an unsubscribe function
     */
    onRelationshipCreated: (callback: EventCallback<EntityEventPayload>): Promise<UnlistenFn> => {
        return createEventSubscription<EntityEventPayload>(EntityEvents.RELATIONSHIP_CREATED, callback);
    },

    /**
     * Subscribe to a relationship updated event
     * @param callback The callback to call when the event is emitted
     * @returns A promise that resolves to an unsubscribe function
     */
    onRelationshipUpdated: (callback: EventCallback<EntityEventPayload>): Promise<UnlistenFn> => {
        return createEventSubscription<EntityEventPayload>(EntityEvents.RELATIONSHIP_UPDATED, callback);
    },

    /**
     * Subscribe to a relationship removed event
     * @param callback The callback to call when the event is emitted
     * @returns A promise that resolves to an unsubscribe function
     */
    onRelationshipRemoved: (callback: EventCallback<EntityEventPayload>): Promise<UnlistenFn> => {
        return createEventSubscription<EntityEventPayload>(EntityEvents.RELATIONSHIP_REMOVED, callback);
    },

    /**
     * Subscribe to an all reset event
     * @param callback The callback to call when the event is emitted
     * @returns A promise that resolves to an unsubscribe function
     */
    onAllReset: (callback: EventCallback<void>): Promise<UnlistenFn> => {
        return createEventSubscription<void>(EntityEvents.ALL_RESET, callback);
    },

    /**
     * Subscribe to multiple file-related events
     * @param callbacks Object containing callbacks for different events
     * @returns A function that unsubscribes from all events
     */
    subscribeToFileEvents: (callbacks: {
        onCreated?: EventCallback<EntityEventPayload>;
        onUpdated?: EventCallback<EntityEventPayload>;
        onRemoved?: EventCallback<EntityEventPayload>;
        onReset?: EventCallback<void>;
    }): () => Promise<void> => {
        const subscriptions: Promise<UnlistenFn>[] = [];

        if (callbacks.onCreated) {
            subscriptions.push(directAccessEventService.onFileCreated(callbacks.onCreated));
        }

        if (callbacks.onUpdated) {
            subscriptions.push(directAccessEventService.onFileUpdated(callbacks.onUpdated));
        }

        if (callbacks.onRemoved) {
            subscriptions.push(directAccessEventService.onFileRemoved(callbacks.onRemoved));
        }

        if (callbacks.onReset) {
            subscriptions.push(directAccessEventService.onAllReset(callbacks.onReset));
        }

        return createUnsubscribeFn(subscriptions);
    },

    /**
     * Subscribe to multiple root-related events
     * @param callbacks Object containing callbacks for different events
     * @returns A function that unsubscribes from all events
     */
    subscribeToRootEvents: (callbacks: {
        onCreated?: EventCallback<EntityEventPayload>;
        onUpdated?: EventCallback<EntityEventPayload>;
        onRemoved?: EventCallback<EntityEventPayload>;
        onReset?: EventCallback<void>;
    }): () => Promise<void> => {
        const subscriptions: Promise<UnlistenFn>[] = [];

        if (callbacks.onCreated) {
            subscriptions.push(directAccessEventService.onRootCreated(callbacks.onCreated));
        }

        if (callbacks.onUpdated) {
            subscriptions.push(directAccessEventService.onRootUpdated(callbacks.onUpdated));
        }

        if (callbacks.onRemoved) {
            subscriptions.push(directAccessEventService.onRootRemoved(callbacks.onRemoved));
        }

        if (callbacks.onReset) {
            subscriptions.push(directAccessEventService.onAllReset(callbacks.onReset));
        }

        return createUnsubscribeFn(subscriptions);
    },

    /**
     * Subscribe to multiple entity-related events
     * @param callbacks Object containing callbacks for different events
     * @returns A function that unsubscribes from all events
     */
    subscribeToEntityEvents: (callbacks: {
        onCreated?: EventCallback<EntityEventPayload>;
        onUpdated?: EventCallback<EntityEventPayload>;
        onRemoved?: EventCallback<EntityEventPayload>;
        onReset?: EventCallback<void>;
    }): () => Promise<void> => {
        const subscriptions: Promise<UnlistenFn>[] = [];

        if (callbacks.onCreated) {
            subscriptions.push(directAccessEventService.onEntityCreated(callbacks.onCreated));
        }

        if (callbacks.onUpdated) {
            subscriptions.push(directAccessEventService.onEntityUpdated(callbacks.onUpdated));
        }

        if (callbacks.onRemoved) {
            subscriptions.push(directAccessEventService.onEntityRemoved(callbacks.onRemoved));
        }

        if (callbacks.onReset) {
            subscriptions.push(directAccessEventService.onAllReset(callbacks.onReset));
        }

        return createUnsubscribeFn(subscriptions);
    },

    /**
     * Subscribe to multiple feature-related events
     * @param callbacks Object containing callbacks for different events
     * @returns A function that unsubscribes from all events
     */
    subscribeToFeatureEvents: (callbacks: {
        onCreated?: EventCallback<EntityEventPayload>;
        onUpdated?: EventCallback<EntityEventPayload>;
        onRemoved?: EventCallback<EntityEventPayload>;
        onReset?: EventCallback<void>;
    }): () => Promise<void> => {
        const subscriptions: Promise<UnlistenFn>[] = [];

        if (callbacks.onCreated) {
            subscriptions.push(directAccessEventService.onFeatureCreated(callbacks.onCreated));
        }

        if (callbacks.onUpdated) {
            subscriptions.push(directAccessEventService.onFeatureUpdated(callbacks.onUpdated));
        }

        if (callbacks.onRemoved) {
            subscriptions.push(directAccessEventService.onFeatureRemoved(callbacks.onRemoved));
        }

        if (callbacks.onReset) {
            subscriptions.push(directAccessEventService.onAllReset(callbacks.onReset));
        }

        return createUnsubscribeFn(subscriptions);
    },

    /**
     * Subscribe to multiple use case-related events
     * @param callbacks Object containing callbacks for different events
     * @returns A function that unsubscribes from all events
     */
    subscribeToUseCaseEvents: (callbacks: {
        onCreated?: EventCallback<EntityEventPayload>;
        onUpdated?: EventCallback<EntityEventPayload>;
        onRemoved?: EventCallback<EntityEventPayload>;
        onReset?: EventCallback<void>;
    }): () => Promise<void> => {
        const subscriptions: Promise<UnlistenFn>[] = [];

        if (callbacks.onCreated) {
            subscriptions.push(directAccessEventService.onUseCaseCreated(callbacks.onCreated));
        }

        if (callbacks.onUpdated) {
            subscriptions.push(directAccessEventService.onUseCaseUpdated(callbacks.onUpdated));
        }

        if (callbacks.onRemoved) {
            subscriptions.push(directAccessEventService.onUseCaseRemoved(callbacks.onRemoved));
        }

        if (callbacks.onReset) {
            subscriptions.push(directAccessEventService.onAllReset(callbacks.onReset));
        }

        return createUnsubscribeFn(subscriptions);
    },

    /**
     * Subscribe to multiple field-related events
     * @param callbacks Object containing callbacks for different events
     * @returns A function that unsubscribes from all events
     */
    subscribeToFieldEvents: (callbacks: {
        onCreated?: EventCallback<EntityEventPayload>;
        onUpdated?: EventCallback<EntityEventPayload>;
        onRemoved?: EventCallback<EntityEventPayload>;
        onReset?: EventCallback<void>;
    }): () => Promise<void> => {
        const subscriptions: Promise<UnlistenFn>[] = [];

        if (callbacks.onCreated) {
            subscriptions.push(directAccessEventService.onFieldCreated(callbacks.onCreated));
        }

        if (callbacks.onUpdated) {
            subscriptions.push(directAccessEventService.onFieldUpdated(callbacks.onUpdated));
        }

        if (callbacks.onRemoved) {
            subscriptions.push(directAccessEventService.onFieldRemoved(callbacks.onRemoved));
        }

        if (callbacks.onReset) {
            subscriptions.push(directAccessEventService.onAllReset(callbacks.onReset));
        }

        return createUnsubscribeFn(subscriptions);
    },

    /**
     * Subscribe to multiple DTO-related events
     * @param callbacks Object containing callbacks for different events
     * @returns A function that unsubscribes from all events
     */
    subscribeToDtoEvents: (callbacks: {
        onCreated?: EventCallback<EntityEventPayload>;
        onUpdated?: EventCallback<EntityEventPayload>;
        onRemoved?: EventCallback<EntityEventPayload>;
        onReset?: EventCallback<void>;
    }): () => Promise<void> => {
        const subscriptions: Promise<UnlistenFn>[] = [];

        if (callbacks.onCreated) {
            subscriptions.push(directAccessEventService.onDtoCreated(callbacks.onCreated));
        }

        if (callbacks.onUpdated) {
            subscriptions.push(directAccessEventService.onDtoUpdated(callbacks.onUpdated));
        }

        if (callbacks.onRemoved) {
            subscriptions.push(directAccessEventService.onDtoRemoved(callbacks.onRemoved));
        }

        if (callbacks.onReset) {
            subscriptions.push(directAccessEventService.onAllReset(callbacks.onReset));
        }

        return createUnsubscribeFn(subscriptions);
    },

    /**
     * Subscribe to multiple DTO field-related events
     * @param callbacks Object containing callbacks for different events
     * @returns A function that unsubscribes from all events
     */
    subscribeToDtoFieldEvents: (callbacks: {
        onCreated?: EventCallback<EntityEventPayload>;
        onUpdated?: EventCallback<EntityEventPayload>;
        onRemoved?: EventCallback<EntityEventPayload>;
        onReset?: EventCallback<void>;
    }): () => Promise<void> => {
        const subscriptions: Promise<UnlistenFn>[] = [];

        if (callbacks.onCreated) {
            subscriptions.push(directAccessEventService.onDtoFieldCreated(callbacks.onCreated));
        }

        if (callbacks.onUpdated) {
            subscriptions.push(directAccessEventService.onDtoFieldUpdated(callbacks.onUpdated));
        }

        if (callbacks.onRemoved) {
            subscriptions.push(directAccessEventService.onDtoFieldRemoved(callbacks.onRemoved));
        }

        if (callbacks.onReset) {
            subscriptions.push(directAccessEventService.onAllReset(callbacks.onReset));
        }

        return createUnsubscribeFn(subscriptions);
    },

    /**
     * Subscribe to global events
     * @param callbacks Object containing callbacks for different events
     * @returns A function that unsubscribes from all events
     */
    subscribeToGlobalEvents: (callbacks: {
        onCreated?: EventCallback<EntityEventPayload>;
        onUpdated?: EventCallback<EntityEventPayload>;
        onRemoved?: EventCallback<EntityEventPayload>;
        onReset?: EventCallback<void>;
    }): () => Promise<void> => {
        const subscriptions: Promise<UnlistenFn>[] = [];

        if (callbacks.onCreated) {
            subscriptions.push(directAccessEventService.onGlobalCreated(callbacks.onCreated));
        }

        if (callbacks.onUpdated) {
            subscriptions.push(directAccessEventService.onGlobalUpdated(callbacks.onUpdated));
        }

        if (callbacks.onRemoved) {
            subscriptions.push(directAccessEventService.onGlobalRemoved(callbacks.onRemoved));
        }

        if (callbacks.onReset) {
            subscriptions.push(directAccessEventService.onAllReset(callbacks.onReset));
        }

        return createUnsubscribeFn(subscriptions);
    },

    /**
     * Subscribe to multiple relationship-related events
     * @param callbacks Object containing callbacks for different events
     * @returns A function that unsubscribes from all events
     */
    subscribeToRelationshipEvents: (callbacks: {
        onCreated?: EventCallback<EntityEventPayload>;
        onUpdated?: EventCallback<EntityEventPayload>;
        onRemoved?: EventCallback<EntityEventPayload>;
        onReset?: EventCallback<void>;
    }): () => Promise<void> => {
        const subscriptions: Promise<UnlistenFn>[] = [];

        if (callbacks.onCreated) {
            subscriptions.push(directAccessEventService.onRelationshipCreated(callbacks.onCreated));
        }

        if (callbacks.onUpdated) {
            subscriptions.push(directAccessEventService.onRelationshipUpdated(callbacks.onUpdated));
        }

        if (callbacks.onRemoved) {
            subscriptions.push(directAccessEventService.onRelationshipRemoved(callbacks.onRemoved));
        }

        if (callbacks.onReset) {
            subscriptions.push(directAccessEventService.onAllReset(callbacks.onReset));
        }

        return createUnsubscribeFn(subscriptions);
    },

};