import { MantineProvider } from "@mantine/core";

import { Game } from "~/components/widgets";

import "@mantine/core/styles.css";
import "@mantine/charts/styles.css";

function App() {
    return (
        <MantineProvider>
            <Game />
        </MantineProvider>
    );
}

export default App;
