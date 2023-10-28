use std::io::{self, BufRead};

mod board;
mod command;
mod object;
mod parser;

fn main() {
    // stdin should be 2 lines.
    // First the header, comprised of `width,height,x,y`
    // then, the body, comprised of `cmd1,cmd2,...,cmdn`
    let stdin = io::stdin();
    let mut iterator = stdin.lock().lines();

    let (board, object) = parser::parse_header(&iterator.next().unwrap().unwrap());
    let body = parser::parse_body(&iterator.next().unwrap().unwrap());
}
