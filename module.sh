#!/usr/bin/env bash

CURRENT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

source "$CURRENT_DIR/tmux-wax.tmux"

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

tmux_wax_module() {
  local index=$1
  local icon="$(get_tmux_option "@catppuccin_tmux_wax_icon" "")"
  local color="$(get_tmux_option "@catppuccin_tmux_wax_color" "cyan")"
  local text="$(print_random_number)"

  local module=""
  module+="#[fg=$color]"
  module+="$icon"
  module+="$text"
  module+="#[default]"

  echo "$module"
}
