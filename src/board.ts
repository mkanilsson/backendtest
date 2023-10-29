import Point from "./point";

export default class Board {
    public width: number;
    public height: number;

    public constructor(width: number, height: number) {
        this.width = width;
        this.height = height;
    }

    public isLegalPosition(point: Point): boolean {
        if(point.y < 0 || point.y >= this.height) return false;
        if(point.x < 0 || point.x >= this.width) return false;
        return true;
    }
}
