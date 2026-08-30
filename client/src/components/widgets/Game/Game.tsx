import { useCallback, useEffect, useState } from "react";
import { LineChart } from "@mantine/charts";
import { Button, Group, List, Skeleton } from "@mantine/core";

import { api } from "~/api";
import type { GuessResolvedEvent, PriceChangeEvent, ScoreUpdateEvent } from "~/api/game";

import styles from "./styles.module.css";
import { usePlayerSession, useSmoothedPriceLine } from "./hooks";
import type { Guess } from "./types";

type Props = {};

export default function Game(props: Props) {
    const { points, isReady, pushLatestPoint } = useSmoothedPriceLine();
    const { session, isInitializing } = usePlayerSession();

    const [livePrice, setLivePrice] = useState<number | null>(null);
    const [score, setScore] = useState<number>(0);
    const [guesses, setGuesses] = useState<Guess[]>([]);

    const [isSubmittingGuess, setIsSubmittingGuess] = useState<boolean>(false);

    const canGuess = Boolean(session) && !isInitializing && !isSubmittingGuess;

    const onPriceChange = useCallback(
        (event: PriceChangeEvent) => {
            // update the live price
            setLivePrice(event.price);

            // push the latest point to the smoothed price line
            pushLatestPoint({
                price: event.price,
                timestamp: event.timestamp
            });
        },
        [pushLatestPoint]
    );

    const onGuessResolved = useCallback((event: GuessResolvedEvent) => {
        console.log("Guess resolved event received:", event);
    }, []);

    const onScoreUpdate = useCallback((event: ScoreUpdateEvent) => {
        console.log("Score update event received:", event);
        setScore(event.newScore);
    }, []);

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
    }, [session?.playerId, onPriceChange, onGuessResolved, onScoreUpdate]);

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
        <div className={styles.game}>
            <Skeleton visible={isInitializing} height={600}>
                <h1>Score: {score}</h1>
                <h1>Live Price: {livePrice ?? "Loading..."}</h1>
                <Skeleton visible={!isReady} height={500}>
                    <LineChart
                        className={styles.lineChart}
                        h={500}
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
                        yAxisProps={{ domain: ["dataMin - 50", "dataMax + 50"] }}
                    />
                </Skeleton>
                <Group>
                    <Button onClick={() => onSubmitGuess("up")} disabled={!canGuess} loading={isSubmittingGuess}>
                        Up
                    </Button>
                    <Button onClick={() => onSubmitGuess("down")} disabled={!canGuess} loading={isSubmittingGuess}>
                        Down
                    </Button>
                </Group>
                <List>
                    {guesses.map(guess => (
                        <List.Item key={guess.id}>
                            {new Date(guess.createdAt).toLocaleTimeString()} - {guess.direction} - {guess.entryPrice}
                        </List.Item>
                    ))}
                </List>
            </Skeleton>
        </div>
    );
}
