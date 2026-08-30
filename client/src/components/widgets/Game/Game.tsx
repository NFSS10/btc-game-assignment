import { useEffect } from "react";

import { api } from "~/api";

import styles from "./styles.module.css";

type Props = {};

export default function Game(props: Props) {
    useEffect(() => {
        const subscription = api.game.subscribeToEvents();
        return () => {
            subscription.unsubscribe();
        };
    }, []);

    return (
        <div className={styles.game}>
            <h1>Game component!</h1>
        </div>
    );
}
