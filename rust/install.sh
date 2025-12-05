#!/bin/bash
set -e

echo "🚀 Setting up CC-Ex Rust project..."

if ! command -v cargo &> /dev/null; then
    echo "❌ Rust/Cargo is not installed. Please install Rust first."
    echo "   Visit: https://rustup.rs/"
    exit 1
fi

cargo build

echo "✅ Setup complete! Run 'cargo run' to start the server."

