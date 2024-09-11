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

update_random_number() {
  local frequency=$(get_tmux_option "@wax_frequency" "$default_frequency")
  tmux set-option -g status-interval "$frequency"

  local format_string="#[fg=#{@wax_color}]WAX: %random_number#[default]"
  tmux set-option -gq "status-right" "#($CURRENT_DIR/bin/tmux-wax '$format_string')$(get_tmux_option status-right)"
}

main() {
  local color=$(get_tmux_option "@wax_color" "cyan")
  set_tmux_option "@wax_color" "$color"

  update_random_number
}

main
