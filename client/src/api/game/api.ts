import type { SseSubscription } from "../types";
import { subscribeToSse } from "../utils";

const subscribeToEvents = (): SseSubscription => {
    const subscription = subscribeToSse("/game/events", {
        handlers: {
            onOpen: () => console.log("Connected to game events"),
            onMessage: event => console.log("Received message:", event.data),
            onError: event => console.error("Error in SSE connection:", event),
            onEvent: (name, event) => {
                // TODO: act on this
                console.log("EVENT", name, event);
            }
        },
        events: ["price_change"]
    });

    return subscription;
};

const api = {
    subscribeToEvents: subscribeToEvents
};
export { api };
export type {};
