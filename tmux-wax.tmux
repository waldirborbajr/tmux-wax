#!/usr/bin/env bash

CURRENT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# Default update frequency in seconds
default_frequency=15

get_tmux_option() {
  local option=$1
  local default_value=$2
  local option_value=$(tmux show-option -gqv "$option")
  if [ -z "$option_value" ]; then
    echo "$default_value"
  else
    echo "$option_value"
  fi
}

set_tmux_option() {
  local option=$1
  local value=$2
  tmux set-option -gq "$option" "$value"
}

print_random_number() {
  local random_number=$($CURRENT_DIR/bin/tmux-wax)
  echo "WAX: $random_number"
}

update_random_number() {
  local frequency=$(get_tmux_option "@wax_frequency" "$default_frequency")
  tmux set-option -g status-interval "$frequency"
}

# Main function for standalone use
main() {
  update_random_number

  local color=$(get_tmux_option "@wax_color" "cyan")
  local format_string="#[fg=$color]#(${CURRENT_DIR}/tmux-wax.tmux print)#[default]"

  local current_status_right=$(get_tmux_option "status-right")
  tmux set-option -gq "status-right" "$format_string $current_status_right"
}

# Check if the script is being sourced or run directly
if [[ "${BASH_SOURCE[0]}" == "${0}" ]]; then
  if [[ "$1" == "print" ]]; then
    print_random_number
  else
    main
  fi
fi
