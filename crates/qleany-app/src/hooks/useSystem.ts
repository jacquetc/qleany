import {useCallback, useState} from 'react';
import {exit} from '@tauri-apps/plugin-process';
import {error, info} from '@tauri-apps/plugin-log';

export function useSystem() {
    const [isLoading, setIsLoading] = useState(false);
    const [errorMessage, setErrorMessage] = useState<string | null>(null);

    /**
     * Handle exiting the application
     */
    const handleExit = useCallback(async () => {
        setIsLoading(true);
        setErrorMessage(null);

        try {
            info("Exit application triggered");
            await exit(0);
        } catch (err) {
            const errorMsg = `Failed to exit application: ${err}`;
            error(errorMsg);
            setErrorMessage(errorMsg);
        } finally {
            setIsLoading(false);
        }
    }, []);

    return {
        isLoading,
        errorMessage,
        handleExit
    };
}