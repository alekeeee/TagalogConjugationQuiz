#!/bin/bash

# Get the directory where the script is located
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"

# Run the executable from the script's directory
gnome-terminal -- bash -c "$SCRIPT_DIR/target/release/quiz; exec bash"

