import readline from "readline";
import { parseHeader, parseBody } from "./parser";

const rl = readline.createInterface({
    input: process.stdin,
    output: process.stdout,
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
    }
});
