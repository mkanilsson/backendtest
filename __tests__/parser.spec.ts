import { parseHeader, parseBody } from "../src/parser";

describe("parser", () => {
    it("should parse valid header", () => {
        let [board, point] = parseHeader("1,22,333,4444");

        expect(board.width).toEqual(1);
        expect(board.height).toEqual(22);
        expect(point.x).toEqual(333);
        expect(point.y).toEqual(4444);
    });

    it("shouldn't parse invalid header", () => {
        expect(() => {
            parseHeader("a,b,c,d");
        }).toThrowError("Invalid Header");
    });

    it("should parse valid body", () => {
        let commands = parseBody("1,2,3,4,0");

        expect(commands[0]).toEqual("MoveForward");
        expect(commands[1]).toEqual("MoveBackward");
        expect(commands[2]).toEqual("RotateClockwise");
        expect(commands[3]).toEqual("RotateCounterClockwise");
        expect(commands[4]).toEqual("Quit");
    });

    it("shouldn't parse invalid body", () => {
        expect(() => {
            parseBody("5");
        }).toThrow(Error);
    });
});
