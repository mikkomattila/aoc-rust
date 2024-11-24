# aoc-rust

## Overview

This repository contains my solutions to Advent of Code challenges, implemented in Rust. Advent of code can be found from: [link](https://adventofcode.com/).

## Available solutions

### 2022

| Day | Problem                                                    | Solution                              |
| --- | ---------------------------------------------------------- | ------------------------------------- |
| 1   | [Calorie Counting](https://adventofcode.com/2022/day/1)    | [Source](src/year_2022/day_1_2022.rs) |
| 2   | [Rock Paper Scissors](https://adventofcode.com/2022/day/2) | [Source](src/year_2022/day_2_2022.rs) |

## Prerequisites and usage

Make sure you have Rust and Cargo installed.

Before running the program, copy the `.env.example` file to `.env` and fill in your Advent of Code session token. You can find it by logging to [Advent of Code](https://adventofcode.com/) and copying the value of the session cookie.

Either run or build the program with Cargo. See examples below.

### Examples

**Run:**

```sh
cargo run -- --day 1 --year 2022
```

**Build:**

1. Build with release configuration

```sh
cargo build --release
```

2. Produced binary can be found from `src/target/release` and ran with the following command:

```sh
./aoc-rust --day 1 --year 2022
```
