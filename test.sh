#!/bin/bash

# Test script for File Manager
# This script initializes a local SQLite DB and starts the File Manager server for testing

set -e

# Ensure data directory exists and initialize SQLite DB file
DATA_DIR="./data"
mkdir -p "$DATA_DIR"
DB_FILE="${DATABASE_FILE:-$DATA_DIR/fm.db}"
if [ ! -f "$DB_FILE" ]; then
    echo "Initializing SQLite database at $DB_FILE"
    touch "$DB_FILE"
fi

# Check if the binary exists
if [ ! -f "target/debug/fm" ] && [ ! -f "target/release/fm" ]; then
    echo "Binary not found. Building..."
    ./build.sh
fi

# Determine which binary to use
BINARY="target/debug/fm"
if [ -f "target/release/fm" ]; then
    BINARY="target/release/fm"
fi

echo "Using binary: $BINARY"

# Create a test directory for local filesystem
TEST_DIR="/tmp/fm-test"
mkdir -p "$TEST_DIR"
echo "Test directory created: $TEST_DIR"

# Initialize / report SQLite DB (no external DB server required)
echo "Using SQLite DB at $DB_FILE"

echo ""
echo "SQLite DB initialized at $DB_FILE"
echo "Test directory: $TEST_DIR"
echo ""
echo "Starting File Manager server..."
echo "Access the application at: http://127.0.0.1:8080"
echo ""
echo "Press Ctrl+C to stop both services"
echo ""

# Function to cleanup on exit
cleanup() {
    echo ""
    echo "Shutting down..."
    kill ${FM_PID:-} 2>/dev/null || true
    echo "Cleanup complete"
}

trap cleanup EXIT INT TERM

# Start the File Manager server
DATABASE_URL=127.0.0.1:8000 \
JWT_SECRET=test-secret-key \
ENCRYPTION_KEY=test-32-byte-encryption-key!! \
HOST=127.0.0.1 \
PORT=8080 \
RUST_LOG=info \
"$BINARY"
