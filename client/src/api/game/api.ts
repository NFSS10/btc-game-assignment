import type { SseSubscription } from "../types";
import { subscribeToSse } from "../utils";
import type { PriceChangeEvent } from "./types";

type SubscribeToEventsOptions = {
    onOpen?: () => void;
    onError?: (event: Event) => void;
    onPriceChange?: (event: PriceChangeEvent) => void;
};
const subscribeToEvents = (options: SubscribeToEventsOptions = {}): SseSubscription => {
    const { onOpen, onError, onPriceChange } = options;

    const subscription = subscribeToSse("/game/events", {
        handlers: {
            onOpen: onOpen,
            onError: onError,
            onEvent: (name, event) => {
                switch (name) {
                    case "price_change":
                        const data = safeEventDataParse<PriceChangeEvent>(event);
                        if (data) onPriceChange?.(data);
                        break;
                    default:
                        console.warn(`Unhandled event: ${name}`, event);
                }
            }
        },
        events: ["price_change"]
    });

    return subscription;
};

const safeEventDataParse = <T>(event: MessageEvent): T | null => {
    try {
        return JSON.parse(event.data);
    } catch (error) {
        console.error("Failed to parse event data:", event.data, error);
        return null;
    }
};

const api = {
    subscribeToEvents: subscribeToEvents
};
export { api };
export type { SubscribeToEventsOptions };
