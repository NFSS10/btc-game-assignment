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
        setIsInitializing(true);

        loadSession()
            .then(session => {
                setSession(session);
                setIsInitializing(false);
            })
            .finally(() => {
                setIsInitializing(false);
            });
    }, [loadSession]);

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
