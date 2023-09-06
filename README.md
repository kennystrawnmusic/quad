# Pure-Rust command line quadratic equation solver

`quad` is a command-line utility written in Rust that runs the quadratic formula on your computer natively and prints a tuple on the command line containing the two solutions to the equation you want to solve. Instead of needing to take multiple steps, you can use this to do everything in one single step and get right to the solution.

## Installation

```
$ cargo install quad --git https://github.com/kennystrawnmusic/quad
```

## Usage
Assuming that a, b, and c are the coefficients that you want to plug in:
```
$ quad a b c
```
The utility parses the quadratic formula completely in the background, so no need to go any further than this.