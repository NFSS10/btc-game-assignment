type SseOptions = {
    /** Event handlers for the SSE connection */
    handlers?: {
        onOpen?: (event: Event) => void;
        onMessage?: (event: MessageEvent) => void;
        onEvent?: (name: string, event: MessageEvent) => void;
        onError?: (event: Event) => void;
    };
    /** List of custom events to listen for */
    events?: string[];
};

type SseUnsubscribe = () => void;

type SseSubscription = {
    /** The EventSource instance for the SSE connection */
    eventSource: EventSource;
    /** Function to unsubscribe from the SSE connection (handles cleanup) */
    unsubscribe: SseUnsubscribe;
};

export type { SseOptions, SseUnsubscribe, SseSubscription };
