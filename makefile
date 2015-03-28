# globals
default: build
freshen: clean build
clean:
	cargo clean

# vars

# commands
build:
	cargo build

package:
	cargo package

# tests
test:
	cargo test
