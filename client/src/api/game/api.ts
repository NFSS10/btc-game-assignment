import type { SseSubscription } from "../types";
import { post, subscribeToSse } from "../utils";
import type { PriceChangeEvent, SubmitGuessResponse } from "./types";

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

type GuessBody = {
    playerId: string;
    direction: "up" | "down";
};
const submitGuess = async (playerId: string, direction: "up" | "down"): Promise<SubmitGuessResponse> => {
    const response = await post<SubmitGuessResponse, GuessBody>("/game/guess", { playerId, direction });
    return response;
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
    subscribeToEvents,
    submitGuess
};
export { api };
export type { SubscribeToEventsOptions };
