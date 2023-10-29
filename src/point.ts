import Board from "./board";

type Direction = "North" | "East" | "South" | "West";

export default class Point {
    public x: number;
    public y: number;

    public direction: Direction;

    public constructor(x: number, y: number) {
        this.x = x;
        this.y = y;

        this.direction = "North";
    }

    // Returns true if the move was legal (e.g. on the board)
    // else it return false.
    public moveForward(board: Board): boolean {
        switch(this.direction) {
            case "North":
                this.y -= 1;
                break;
            case "South":
                this.y += 1;
                break;
            case "East":
                this.x += 1;
                break;
            case "West":
                this.x -= 1;
                break;
        }

        return board.isLegalPosition(this);
    }

    // Returns true if the move was legal (e.g. on the board)
    // else it return false.
    public moveBackward(board: Board): boolean {
        switch(this.direction) {
            case "North":
                this.y += 1;
                break;
            case "South":
                this.y -= 1;
                break;
            case "East":
                this.x -= 1;
                break;
            case "West":
                this.x += 1;
                break;
        }

        return board.isLegalPosition(this);
    }

    public rotateClockwise() {
        switch(this.direction) {
            case "North":
                this.direction = "East";
                break;
            case "East":
                this.direction = "South";
                break;
            case "South":
                this.direction = "West";
                break;
            case "West":
                this.direction = "North";
                break;
        }
    }

    public rotateCounterClockwise() {
        switch(this.direction) {
            case "North":
                this.direction = "West";
                break;
            case "West":
                this.direction = "South";
                break;
            case "South":
                this.direction = "East";
                break;
            case "East":
                this.direction = "North";
                break;
        }
    }
}
