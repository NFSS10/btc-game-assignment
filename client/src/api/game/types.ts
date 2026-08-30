type PriceChangeEvent = {
    price: number;
    timestamp: number;
};

type SubmitGuessResponse = {
    accepted: boolean;
    guess: {
        id: string;
        createdAt: number;
        entryPrice: number;
        direction: "up" | "down";
    } | null;
};

export type { PriceChangeEvent, SubmitGuessResponse };
