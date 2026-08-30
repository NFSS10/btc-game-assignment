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

export default function Game(props: Props) {
    const { className } = props;

    const { points, isReady, pushLatestPoint } = useSmoothedPriceLine();
    const { session, isInitializing } = usePlayerSession();

    const [livePrice, setLivePrice] = useState<number | null>(null);
    const [score, setScore] = useState<number>(0);
    const [guesses, setGuesses] = useState<Guess[]>([]);

    const [isSubmittingGuess, setIsSubmittingGuess] = useState<boolean>(false);

    const isLoading = isInitializing || !isReady;
    const hasPendingGuess = guesses.some(guess => !guess.resolvedAt);
    const canGuess = Boolean(session) && !isInitializing && !isSubmittingGuess && !hasPendingGuess;

    const onPriceChange = (event: PriceChangeEvent) => {
        // update the live price
        setLivePrice(event.price);

        // push the latest point to the smoothed price line
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
        console.log("Score update event received:", event);
        setScore(event.newScore);
    };

    useEffect(() => {
        const playerId = session?.playerId;
        if (!playerId) return;

        setScore(session.score);

        const subscription = api.game.subscribeToEvents(playerId, {
            onPriceChange: onPriceChange,
            onGuessResolved: onGuessResolved,
            onScoreUpdate: onScoreUpdate
        });
        return () => {
            subscription.unsubscribe();
        };
    }, [session?.playerId]);

    const onSubmitGuess = async (direction: "up" | "down") => {
        if (!canGuess) return;

        setIsSubmittingGuess(true);
        try {
            const response = await api.game.submitGuess(session!.playerId, direction);
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
            <Skeleton visible={isLoading} height={400}>
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
