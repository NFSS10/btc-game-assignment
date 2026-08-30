import { Center, MantineProvider, Stack, Text, Title } from "@mantine/core";

import { Game } from "~/components/widgets";

import styles from "./styles.module.css";

import "@mantine/core/styles.css";
import "@mantine/charts/styles.css";

function App() {
    return (
        <MantineProvider>
            <Stack>
                <Title ta="center">BTC Game</Title>
                <Game className={styles.game} />
            </Stack>
        </MantineProvider>
    );
}

export default App;
