#!/bin/bash

echo "Restarting VS Code to apply markdown linting configuration..."

# Kill any existing VS Code processes for this workspace
pkill -f "code.*Leptos-Radix" || true

# Wait a moment
sleep 2

# Restart VS Code with the workspace file
code Leptos-Radix.code-workspace

echo "VS Code restarted with new configuration."
