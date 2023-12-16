# a comment

# prints commands to stderr before running
tut-hello:
	echo 'This is a recipe:'

# stderr before running is suppressed with @
tut-hello2:
	@echo 'this is another recipe'

# recipes stop when cmd fails
tut-publish:
	cargo test
	# tests passed, time to publish
	cargo publish

# dependencies can be defined out of order
tut-dependency1:
	@echo run me first!

tut-dependent: tut-dependency1
	@echo I run after dependency1!

# run multiple tasks 'just <task1_name> <taskx_name...>'

# run the special binary for 08, uses problem input
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