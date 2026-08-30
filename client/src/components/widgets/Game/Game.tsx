import { useCallback, useEffect } from "react";
import { LineChart } from "@mantine/charts";

import { api } from "~/api";
import type { PriceChangeEvent } from "~/api/game";

import styles from "./styles.module.css";
import { useSmoothedPriceLine } from "./hooks";

type Props = {};

export default function Game(props: Props) {
    const { points, pushLatestPoint } = useSmoothedPriceLine();

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
        const subscription = api.game.subscribeToEvents({ onPriceChange });
        return () => {
            subscription.unsubscribe();
        };
    }, [onPriceChange]);

    return (
        <div className={styles.game}>
            <h1>Game component!</h1>
            <LineChart
                className={styles.game}
                h={140}
                data={points}
                dataKey="price"
                series={[{ name: "price", color: "orange.6" }]}
                curveType="natural"
                withXAxis={false}
                withYAxis={false}
                withDots={false}
                withTooltip={false}
                gridAxis="none"
                tickLine="none"
                strokeWidth={2.4}
                yAxisProps={{ domain: ["dataMin - 80", "dataMax + 80"] }}
            />
        </div>
    );
}
