import Board from "../src/board";
import { parseHeader } from "../src/parser";
import Point from "../src/point";

describe("board", () => {
    it("should return false when out of bounds", () => {
        // Moving north
        let [nboard, npoint] = parseHeader("2,2,1,1");

        expect(npoint.moveForward(nboard)).toEqual(true);
        expect(npoint.y).toEqual(0)

        expect(npoint.moveForward(nboard)).toEqual(false);

        // Moving south
        let [sboard, spoint] = parseHeader("2,2,1,1");

        expect(spoint.moveBackward(sboard)).toEqual(false);

        // Moving west
        let [wboard, wpoint] = parseHeader("2,2,1,1");
        wpoint.direction = "West"

        expect(wpoint.moveForward(wboard)).toEqual(true);
        expect(wpoint.x).toEqual(0)

        expect(wpoint.moveForward(wboard)).toEqual(false);

        // Moving west
        let [eboard, epoint] = parseHeader("2,2,1,1");
        epoint.direction = "East"

        expect(epoint.moveForward(eboard)).toEqual(false);
    });
});
