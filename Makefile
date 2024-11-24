
# Build the project
build:
	cargo build -r

# Test the project
test:
	cargo test

# Get solution for a specific day
solution:
	cargo run -- --day $(day) --year $(year)
