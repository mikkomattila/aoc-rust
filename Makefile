d ?= 1
y ?= 2024

# Solution for specified day and year
get:
	cargo run -- --day $(d) --year $(y)

# Test specified day and year
test:
	cargo test --lib year_$(y)::day_$(d) -- --nocapture

# Build the project
build:
	cargo build -r

# Run the project
run:
	./target/release/aoc-rust --day $(d) --year $(y)
