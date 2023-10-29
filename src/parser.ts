import Board from "./board";
import Point from "./point";

export function parseHeader(line: string): [Board, Point] {
    const parts = line.split(",");

    let width = parseInt(parts[0]);
    let height = parseInt(parts[1]);
    if(isNaN(width) || isNaN(height)) throw new Error("Invalid Header")

    let x = parseInt(parts[2]);
    let y = parseInt(parts[3]);
    if(isNaN(x) || isNaN(y)) throw new Error("Invalid Header")

    return [new Board(width, height), new Point(x, y)];
}
