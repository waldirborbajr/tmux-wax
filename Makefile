.PHONY: all build install

all: build

build:
	cargo build --release

install: build
	mkdir -p ~/.tmux/plugins/tmux-wax/bin
	cp target/release/tmux-wax ~/.tmux/plugins/tmux-wax/bin
	cp tmux-wax.tmux ~/.tmux/plugins/tmux-wax/
	cp module.sh ~/.tmux/plugins/tmux-wax/

clean:
	cargo clean
