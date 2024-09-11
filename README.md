# tmux-wax

A tmux plugin that displays a random number in your status bar, written in Rust.

## Installation

### Using TPM (recommended)

Add the following line to your `~/.tmux.conf`:

```
set -g @plugin 'yourusername/tmux-wax'
```

Press `prefix + I` to fetch and install the plugin.

### Manual Installation

Clone the repository:

```
git clone https://github.com/yourusername/tmux-wax ~/.tmux/plugins/tmux-wax
```

Add the following line to your `~/.tmux.conf`:

```
run-shell ~/.tmux/plugins/tmux-wax/tmux-wax.tmux
```

Reload tmux configuration:

```
tmux source-file ~/.tmux.conf
```

## Configuration

You can customize the plugin behavior with the following options in your `~/.tmux.conf`:

- `@wax_frequency`: Update frequency in seconds (default: 15)
- `@wax_color`: Color of the displayed text (default: cyan)

Example:

```
set -g @wax_frequency 30
set -g @wax_color "blue"
```

## Using with Catppuccin

To use tmux-wax as a Catppuccin module, add it to your Catppuccin configuration in `~/.tmux.conf`:

```
set -g @catppuccin_status_modules_right "... tmux-wax ..."
```

You can customize the Catppuccin integration with these options:

- `@catppuccin_tmux_wax_icon`: Set a custom icon for the module (default: "")
- `@catppuccin_tmux_wax_color`: Set the color for the module (default: cyan)

Example:

```
set -g @catppuccin_tmux_wax_icon "🎲"
set -g @catppuccin_tmux_wax_color "blue"
```

## Building

To build the Rust binary, navigate to the plugin directory and run:

```
cargo build --release
```

## Compatibility

This plugin is compatible with Catppuccin tmux theme and can be used as a Catppuccin module.
