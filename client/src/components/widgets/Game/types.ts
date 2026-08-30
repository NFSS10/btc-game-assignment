type Point = {
    price: number;
    timestamp: number;
};

type Guess = {
    id: string;
    createdAt: number;
    entryPrice: number;
    direction: "up" | "down";
};

export type { Point, Guess };
