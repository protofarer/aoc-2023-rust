# run multiple tasks `just <task1_name> <taskx_name...>`

# run the special binary, uses problem input
d day:
	cargo run --bin {{day}} -- input/{{day}}.txt

# run special binary with no input
dx day:
	cargo run --bin {{day}}

# run special binary with default 'dev' input file
dd day:
	cargo run --bin {{day}} -- data/{{day}}.txt

# run special binary with specified input file
df day file:
	cargo run --bin {{day}} -- data/{{file}}.txt

new-special day:
	mkdir -p src/bin/{{day}}
	cp -r template_special.rs src/bin/{{day}}/main.rs
	sed -i 's|// INJECT_USE_DAY_MODULE|use aoc_2023_rust::day{{day}}::*;|' src/bin/{{day}}/main.rs

new-day day:
	cp template_day.rs src/day{{day}}.rs