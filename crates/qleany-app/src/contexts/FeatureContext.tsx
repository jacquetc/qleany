import React, {createContext, useContext, useState, useEffect} from 'react';
import {Alert} from '@mantine/core';
import {error as logError} from '@tauri-apps/plugin-log';
import {useFeatures} from '../hooks/useFeatures';
import {useUseCases} from '../hooks/useUseCases';
import {useDtos} from '../hooks/useDtos';
import {useDtoFields} from '../hooks/useDtoFields';
import {FeatureDTO} from '../services/feature-service';
import {UseCaseDTO} from '../services/use-case-service';
import {DtoDTO} from '../services/dto-service';
import {DtoFieldDTO} from '../services/dto-field-service';

/**
 * Interface for the FeatureContext value
 */
interface FeatureContextValue {
    // Features data
    features: FeatureDTO[];
    selectedFeatureId: number | null;
    isLoadingFeatures: boolean;
    featureError: unknown;

    // Use Cases data
    useCases: UseCaseDTO[];
    selectedUseCaseId: number | null;
    isLoadingUseCases: boolean;
    useCaseError: unknown;

    // DTOs data
    dtos: DtoDTO[];
    selectedDtoId: number | null;
    isLoadingDtos: boolean;
    dtoError: unknown;

    // DTO Fields data
    dtoFields: DtoFieldDTO[];
    selectedDtoFieldId: number | null;
    isLoadingDtoFields: boolean;
    dtoFieldError: unknown;

    // Hook error
    hookError: Error | null;

    // Actions
    selectFeature: (featureId: number | null) => void;
    createFeature: () => void;
    updateFeature: (feature: FeatureDTO) => void;
    reorderFeatures: (reorderedIds: number[]) => void;

    selectUseCase: (useCaseId: number | null) => void;
    createUseCase: () => void;
    updateUseCase: (useCase: UseCaseDTO) => void;
    reorderUseCases: (reorderedIds: number[]) => void;

    selectDto: (dtoId: number | null) => void;
    createDto: (name?: string) => void;
    updateDto: (dto: DtoDTO) => void;
    removeDto: (dtoId: number) => void;

    selectDtoField: (dtoFieldId: number | null) => void;
    createDtoField: (fieldData?: {
        name: string;
        fieldType: string;
        isNullable: boolean;
        isList: boolean;
    }) => void;
    updateDtoField: (dtoField: DtoFieldDTO) => void;
    removeDtoField: (dtoFieldId: number) => void;

    refetchAll: () => Promise<void>;
}

// Create the context with a default value
const FeatureContext = createContext<FeatureContextValue | undefined>(undefined);

/**
 * Props for the FeatureProvider component
 */
interface FeatureProviderProps {
    rootId: number | null;
    children: React.ReactNode;
}

/**
 * FeatureProvider component that provides feature-related data to its children
 */
