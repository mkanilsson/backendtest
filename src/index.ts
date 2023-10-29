import readline from "readline";
import { parseHeader, parseBody } from "./parser";

import Board from "./board";
import Point from "./point";
import { Command } from "./command";

const rl = readline.createInterface({
    input: process.stdin,
});

// HACK: Javascipt has no good way to read from stdin
//       so I need to keep track of how many lines have
//       been read so I know if it's the header or body,
//       and to know when to execute the "algorithm."
let lineNumber = 0;
let header: string;

rl.on('line', (line) => {
    if (lineNumber == 0) {
        lineNumber++;
        header = line;
    }
    else if (lineNumber == 1) {
        lineNumber++;
        let [board, point] = parseHeader(header);
        let commands = parseBody(line);

        let result = run(board, point, commands);

        if(result == null) console.log("[-1,-1]");
        else console.log(`[${result.x},${result.y}]`);
    }
});

export function run(board: Board, point: Point, commands: Command[]): Point | null {
    for(let cmd of commands) {
        switch(cmd) {
            case "Quit":
                return point;
            case "MoveForward":
                if (!point.moveForward(board)) return null;
                break;
            case "MoveBackward":
                if (!point.moveBackward(board)) return null;
                break;
            case "RotateClockwise":
                point.rotateClockwise();
                break;
            case "RotateCounterClockwise":
                point.rotateCounterClockwise();
                break;
        }
    }

    return point;
}
