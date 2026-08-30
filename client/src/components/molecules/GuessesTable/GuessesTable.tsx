import { Stack, Table, Title, Text } from "@mantine/core";

import type { Guess } from "./types";
import TableRow from "./TableRow";

type Props = {
    guesses: Guess[];
    className?: string;
};

export default function GuessesTable(props: Props) {
    const { guesses, className } = props;

    return (
        <Stack className={className} gap="xs">
            <Title order={3}>Guesses</Title>
            <Table highlightOnHover verticalSpacing="xs">
                <Table.Thead>
                    <Table.Tr>
                        <Table.Th>Time</Table.Th>
                        <Table.Th>Direction</Table.Th>
                        <Table.Th>Entry</Table.Th>
                        <Table.Th>Resolved</Table.Th>
                        <Table.Th style={{ textAlign: "right" }}>Result</Table.Th>
                    </Table.Tr>
                </Table.Thead>
                <Table.Tbody>
                    {guesses.map(guess => (
                        <TableRow key={guess.id} guess={guess} />
                    ))}
                </Table.Tbody>
            </Table>
        </Stack>
    );
}
