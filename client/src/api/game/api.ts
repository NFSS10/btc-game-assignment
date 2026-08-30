import type { SseSubscription } from "../types";
import { post, subscribeToSse } from "../utils";
import type { PriceChangeEvent, SubmitGuessResponse, GuessResolvedEvent, ScoreUpdateEvent } from "./types";

type SubscribeToEventsOptions = {
    onOpen?: () => void;
    onError?: (event: Event) => void;
    onPriceChange?: (event: PriceChangeEvent) => void;
    onGuessResolved?: (event: GuessResolvedEvent) => void;
    onScoreUpdate?: (event: ScoreUpdateEvent) => void;
};
const subscribeToEvents = (playerId: string, options: SubscribeToEventsOptions = {}): SseSubscription => {
    const { onOpen, onError, onPriceChange, onGuessResolved, onScoreUpdate } = options;

    const subscription = subscribeToSse(`/game/events?playerId=${playerId}`, {
        handlers: {
            onOpen: onOpen,
            onError: onError,
            onEvent: (name, event) => {
                switch (name) {
                    case "price_change": {
                        const data = safeEventDataParse<PriceChangeEvent>(event);
                        if (data) onPriceChange?.(data);
                        return;
                    }
                    case "guess_resolved": {
                        const data = safeEventDataParse<GuessResolvedEvent>(event);
                        if (data) onGuessResolved?.(data);
                        return;
                    }
                    case "score_update": {
                        const data = safeEventDataParse<ScoreUpdateEvent>(event);
                        if (data) onScoreUpdate?.(data);
                        return;
                    }
                    default:
                        console.warn(`Unhandled event: ${name}`, event);
                }
            }
        },
        events: ["price_change", "guess_resolved", "score_update"]
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
