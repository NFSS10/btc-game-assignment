type PriceChangeEvent = {
    price: number;
    timestamp: number;
};

type GuessResolvedEvent = {
    guessId: number;
    playerId: string;
    entryPrice: number;
    direction: "up" | "down";
    createdAt: number;
    resolvedPrice: number;
    resolvedAt: number;
    isCorrect: boolean;
};

type ScoreUpdateEvent = {
    playerId: string;
    newScore: number;
};

type SubmitGuessResponse = {
    accepted: boolean;
    guess: {
        id: number;
        createdAt: number;
        entryPrice: number;
        direction: "up" | "down";
    } | null;
};

export type { PriceChangeEvent, GuessResolvedEvent, ScoreUpdateEvent, SubmitGuessResponse };
