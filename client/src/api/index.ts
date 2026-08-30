import { api as gameApi } from "./game";
import { api as playerApi } from "./player";

const api = {
    game: gameApi,
    player: playerApi
};
export { api };
