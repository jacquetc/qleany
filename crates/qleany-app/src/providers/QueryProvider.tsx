import React from 'react';
import { QueryClient, QueryClientProvider } from '@tanstack/react-query';

// Create a client
const queryClient = new QueryClient({
    defaultOptions: {
        queries: {
            refetchOnWindowFocus: false, // Don't refetch on window focus by default
            retry: 1, // Only retry failed queries once
            staleTime: 1000 * 60 * 5, // Data is fresh for 5 minutes
        },
    },
});

interface QueryProviderProps {
    children: React.ReactNode;
}

/**
 * QueryProvider component that sets up React Query
 * 
 * This component provides React Query functionality to the entire application.
 * It configures the QueryClient with sensible defaults and wraps the application
 * with QueryClientProvider.
 */
export function QueryProvider({children}: QueryProviderProps) {
    return (
        <QueryClientProvider client={queryClient}>
            {children}
        </QueryClientProvider>
    );
}