import { MantineProvider } from "@mantine/core";

import { Game } from "~/components/widgets";

import "@mantine/core/styles.css";

function App() {
    return (
        <MantineProvider>
            <Game />
        </MantineProvider>
    );
}

export default App;
