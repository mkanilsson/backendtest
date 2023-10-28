# Backend test

Time it took too complete: `1h 27min` (between first commit `Initial commit 19394231` and `Add test for run() 3c94c958` + like `5min` to add tests for `Direction`)

## How to run

Use `bash`/`zsh`/`fish` or `git bash` and `cd` into the project and run:

```bash
cat testinput.txt | cargo run
```

> NOTE: `testinput.txt` can be edited to test different commands and boards.

To run the test, just run:

``` sh
cargo test
```

## Assumptions made

The input format is:

``` sh
width,height,x,y
cmd1,cmd2,cmd3,...,cmdn
```

`cmd` can be one of `0`, `1`, `2`, `3` or `4`.

If the input is malformatted, the program will panic.

If `width`, `height`, `x`, `y` isn't a number or if a `command` is not one of the valid ones, the program will panic.

If the `object` is out-of-bounds at any point in the execution, the program will halt and not process the remaining commands.

## Things to keep in mind

### Handle different shapes than a rectangle

This could easily be added by changing `Board` to a trait and pass a `Box<dyn Board>` around instead of `Board`. Then implement, for example `Rectagle` by implementing the `Board` trait for `Rectagle` and write a custom `is_legal_position`.

### Add more commands like rotating the table instead of the object

This could be done by just adding more variants to the `Command` trait then implement them in `run()`.

### Change the binary form of the protocol to JSON

Update the `parse_header()` and `parse_body()` functions and parse it like json via `serde_json` or something similar.

The output would have to be changed in `main()` as well.

## Test coverage:

[![codecov.io Code Coverage](https://img.shields.io/codecov/c/github/mkanilsson/backendtest.svg)](https://codecov.io/github/mkanilsson/backendtest?branch=master)

The logic in `main` is so simple and there is no resonable way to extract the functionallity that `main` provide to a more testable function.
