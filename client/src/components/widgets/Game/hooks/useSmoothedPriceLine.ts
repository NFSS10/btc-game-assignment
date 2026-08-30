"use client";

import { useCallback, useEffect, useRef, useState } from "react";

import type { Point } from "../types";

type HookOptions = {
    initialPoint?: Point;
    pointsUntilReady?: number;
    maxPoints?: number;
    updateIntervalMs?: number;
};

/**
 * Helper hook to manage a smoothed price line experience.
 * @returns An object containing the current points and a function to push the latest point.
 */
const useSmoothedPriceLine = (options: HookOptions = {}) => {
    const { initialPoint, pointsUntilReady = 0, maxPoints = 100, updateIntervalMs = 100 } = options;

    const [points, setPoints] = useState<Point[]>(initialPoint ? [initialPoint] : []);
    const latestPriceRef = useRef<number | null>(initialPoint ? initialPoint.price : null);

    const isReady = points.length >= pointsUntilReady;

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
            // if there's no latest price, don't push a point
            if (latestPriceRef.current === null) return;

            pushLatestPoint({
                price: latestPriceRef.current,
                timestamp: Date.now()
            });
        }, updateIntervalMs);

        return () => clearInterval(interval);
    }, [pushLatestPoint, updateIntervalMs]);

    return {
        points,
        isReady,
        pushLatestPoint,
        setPoints
    };
};

export { useSmoothedPriceLine };
export type { HookOptions };
