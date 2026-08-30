import { useEffect, useState } from "react";

import { api } from "~/api";

const PLAYER_ID_KEY = "btc-game:player-id";

type PlayerSessionState = {
    playerId: string;
    score: number;
};

const usePlayerSession = () => {
    const [session, setSession] = useState<PlayerSessionState | null>(null);
    const [isInitializing, setIsInitializing] = useState(true);

    useEffect(() => {
        let isMounted = true;

        loadSession()
            .then(sessionData => {
                if (isMounted) setSession(sessionData);
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

const loadSession = async (): Promise<PlayerSessionState> => {
    const existingPlayerId: string | null = localStorage.getItem(PLAYER_ID_KEY);

    // if the player exists it will return the player state, otherwise it will create
    // a new player and return the state
    const playerState = await api.player.init(existingPlayerId);

    // store the player id in local storage for future sessions
    localStorage.setItem(PLAYER_ID_KEY, playerState.id);

    const session: PlayerSessionState = {
        playerId: playerState.id,
        score: playerState.score
    };
    return session;
};

export { usePlayerSession };
