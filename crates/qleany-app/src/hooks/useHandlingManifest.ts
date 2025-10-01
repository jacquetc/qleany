import {useCallback, useEffect, useState} from 'react';
import {handlingManifestService, LoadManifestDTO, SaveManifestDTO} from '../services/handling-manifest-service';
import {rootService} from '../services/root-service';
import {featureEventService} from '../services/feature-event-service.ts';
import {error, info} from '@tauri-apps/plugin-log';
import {open, save} from '@tauri-apps/plugin-dialog';

/**
 * Custom hook for Home component functionality
 *
 * This hook provides functions for managing manifests and application state
 */
export function useHandlingManifest() {
    const [isLoading, setIsLoading] = useState(false);
    const [errorMessage, setErrorMessage] = useState<string | null>(null);
    const [manifestStatus, setManifestStatus] = useState<{
        loaded: boolean;
        saved: boolean;
        path?: string;
    }>({
        loaded: false,
        saved: false
    });

    // Set up event listeners for manifest events
    useEffect(() => {
        // Handler for manifest loaded events
        const handleManifestLoaded = () => {
            info('Manifest loaded event received');
            setManifestStatus(prev => ({
                ...prev,
                loaded: true
            }));
        };

        // Handler for manifest saved events
        const handleManifestSaved = () => {
            info('Manifest saved event received');
            setManifestStatus(prev => ({
                ...prev,
                saved: true
            }));
        };

        // Subscribe to manifest events
        const unsubscribe = featureEventService.subscribeToHandlingManifestEvents({
            onLoaded: handleManifestLoaded,
            onSaved: handleManifestSaved
        });

        // Cleanup function
        return () => {
            unsubscribe().catch(err => {
                error(`Error unsubscribing from manifest events: ${err}`);
            });
        };
    }, []);

    /**
     * Handle creating a new manifest
     * Currently just logs a message as the functionality is not implemented
     */
    const handleNewManifest = useCallback(async () => {
        info("New manifest action triggered");
        // This would typically create a new manifest
        // For now, just log a message
        info("New manifest functionality not yet implemented");
    }, []);

    /**
     * Handle opening a manifest file
     * Opens a file dialog and loads the selected manifest
     */
    const handleOpenManifest = useCallback(async () => {
        setIsLoading(true);
        setErrorMessage(null);
        // Reset manifest status
        setManifestStatus({
            loaded: false,
            saved: false
        });

        try {
            // Open file dialog to select a manifest file
            const selected = await open({
                multiple: false,
                directory: false,
                filters: [{
                    name: 'Manifest Files',
                    extensions: ['yaml', 'yml']
                }]
            });

            if (selected) {
                // If it's an array, take the first file
                const filePath = Array.isArray(selected) ? selected[0] : selected;

                const dto: LoadManifestDTO = {
                    manifest_path: filePath
                };

                const returnDTO = await handlingManifestService.loadManifest(dto);
                info(`Manifest loaded: ${filePath}`);

                // Note: The actual status update will come from the event subscription
                // This just updates the path
                setManifestStatus(prev => ({
                    ...prev,
                    path: filePath
                }));
                sessionStorage.setItem("rootId", returnDTO.root_id.toString());
                return returnDTO;
            }
            return null
        } catch (err) {
            const errorMsg = `Failed to open manifest: ${err}`;
            error(errorMsg);
            setErrorMessage(errorMsg);
        } finally {
            setIsLoading(false);
            return null
        }
    }, []);

    /**
     * Handle saving a manifest file
     * Opens a save dialog to select the save location
     */
    const handleSaveManifest = useCallback(async () => {
        setIsLoading(true);
        setErrorMessage(null);
        // Reset saved status
        setManifestStatus(prev => ({
            ...prev,
            saved: false
        }));

        try {
            info("Save manifest action triggered");
            const path = await save({
                filters: [
                    {
                        name: 'Manifest Files',
                        extensions: ['yaml', 'yml'],
                    },
                ],
            });

            if (path) {
                const dto: SaveManifestDTO = {
                    manifest_path: path
                };

                await handlingManifestService.saveManifest(dto);
                info(`Manifest saved to: ${path}`);

                // Note: The actual status update will come from the event subscription
                // This just updates the path
                setManifestStatus(prev => ({
                    ...prev,
                    path: path
                }));
            }
        } catch (err) {
            const errorMsg = `Failed to save manifest: ${err}`;
            error(errorMsg);
            setErrorMessage(errorMsg);
        } finally {
            setIsLoading(false);
        }
    }, []);

    /**
     * Handle closing the current manifest
     * Removes the root with ID 1
     */
    const handleCloseManifest = useCallback(async () => {
        setIsLoading(true);
        setErrorMessage(null);

        try {
            info("Close manifest action triggered");
            await rootService.removeRoot(1);
            info("Manifest closed successfully");

            // Reset manifest status completely when closing
            setManifestStatus({
                loaded: false,
                saved: false,
                path: undefined
            });
        } catch (err) {
            const errorMsg = `Failed to close manifest: ${err}`;
            error(errorMsg);
            setErrorMessage(errorMsg);
        } finally {
            setIsLoading(false);
        }
    }, []);

    /**
     * Handle opening the Qleany manifest (for testing)
     */
    const handleOpenQleanyManifest = useCallback(async () => {
        setIsLoading(true);
        setErrorMessage(null);
        // Reset manifest status
        setManifestStatus({
            loaded: false,
            saved: false
        });

        try {
            const manifestPath = "../../../qleany.yaml";
            const dto: LoadManifestDTO = {
                manifest_path: manifestPath
            };

            await handlingManifestService.loadManifest(dto);
            info("Qleany manifest loaded successfully");

            // Note: The actual status update will come from the event subscription
            // This just updates the path
            setManifestStatus(prev => ({
                ...prev,
                path: manifestPath
            }));
        } catch (err) {
            const errorMsg = `Failed to open Qleany manifest: ${err}`;
            error(errorMsg);
            setErrorMessage(errorMsg);
        } finally {
            setIsLoading(false);
        }
    }, []);

    return {
        isLoading,
        errorMessage,
        manifestStatus,
        handleNewManifest,
        handleOpenManifest,
        handleSaveManifest,
        handleCloseManifest,
        handleOpenQleanyManifest
    };
}