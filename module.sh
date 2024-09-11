#!/usr/bin/env bash

CURRENT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

source "$CURRENT_DIR/tmux-wax.tmux"

tmux_wax_module() {
  local index=$1
  local icon="$(get_tmux_option "@catppuccin_tmux_wax_icon" "")"
  local color="$(get_tmux_option "@catppuccin_tmux_wax_color" "cyan")"
  local text="$(print_random_number)"

  local module=""
  module+="#[fg=$color]"
  if [[ -n "$icon" ]]; then
    module+="${icon} "
  fi
  module+="WAX: $text"
  module+="#[default]"

  echo "$module"
}

update_frequency
