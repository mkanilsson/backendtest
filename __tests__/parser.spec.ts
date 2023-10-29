import { parseHeader } from "../src/parser";

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
});
