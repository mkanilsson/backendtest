import { run } from "../src/index";
import { parseBody, parseHeader } from "../src/parser";

describe("run", () => {
    it("should return a point if the game is valid", () => {
        let [board, point] = parseHeader("4,4,2,2");
        let commands = parseBody("1,4,1,3,2,3,2,4,1,0");
        let result = run(board, point, commands);

        console.log(result);

        expect(result).toBeTruthy();
        expect(result!.x).toEqual(0);
        expect(result!.y).toEqual(1);
    });

    it("should return null if the game is invalid", () => {
        let [board, point] = parseHeader("4,4,2,2");
        // move of the board
        let commands = parseBody("1,1,1,1,1,1,0");
        let result = run(board, point, commands);

        expect(result).toBeNull();
    });
})
