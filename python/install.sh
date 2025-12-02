#!/bin/bash
set -e

echo "🚀 Setting up CC-Ex Python project..."

if ! command -v python3 &> /dev/null; then
    echo "❌ Python 3 is not installed. Please install Python 3 first."
    exit 1
fi

if [ ! -d ".venv" ]; then
    python3 -m venv .venv
fi

source .venv/bin/activate
pip install -r requirements.txt

echo "✅ Setup complete! Run 'source .venv/bin/activate && uvicorn main:app --reload' to start the server."

