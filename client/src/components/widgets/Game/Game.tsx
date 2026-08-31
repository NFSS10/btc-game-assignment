import clsx from "clsx";
import { useEffect, useState } from "react";
import { LineChart } from "@mantine/charts";
import { Button, Text, Group, Skeleton, Stack } from "@mantine/core";

import { api } from "~/api";
import type { GuessResolvedEvent, PriceChangeEvent, ScoreUpdateEvent } from "~/api/game";
import { GuessesTable } from "~/components/molecules";
import type { Guess } from "~/components/molecules/GuessesTable/types";

import styles from "./styles.module.css";
import { usePlayerSession, useSmoothedPriceLine } from "./hooks";

type Props = {
    className?: string;
};
export default function GameWrapper(props: Props) {
    const { className } = props;

    const { session, isInitializing } = usePlayerSession();

    if (isInitializing) return <Skeleton visible height={400} />;
    if (!session) return <Text color="red">Failed to load player session</Text>;

    const { playerId, score, initialGuesses } = session;

    return <Game className={className} playerId={playerId} initialScore={score} initialGuesses={initialGuesses} />;
}

type GameProps = {
    playerId: string;
    initialScore: number;
    initialGuesses: Guess[];
    className?: string;
};
function Game(props: GameProps) {
    const { playerId, initialScore, initialGuesses, className } = props;

    const { points, isReady: isChartReady, pushLatestPoint } = useSmoothedPriceLine();

    const [score, setScore] = useState<number>(initialScore);
    const [livePrice, setLivePrice] = useState<number | null>(null);

    const [guesses, setGuesses] = useState<Guess[]>(initialGuesses);
    const [isSubmittingGuess, setIsSubmittingGuess] = useState<boolean>(false);

    const hasPendingGuess = guesses.some(guess => !guess.resolvedAt);
    const canGuess = !isSubmittingGuess && !hasPendingGuess;

    const onPriceChange = (event: PriceChangeEvent) => {
        setLivePrice(event.price);

        pushLatestPoint({
            price: event.price,
            timestamp: event.timestamp
        });
    };

    const onGuessResolved = (event: GuessResolvedEvent) => {
        setGuesses(prev =>
            prev.map(guess =>
                guess.id === event.guessId
                    ? {
                          ...guess,
                          resolvedAt: event.resolvedAt,
                          resolvedPrice: event.resolvedPrice,
                          isCorrect: event.isCorrect
                      }
                    : guess
            )
        );
    };

    const onScoreUpdate = (event: ScoreUpdateEvent) => {
        setScore(event.newScore);
    };

    useEffect(() => {
        const subscription = api.game.subscribeToEvents(playerId, {
            onPriceChange: onPriceChange,
            onGuessResolved: onGuessResolved,
            onScoreUpdate: onScoreUpdate
        });
        return () => {
            subscription.unsubscribe();
        };
        // eslint-disable-next-line react-hooks/exhaustive-deps
    }, [playerId]);

    const onSubmitGuess = async (direction: "up" | "down") => {
        if (!canGuess) return;

        setIsSubmittingGuess(true);
        try {
            const response = await api.game.submitGuess(playerId, direction);
            if (!response.accepted) return;

            setGuesses(prev => [...prev, response.guess!]);
        } catch (error) {
            console.error("Error submitting guess:", error);
        } finally {
            setIsSubmittingGuess(false);
        }
    };
    return (
        <div className={clsx(styles.game, className)}>
            <Skeleton visible={!isChartReady} height={400}>
                <div className={styles.chartContainer}>
                    <Text className={styles.livePrice}>{money(livePrice) ?? "Loading..."}</Text>
                    <Text className={styles.score}>Score: {score}</Text>
                    <LineChart
                        className={styles.lineChart}
                        h={400}
                        data={points}
                        dataKey="price"
                        series={[{ name: "price", color: "orange.6" }]}
                        curveType="natural"
                        withXAxis={false}
                        withYAxis={true}
                        withDots={false}
                        withTooltip={true}
                        gridAxis="x"
                        tickLine="none"
                        strokeWidth={4}
                        yAxisProps={{ domain: ["dataMin - 40", "dataMax + 40"] }}
                    />
                </div>
            </Skeleton>
            <Stack>
                <Group w="100%" justify="center" mt="md">
                    <Button
                        onClick={() => onSubmitGuess("up")}
                        disabled={!canGuess}
                        loading={isSubmittingGuess}
                        color="green"
                        size="lg"
                        w="250"
                    >
                        Up
                    </Button>
                    <Button
                        onClick={() => onSubmitGuess("down")}
                        disabled={!canGuess}
                        loading={isSubmittingGuess}
                        size="lg"
                        color="red"
                        w="250"
                    >
                        Down
                    </Button>
                </Group>
                <GuessesTable className={styles.guessesTable} guesses={guesses} />
            </Stack>
        </div>
    );
}

const moneyFormatter = new Intl.NumberFormat("en-US", {
    style: "currency",
    currency: "USD"
});

const money = (value: number | null) => {
    if (value === null) return null;
    return moneyFormatter.format(value);
};
