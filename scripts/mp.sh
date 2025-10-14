#!/usr/bin/env bash
set -euo pipefail

SESSION="bevy-mp"

# Kill any existing session with the same name
tmux kill-session -t $SESSION 2>/dev/null || true

# Create a new detached session
tmux new-session -d -s $SESSION

# --- Pane 0: Server ---
tmux send-keys -t $SESSION "cargo run --no-default-features --features server" C-m

# --- Pane 1: Client 1 ---
tmux split-window -h -t $SESSION
tmux send-keys -t $SESSION:0.1 "cargo run --no-default-features --features client" C-m

# --- Pane 2: Client 2 ---
tmux split-window -v -t $SESSION:0.1
tmux send-keys -t $SESSION:0.2 "cargo run --no-default-features --features client" C-m

# Optional: give the server pane more space
tmux select-layout -t $SESSION tiled

# Attach to the session so you can see everything
tmux attach -t $SESSION