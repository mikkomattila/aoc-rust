# AoC-Rust

## Overview

This repository contains my solutions to the Advent of Code 2024 challenges, implemented in Rust. Advent of code can be found from: [link](https://adventofcode.com/).

## Usage

Before running the program, copy the `.env.example` file to `.env` and fill in your Advent of Code session token. You can find it by logging to [Advent of Code](https://adventofcode.com/) and copying the value of the session cookie.

Run the program with Cargo. Specify the day of the challenge you want to execute. Optionally, you can also specify the year. If the year is not provided, the program defaults to 2024.

### Examples

With default year:

```sh
cargo run -- --day 1
```

With explicit year:

```sh
cargo run -- --day  --year 2023
```

## Solutions

### 2022

| Day | Problem                                                 | Solution                              |
| --- | ------------------------------------------------------- | ------------------------------------- |
| 1   | [Calorie Counting](https://adventofcode.com/2022/day/1) | [Source](src/year_2022/day_1_2022.rs) |