export function FeatureProvider({rootId, children}: FeatureProviderProps) {
    // State for selected items
    const [selectedFeatureId, setSelectedFeatureId] = useState<number | null>(null);
    const [selectedUseCaseId, setSelectedUseCaseId] = useState<number | null>(null);
    const [selectedDtoId, setSelectedDtoId] = useState<number | null>(null);
    const [selectedDtoFieldId, setSelectedDtoFieldId] = useState<number | null>(null);

    // Add state for handling hook errors
    const [hookError, setHookError] = useState<Error | null>(null);

    // Use the custom hooks for data with error handling
    let featuresData = {
        features: [],
        isLoading: true,
        error: null,
        createFeature: () => {
        },
        updateFeature: (_: FeatureDTO) => {
        },
        reorderFeatures: (_: number[]) => {
        },
        refetch: async () => {
        }
    };

    let useCasesData = {
        useCases: [],
        isLoading: true,
        error: null,
        createUseCase: () => {
        },
        updateUseCase: (_: UseCaseDTO) => {
        },
        reorderUseCases: (_: number[]) => {
        },
        refetch: async () => {
        }
    };

    let dtosData = {
        dtos: [],
        isLoading: true,
        error: null,
        createDto: (_?: string) => {
        },
        updateDto: (_: DtoDTO) => {
        },
        removeDto: (_: number) => {
        },
        getDto: async (_: number) => null,
        refetch: async () => {
        }
    };

    let dtoFieldsData = {
        dtoFields: [],
        isLoading: true,
        error: null,
        createDtoField: (_?: any) => {
        },
        updateDtoField: (_: DtoFieldDTO) => {
        },
        removeDtoField: (_: number) => {
        },
        refetch: async () => {
        }
    };

    try {
        featuresData = useFeatures(rootId);
    } catch (err) {
        const errorMessage = `Error in useFeatures hook: ${err}`;
        logError(errorMessage);
        setHookError(err instanceof Error ? err : new Error('Unknown error in useFeatures hook'));
    }

    try {
        useCasesData = useUseCases(selectedFeatureId);
    } catch (err) {
        const errorMessage = `Error in useUseCases hook: ${err}`;
        logError(errorMessage);
        setHookError(err instanceof Error ? err : new Error('Unknown error in useUseCases hook'));
    }

    try {
        dtosData = useDtos();
    } catch (err) {
        const errorMessage = `Error in useDtos hook: ${err}`;
        logError(errorMessage);
        setHookError(err instanceof Error ? err : new Error('Unknown error in useDtos hook'));
    }

    try {
        dtoFieldsData = useDtoFields(selectedDtoId);
    } catch (err) {
        const errorMessage = `Error in useDtoFields hook: ${err}`;
        logError(errorMessage);
        setHookError(err instanceof Error ? err : new Error('Unknown error in useDtoFields hook'));
    }

    const {
        features,
        isLoading: isLoadingFeatures,
        error: featureError,
        createFeature,
        updateFeature,
        reorderFeatures,
        refetch: refetchFeatures
    } = featuresData;

    const {
        useCases,
        isLoading: isLoadingUseCases,
        error: useCaseError,
        createUseCase,
        updateUseCase,
        reorderUseCases,
        refetch: refetchUseCases
    } = useCasesData;

    const {
        dtos,
        isLoading: isLoadingDtos,
        error: dtoError,
        createDto,
        updateDto,
        removeDto,
        refetch: refetchDtos
    } = dtosData;

    const {
        dtoFields,
        isLoading: isLoadingDtoFields,
        error: dtoFieldError,
        createDtoField,
        updateDtoField,
        removeDtoField,
        refetch: refetchDtoFields
    } = dtoFieldsData;

    // Keep track of the previous features count to detect new feature creation
    const [previousFeaturesCount, setPreviousFeaturesCount] = useState(0);

    // Effect to automatically select newly created features
    useEffect(() => {
        if (features.length > previousFeaturesCount && previousFeaturesCount > 0) {
            // A new feature was created, select the latest one (assuming it has the highest ID)
            const latestFeature = [...features].sort((a, b) => b.id - a.id)[0];
            if (latestFeature) {
                setSelectedFeatureId(latestFeature.id);
                setSelectedUseCaseId(null); // Reset selected use case when feature changes
            }
        }
        setPreviousFeaturesCount(features.length);
    }, [features, previousFeaturesCount]);

    // Function to select a feature
    const selectFeature = (featureId: number | null) => {
        setSelectedFeatureId(featureId);
        setSelectedUseCaseId(null); // Reset selected use case when feature changes
    };

    // Function to select a use case
    const selectUseCase = (useCaseId: number | null) => {
        setSelectedUseCaseId(useCaseId);
    };

    // Function to select a DTO
    const selectDto = (dtoId: number | null) => {
        setSelectedDtoId(dtoId);
        setSelectedDtoFieldId(null); // Reset selected DTO field when DTO changes
    };

    // Function to select a DTO field
    const selectDtoField = (dtoFieldId: number | null) => {
        setSelectedDtoFieldId(dtoFieldId);
    };

    // Function to refetch all data
    const refetchAll = async () => {
        await Promise.all([
            refetchFeatures(),
            refetchUseCases(),
            refetchDtos(),
            refetchDtoFields()
        ]);
    };

    // Create the context value
    const contextValue: FeatureContextValue = {
        // Features data
        features,
        selectedFeatureId,
        isLoadingFeatures,
        featureError,

        // Use Cases data
        useCases,
        selectedUseCaseId,
        isLoadingUseCases,
        useCaseError,

        // DTOs data
        dtos,
        selectedDtoId,
        isLoadingDtos,
        dtoError,

        // DTO Fields data
        dtoFields,
        selectedDtoFieldId,
        isLoadingDtoFields,
        dtoFieldError,

        // Hook error
        hookError,

        // Actions
        selectFeature,
        createFeature,
        updateFeature,
        reorderFeatures,

        selectUseCase,
        createUseCase,
        updateUseCase,
        reorderUseCases,

        selectDto,
        createDto,
        updateDto,
        removeDto,

        selectDtoField,
        createDtoField,
        updateDtoField,
        removeDtoField,

        refetchAll
    };

    // If there's a hook error, render an error message
    if (hookError) {
        return (
            <Alert color="red" title="Error initializing feature data">
                <p>{hookError.message}</p>
                <p>Please try refreshing the page.</p>
            </Alert>
        );
    }

    return (
        <FeatureContext.Provider value={contextValue}>
            {children}
        </FeatureContext.Provider>
    );
}

/**
 * Custom hook to use the FeatureContext
 * @returns The FeatureContext value
 * @throws Error if used outside of a FeatureProvider
 */
export function useFeatureContext() {
    const context = useContext(FeatureContext);

    if (context === undefined) {
        throw new Error('useFeatureContext must be used within a FeatureProvider');
    }

    return context;
}