import { Table, Text, Badge } from "@mantine/core";

import type { Guess } from "./types";

type Props = {
    guess: Guess;
    className?: string;
};

export default function TableRow(props: Props) {
    const { guess, className } = props;

    const isResolved = Boolean(guess.resolvedAt);

    return (
        <Table.Tr className={className}>
            <Table.Td>
                <Text size="sm" c="dimmed">
                    {new Date(guess.createdAt).toLocaleTimeString()}
                </Text>
            </Table.Td>
            <Table.Td>
                <Badge variant="light" color={guess.direction === "up" ? "green" : "red"}>
                    {guess.direction.toUpperCase()}
                </Badge>
            </Table.Td>
            <Table.Td>
                <Text size="sm" fw={500}>
                    ${guess.entryPrice.toLocaleString()}
                </Text>
            </Table.Td>
            <Table.Td>
                {isResolved ? (
                    <Text size="sm" fw={500}>
                        ${guess.resolvedPrice?.toLocaleString()}
                    </Text>
                ) : (
                    <Text size="xs" c="dimmed" fs="italic">
                        Pending...
                    </Text>
                )}
            </Table.Td>
            <Table.Td align="right">
                {isResolved ? (
                    <Badge color={guess.isCorrect ? "teal" : "pink"} variant="filled">
                        {guess.isCorrect ? "WIN" : "LOSS"}
                    </Badge>
                ) : (
                    <Badge color="gray" variant="outline">
                        Pending...
                    </Badge>
                )}
            </Table.Td>
        </Table.Tr>
    );
}
