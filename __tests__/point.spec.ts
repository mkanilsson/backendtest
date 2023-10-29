import Board from "../src/board";
import { parseHeader } from "../src/parser";
import Point from "../src/point";


describe("point", () => {

    let board: Board;
    let point: Point;

    beforeEach(() => {
        const header = parseHeader("4,4,1,1");
        board = header[0];
        point = header[1];
    })

    it("should move forward correctly - north", () => {
        expect(point.moveForward(board)).toEqual(true);
        expect(point.x).toEqual(1);
        expect(point.y).toEqual(0);
    });

    it("should move backward correctly - north", () => {
        expect(point.moveBackward(board)).toEqual(true);
        expect(point.x).toEqual(1);
        expect(point.y).toEqual(2);
    });

    it("should move forward correctly - east", () => {
        point.direction = "East";

        expect(point.moveForward(board)).toEqual(true);
        expect(point.x).toEqual(2);
        expect(point.y).toEqual(1);
    });

    it("should move backward correctly - east", () => {
        point.direction = "East";

        expect(point.moveBackward(board)).toEqual(true);
        expect(point.x).toEqual(0);
        expect(point.y).toEqual(1);
    });

    it("should move forward correctly - west", () => {
        point.direction = "West";

        expect(point.moveForward(board)).toEqual(true);
        expect(point.x).toEqual(0);
        expect(point.y).toEqual(1);
    });

    it("should move backward correctly - west", () => {
        point.direction = "West";

        expect(point.moveBackward(board)).toEqual(true);
        expect(point.x).toEqual(2);
        expect(point.y).toEqual(1);
    });

    it("should move forward correctly - south", () => {
        point.direction = "South";

        expect(point.moveForward(board)).toEqual(true);
        expect(point.x).toEqual(1);
        expect(point.y).toEqual(2);
    });

    it("should move backward correctly - south", () => {
        point.direction = "South";

        expect(point.moveBackward(board)).toEqual(true);
        expect(point.x).toEqual(1);
        expect(point.y).toEqual(0);
    });
});
