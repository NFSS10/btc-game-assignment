import { get, post } from "../utils";
import type { Guess, PlayerState } from "./types";

type InitBody = {
    playerId?: string;
};
const init = async (playerId: string | null): Promise<PlayerState> => {
    const body = playerId ? { playerId } : {};
    const state = await post<PlayerState, InitBody>("/players/init", body);
    return state;
};

const listGuesses = async (playerId: string): Promise<Guess[]> => {
    const guesses = await get<Guess[]>(`/players/${playerId}/guesses`);
    return guesses;
};

const api = {
    init,
    listGuesses
};
export { api };
export type { InitBody };
