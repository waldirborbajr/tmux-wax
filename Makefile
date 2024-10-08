.PHONY: all build install clean uninstall

all: build

build:
	cargo build --release

install: build
	mkdir -p ~/.tmux/plugins/tmux-wax/bin
	cp target/release/tmux-wax ~/.tmux/plugins/tmux-wax/bin
	cp tmux-wax.tmux ~/.tmux/plugins/tmux-wax/

clean:
	cargo clean

test:
	cargo test

uninstall:
	rm -rf ~/.tmux/plugins/tmux-wax/
