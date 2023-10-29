import readline from "readline";

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
let body: string;

rl.on('line', (line) => {
    if (lineNumber == 0) {
        header = line;
        lineNumber++;
    }
    else if (lineNumber == 1) {
        header = line;
        lineNumber++;
        // TODO: Run the program
    }
});
