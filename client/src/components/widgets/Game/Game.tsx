import { useCallback, useEffect } from "react";
import { LineChart } from "@mantine/charts";
import { Button, Group, Skeleton } from "@mantine/core";

import { api } from "~/api";
import type { PriceChangeEvent } from "~/api/game";

import styles from "./styles.module.css";
import { useSmoothedPriceLine } from "./hooks";

type Props = {};

export default function Game(props: Props) {
    const { points, isReady, pushLatestPoint } = useSmoothedPriceLine();

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
        const subscription = api.game.subscribeToEvents({
            onPriceChange: onPriceChange
        });
        return () => {
            subscription.unsubscribe();
        };
    }, [onPriceChange]);

    const onUpClick = () => {
        console.log("Up button clicked");
    };

    const onDownClick = () => {
        console.log("Down button clicked");
    };

    return (
        <div className={styles.game}>
            <h1>Game component!</h1>
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
                <Button onClick={onUpClick}>Up</Button>
                <Button onClick={onDownClick}>Down</Button>
            </Group>
        </div>
    );
}
