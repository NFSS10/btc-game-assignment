import type { SseOptions, SseSubscription } from "./types";

const SERVER_API_URL = import.meta.env.VITE_SERVER_API_URL ?? "http://localhost:9000/api/v1";

/** Helper function to make GET requests to the API */
const get = async <T>(path: string): Promise<T> => {
    const response = await fetch(`${SERVER_API_URL}${path}`);
    if (!response.ok) {
        throw new Error(`Failed to fetch ${path}: ${response.statusText}`);
    }

    return response.json();
};

/** Helper function to make POST requests to the API */
const post = async <TResponse, TBody>(path: string, body?: TBody): Promise<TResponse> => {
    const reqInit: RequestInit = {
        method: "POST",
        headers: {
            "Content-Type": "application/json"
        }
    };

    try {
        if (body !== undefined) reqInit.body = JSON.stringify(body);
    } catch (error) {
        throw new Error(`Failed to serialize request body for ${path}`, {
            cause: error
        });
    }

    const response = await fetch(`${SERVER_API_URL}${path}`, reqInit);
    if (!response.ok) {
        throw new Error(`Failed to post ${path}: ${response.statusText}`);
    }

    return response.json();
};

/** Helper function to subscribe to Server-Sent Events (SSE) */
const subscribeToSse = (path: string, options: SseOptions = {}): SseSubscription => {
    const url = `${SERVER_API_URL}${path}`;
    const eventSource = new EventSource(url);

    const { handlers, events = [] } = options;

    // ensure no duplicate event listeners are registered
    const uniqueEvents = [...new Set(events)];

    const onOpen = (e: Event) => handlers?.onOpen?.(e);
    const onMessage = (e: MessageEvent) => handlers?.onMessage?.(e);
    const onError = (e: Event) => handlers?.onError?.(e);

    // register the default event listeners
    eventSource.addEventListener("open", onOpen);
    eventSource.addEventListener("error", onError);
    eventSource.addEventListener("message", onMessage);

    // register the custom event listeners
    const eventsListeners = uniqueEvents.map(name => {
        const listener = (e: MessageEvent) => handlers?.onEvent?.(name, e);
        eventSource.addEventListener(name, listener);
        return { name, listener };
    });

    const unsubscribe = () => {
        // remove the custom event listeners
        for (const { name, listener } of eventsListeners) {
            eventSource.removeEventListener(name, listener);
        }

        // remove the default event listeners
        eventSource.removeEventListener("message", onMessage);
        eventSource.removeEventListener("error", onError);
        eventSource.removeEventListener("open", onOpen);

        // close the EventSource connection
        eventSource.close();
    };

    const subscription: SseSubscription = { eventSource, unsubscribe };
    return subscription;
};

export { get, post, subscribeToSse };
