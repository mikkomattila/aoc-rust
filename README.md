# aoc-rust

## Overview

This repository contains my solutions to Advent of Code challenges, implemented in Rust. Advent of code can be found from: [link](https://adventofcode.com/).

## Available solutions

### 2022

| Day | Problem                                                 | Solution                              |
| --- | ------------------------------------------------------- | ------------------------------------- |
| 1   | [Calorie Counting](https://adventofcode.com/2022/day/1) | [Source](src/year_2022/day_1_2022.rs) |

## Usage

Before running the program, copy the `.env.example` file to `.env` and fill in your Advent of Code session token. You can find it by logging to [Advent of Code](https://adventofcode.com/) and copying the value of the session cookie.

Either build or run the program with Cargo. See examples below.

### Using Make

Run:

```sh
make solution day=1 year=2022
```

Build:

```sh
make build
cd src/target/release/
./aoc-rust --day 1 --year 2022
```

### Using Cargo

Run:

```sh
cargo run -- --day 1
cargo run -- --day  --year 2022
```

Build:

```sh
cargo build -r
cd src/target/release/
./aoc-rust --day 1 --year 2022
```
