import { useEffect, useState } from "react";

import { api } from "~/api";
import type { Guess } from "~/api/player";

const PLAYER_ID_KEY = "btc-game:player-id";

type PlayerSessionState = {
    playerId: string;
    score: number;
    initialGuesses: Guess[];
};

const usePlayerSession = () => {
    const [session, setSession] = useState<PlayerSessionState | null>(null);
    const [isInitializing, setIsInitializing] = useState(true);

    useEffect(() => {
        let isMounted = true;

        loadSessionState()
            .then(data => {
                if (!isMounted) return;
                setSession({
                    playerId: data.playerId,
                    score: data.score,
                    initialGuesses: data.initialGuesses
                });
            })
            .catch(error => {
                console.error("Failed to load player session:", error);
            })
            .finally(() => {
                if (isMounted) setIsInitializing(false);
            });

        return () => {
            // prevents updating state if the component unmounts mid-fetch
            isMounted = false;
        };
    }, []);

    return {
        session,
        isInitializing
    };
};

const loadSessionState = async (): Promise<PlayerSessionState> => {
    const existingPlayerId: string | null = localStorage.getItem(PLAYER_ID_KEY);

    // if the player exists it will return the player state, otherwise it will create
    // a new player and return the state
    const playerState = await api.player.init(existingPlayerId);

    // now fetch the initial guesses state for the player
    const guesses = await api.player.listGuesses(playerState.id);

    // store the player id in local storage for future sessions
    localStorage.setItem(PLAYER_ID_KEY, playerState.id);

    const session: PlayerSessionState = {
        playerId: playerState.id,
        score: playerState.score,
        initialGuesses: guesses
    };
    return session;
};

export { usePlayerSession };
