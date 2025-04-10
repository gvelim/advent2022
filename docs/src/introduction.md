# Advent of Code 2022

This book documents solutions to the [Advent of Code 2022](https://adventofcode.com/2022) programming puzzles implemented in Rust.

## About Advent of Code

Advent of Code is an annual set of Christmas-themed programming puzzles created by Eric Wastl. Each year, starting on December 1st, a new programming puzzle is released every day until December 25th. These puzzles can be solved in any programming language and cover a wide range of algorithms, data structures, and problem-solving techniques.

## Project Structure

This project contains solutions for Advent of Code 2022 implemented in Rust. Each day's puzzle has its own binary in the `src/bin` directory with a corresponding input file. This book provides explanations, walkthroughs, and code snippets for each solution.

## How to Use This Book

You can navigate through the solutions using the sidebar. Each day's solution is organized into:

- **Problem Description**: A summary of the day's challenge
- **Solution Explanation**: A detailed walkthrough of the approach used
- **Code**: The complete implementation with comments

## Running the Solutions

To run any day's solution, use Cargo with the appropriate bin target. For example:

```bash
# Run Day 1's solution
cargo run --bin day1
```

You can also compile and run in release mode for better performance:

```bash
cargo run --release --bin day1
```