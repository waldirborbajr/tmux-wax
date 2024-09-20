#!/usr/bin/env bash

CURRENT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

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

print_docker_status() {
  $CURRENT_DIR/bin/tmux-wax --tmux
}

update_frequency() {
  local frequency=$(get_tmux_option "@wax_frequency" "$default_frequency")
  tmux set-option -g status-interval "$frequency"
}

print_module() {
  local icon="$(get_tmux_option "@catppuccin_tmux_wax_icon" "")"
  local color="$(get_tmux_option "@catppuccin_tmux_wax_color" "cyan")"
  local text="$(print_docker_status)"

  local module=""
  module+="#[fg=$color]"
  if [[ -n "$icon" ]]; then
    module+="${icon} "
  fi
  module+="WAX: $text"
  module+="#[default]"

  echo "$module"
}

main() {
  update_frequency

  local format_string="#(${CURRENT_DIR}/tmux-wax.tmux print_module)"

  local current_status_right=$(get_tmux_option "status-right")
  tmux set-option -gq "status-right" "$format_string $current_status_right"
}

update_frequency

if [[ "${BASH_SOURCE[0]}" == "${0}" ]]; then
  if [[ "$1" == "print_module" ]]; then
    print_module
  else
    main
  fi
fi
