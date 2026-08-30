type Guess = {
    id: number;
    createdAt: number;
    entryPrice: number;
    direction: "up" | "down";
    resolvedPrice?: number;
    resolvedAt?: number;
    isCorrect?: boolean;
};

export type { Guess };
