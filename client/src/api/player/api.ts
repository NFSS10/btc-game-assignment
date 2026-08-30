import { post } from "../utils";
import type { PlayerState } from "./types";

type InitBody = {
    playerId?: string;
};
const init = async (playerId: string | null): Promise<PlayerState> => {
    const body = playerId ? { playerId } : {};
    const state = await post<PlayerState, InitBody>("/players/init", body);
    return state;
};

const api = {
    init
};
export { api };
export type { InitBody };
