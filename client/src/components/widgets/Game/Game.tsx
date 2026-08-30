import { useCallback, useEffect, useState } from "react";
import { LineChart } from "@mantine/charts";
import { Button, Group, Skeleton } from "@mantine/core";

import { api } from "~/api";
import type { PriceChangeEvent } from "~/api/game";

import styles from "./styles.module.css";
import { usePlayerSession, useSmoothedPriceLine } from "./hooks";

type Props = {};

export default function Game(props: Props) {
    const { points, isReady, pushLatestPoint } = useSmoothedPriceLine();
    const { session, isInitializing } = usePlayerSession();

    const [score, setScore] = useState<number>(0);

    const canPlay = Boolean(session) && !isInitializing;

    const onPriceChange = useCallback(
        (event: PriceChangeEvent) => {
            pushLatestPoint({
                price: event.price,
                timestamp: event.timestamp
            });
        },
        [pushLatestPoint]
    );

    useEffect(() => {
        if (!session?.playerId) return;

        setScore(session.score);

        const subscription = api.game.subscribeToEvents({
            onPriceChange: onPriceChange
        });
        return () => {
            subscription.unsubscribe();
        };
    }, [session?.playerId, onPriceChange]);

    const onUpClick = () => {
        if (!canPlay) return;

        console.log("Up button clicked");
    };

    const onDownClick = () => {
        if (!canPlay) return;

        console.log("Down button clicked");
    };

    return (
        <div className={styles.game}>
            <Skeleton visible={isInitializing} height={600}>
                <h1>Score: {score}</h1>
                <Skeleton visible={!isReady} height={500}>
                    <LineChart
                        p={40}
                        className={styles.game}
                        h={500}
                        data={points}
                        dataKey="price"
                        series={[{ name: "price", color: "orange.6" }]}
                        curveType="natural"
                        withXAxis={false}
                        withYAxis={true}
                        withDots={false}
                        withTooltip={true}
                        gridAxis="none"
                        tickLine="none"
                        strokeWidth={2.4}
                        yAxisProps={{ domain: ["dataMin - 50", "dataMax + 50"] }}
                    />
                </Skeleton>
                <Group>
                    <Button onClick={onUpClick} disabled={!canPlay}>
                        Up
                    </Button>
                    <Button onClick={onDownClick} disabled={!canPlay}>
                        Down
                    </Button>
                </Group>
            </Skeleton>
        </div>
    );
}
