"use client";

import { useCallback, useEffect, useRef, useState } from "react";

import type { Point } from "../types";

type HookOptions = {
    initialPoint?: Point;
    maxPoints?: number;
    updateIntervalMs?: number;
};

/**
 * Helper hook to manage a smoothed price line experience.
 * @returns An object containing the current points and a function to push the latest point.
 */
const useSmoothedPriceLine = (options: HookOptions = {}) => {
    const { initialPoint = { price: 0, timestamp: 0 }, maxPoints = 100, updateIntervalMs = 100 } = options;

    const [points, setPoints] = useState<Point[]>([initialPoint]);
    const latestPriceRef = useRef<number>(initialPoint.price);

    const pushLatestPoint = useCallback(
        (point: Point) => {
            latestPriceRef.current = point.price;

            // push the latest point to the points array while keeping the array length within maxPoints
            setPoints(prev => {
                const next = [...prev, point];
                return next.length > maxPoints ? next.slice(next.length - maxPoints) : next;
            });
        },
        [maxPoints]
    );

    // push points at a regular interval
    useEffect(() => {
        const interval = window.setInterval(() => {
            pushLatestPoint({
                price: latestPriceRef.current,
                timestamp: Date.now()
            });
        }, updateIntervalMs);

        return () => clearInterval(interval);
    }, [pushLatestPoint, updateIntervalMs]);

    return {
        points,
        pushLatestPoint
    };
};

export { useSmoothedPriceLine };
export type { HookOptions };
