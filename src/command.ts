export type Command = "Quit" | "MoveForward" | "MoveBackward" | "RotateClockwise" | "RotateCounterClockwise";

export function commandFromString(value: string): Command {
    switch(value) {
        case "0":
            return "Quit";
        case "1":
            return "MoveForward";
        case "2":
            return "MoveBackward";
        case "3":
            return "RotateClockwise";
        case "4":
            return "RotateCounterClockwise";
        default:
            throw new Error(`Unknown Command '${value}'`);
    }
}
