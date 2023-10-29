# Backend test

Time it took too complete: `1h 28min` (between first commit `Initial commit f0a4fd14` and `Add tests for Direction a2371127` + like `10min` to get codecov code coverage working correctly) 

Commits before `f0a4df11` are Rust.

## Commits

* [Initial Commit](https://github.com/mkanilsson/backendtest/commit/f0a4fd1468c81aba1caab11dedb302d5690d08fe)
* [Parse Header](https://github.com/mkanilsson/backendtest/commit/731cb215fe36a7e35ae17878c21f712e222d48d3)
* [Parse Commands](https://github.com/mkanilsson/backendtest/commit/c40626de9af604a983d6704b9cc69535fd48ed23)
* [Implement the Algorithm](https://github.com/mkanilsson/backendtest/commit/82b425f4e3eab3b2a921ab8e3ade978dc562ce2b)
* [Add tests for Board and Point](https://github.com/mkanilsson/backendtest/commit/567b524b97e7118d01947bb855d9f7ff6c5d285e)
* [Add tests for run()](https://github.com/mkanilsson/backendtest/commit/884f5c0bc17972cef1d76e6f53326855854ad612)
* [Add test for Direction](https://github.com/mkanilsson/backendtest/commit/884f5c0bc17972cef1d76e6f53326855854ad612)

## How to run

First get all the dependencies with `npm i`, then use `bash`/`zsh`/`fish` or `git bash` run:

```bash
cat testinput.txt | npm run dev
```

> NOTE: `testinput.txt` can be edited to test different commands and boards.

To run the test, just run:

``` sh
npm test
```

## Assumptions made

The input format is:

``` sh
width,height,x,y
cmd1,cmd2,cmd3,...,cmdn
```

`cmd` can be one of `0`, `1`, `2`, `3` or `4`.

If the input is malformatted, the program will throw.

If `width`, `height`, `x`, `y` isn't a number or if a `command` is not one of the valid ones, the program will throw.

> NOTE: `Object` is a taken word in JavaScript so the word `Point` is used instead.

If the `point` is out-of-bounds at any point in the execution, the program will halt and not process the remaining commands.

## Things to keep in mind

### Handle different shapes than a rectangle

This could easily be added by changing `Board` to a interface that has a `isLegalPosition` function which other classes (e.g. `Rectangle`) could implement. Then, instead of receving `Board` in functions, it could take the interface.

### Add more commands like rotating the table instead of the object

This could be done by just adding more variants to the `Command` type then implement them in `run()`.

### Change the binary form of the protocol to JSON

Update the `parseHeader()` and `parseBody()` functions and parse it like json via `JSON.stringify`.

The output would have to be changed in `rl.on('line')` as well.

## Test coverage:

[![codecov](https://codecov.io/github/mkanilsson/backendtest/branch/typescript/graph/badge.svg?token=N8ZC782EDK)](https://codecov.io/github/mkanilsson/backendtest/tree/typescript)

Some parts of `index.ts` aren't covered, the reason is that there is no resonable way to extract the functionality to a more testable function, doing that would result in a bigger mess than neccessary.
