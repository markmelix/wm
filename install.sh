#!/usr/bin/env bash
echo "Building the window manager"
cargo build --release

echo "Creating ~/.local/bin directory if it doesn't exist"
mkdir -p ~/.local/bin

echo "Copying window manager executable to the ~/.local/bin directory"
cp ./target/release/wm ~/.local/bin/wm
