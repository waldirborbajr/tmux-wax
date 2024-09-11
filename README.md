# tmux-wax

A tmux plugin that displays a random number in your status bar, written in Rust.

## Installation

### Using TPM (recommended)

Add the following line to your `~/.tmux.conf`:

```
set -g @plugin 'yourusername/tmux-wax'
```

Press `prefix + I` to fetch and install the plugin.

## Configuration

Add these lines to your `~/.tmux.conf`:

```
# tmux-wax settings
set -g @wax_frequency 5  # Update every 5 seconds (adjust as needed)
set -g @catppuccin_tmux_wax_icon "🎲"
set -g @catppuccin_tmux_wax_color "blue"

# For standalone use (if not using Catppuccin)
# set -g status-right '#(~/.tmux/plugins/tmux-wax/tmux-wax.tmux print_module)'

# For use with Catppuccin
set -g @catppuccin_status_modules_right "... tmux-wax ..."
```

## Building

To build the Rust binary, navigate to the plugin directory and run:

```
cargo build --release
```

## Compatibility

This plugin is compatible with Catppuccin tmux theme and can be used as a Catppuccin module.

## Troubleshooting

If the icon or number isn't displaying correctly, try these steps:

1. Ensure the Rust binary is built: `cd ~/.tmux/plugins/tmux-wax && cargo build --release`
2. Reload tmux config: `tmux source-file ~/.tmux.conf`
3. Check tmux options: `tmux show-options -g | grep wax`

If issues persist, please open an issue on the GitHub repository.
