# run_local_tmux.ps1
# Launch 1 server + 2 clients in a tmux session (assumes tmux is installed in PATH)
$session = "bevy-mp"

# Kill any existing session with the same name (ignore errors)
tmux kill-session -t $session 2>$null | Out-Null

Write-Host "📦 Building server and client..."
Start-Process cargo -ArgumentList "build --no-default-features --features server" -NoNewWindow -Wait
Start-Process cargo -ArgumentList "build --no-default-features --features client" -NoNewWindow -Wait

Write-Host "🟡 Creating tmux session: $session"

# Start new detached session with server
tmux new-session -d -s $session "cargo run --no-default-features --features server"

# Split for client 1
tmux split-window -h -t $session "cargo run --no-default-features --features client"

# Split for client 2
tmux split-window -v -t "$session":0.1 "cargo run --no-default-features --features client"

# Arrange layout
tmux select-layout -t $session tiled

# Attach
Write-Host "🧠 Attaching to tmux session. Press Ctrl+b d to detach."
tmux attach -t $session