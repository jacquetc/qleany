import {Component, ErrorInfo, ReactNode} from 'react';
import {Alert, Button, Group, Stack, Text, Code} from '@mantine/core';
import {IconAlertCircle} from '@tabler/icons-react';
import {error as logError} from '@tauri-apps/plugin-log';

interface Props {
    children: ReactNode;
    fallback?: ReactNode;
}

interface State {
    hasError: boolean;
    error: Error | null;
}

class ErrorBoundary extends Component<Props, State> {
    constructor(props: Props) {
        super(props);
        this.state = {
            hasError: false,
            error: null
        };
    }

    static getDerivedStateFromError(error: Error): State {
        // Update state so the next render will show the fallback UI
        return {hasError: true, error};
    }

    componentDidCatch(error: Error, errorInfo: ErrorInfo): void {
        // Log detailed error information
        console.error('Error caught by ErrorBoundary:', error, errorInfo);
        
        // Extract and log specific properties for better error reporting
        const errorDetails = {
            name: error.name,
            message: error.message,
            stack: error.stack,
            componentStack: errorInfo.componentStack
        };
        
        // Log structured error information to Tauri's logging system
        logError(`ErrorBoundary caught error: ${error.message}`);
        logError(`Error name: ${error.name}`);
        logError(`Error stack: ${error.stack || 'No stack trace available'}`);
        logError(`Component stack: ${errorInfo.componentStack}`);
    }

    handleReset = (): void => {
        this.setState({hasError: false, error: null});
    };

    render(): ReactNode {
        if (this.state.hasError) {
            // You can render any custom fallback UI
            if (this.props.fallback) {
                return this.props.fallback;
            }

            return (
                <Alert
                    icon={<IconAlertCircle size={16}/>}
                    title="Something went wrong"
                    color="red"
                    variant="filled"
                >
                    <Stack gap="md">
                        <Text fw={700}>
                            {this.state.error?.name || 'Error'}
                        </Text>
                        <Text>
                            {this.state.error?.message || 'An unexpected error occurred'}
                        </Text>
                        {this.state.error?.stack && (
                            <Code block style={{ maxHeight: '200px', overflow: 'auto', whiteSpace: 'pre-wrap' }}>
                                {this.state.error.stack}
                            </Code>
                        )}
                        <Group align="right">
                            <Button onClick={this.handleReset} color="red" variant="light">
                                Try Again
                            </Button>
                        </Group>
                    </Stack>
                </Alert>
            );
        }

        return this.props.children;
    }
}

export default ErrorBoundary;