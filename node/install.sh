#!/bin/bash
set -e

echo "🚀 Setting up CC-Ex Node.js project..."

if ! command -v node &> /dev/null; then
    echo "❌ Node.js is not installed. Please install Node.js first."
    exit 1
fi

npm install

echo "✅ Setup complete! Run 'npm start' to start the server."

