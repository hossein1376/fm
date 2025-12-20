#!/bin/bash
set -e

echo "Building File Manager..."

# Build frontend
echo "Building frontend..."
cd frontend
if [ ! -d "node_modules" ]; then
    echo "Installing frontend dependencies with bun..."
    bun install
fi
bun run build
cd ..

# Build backend
echo "Building backend (release mode)..."
cargo build --release

echo ""
echo "Build complete!"
echo "Binary location: target/release/fm"
echo "Binary size: $(du -h target/release/fm | cut -f1)"
echo ""
echo "To run the server:"
echo "  DATABASE_URL=127.0.0.1:8000 ./target/release/fm"
