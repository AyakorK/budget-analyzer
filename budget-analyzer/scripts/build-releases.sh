#!/bin/bash
# Build binaries for all platforms

set -e

VERSION=${1:-"0.1.0"}

echo "🔨 Building Budget Analyzer v${VERSION}"
echo "========================================"

cd budget-analyzer

mkdir -p ../releases

# macOS ARM64
echo "📦 Building macOS ARM64..."
cargo build --release --target aarch64-apple-darwin 2>/dev/null || echo "⚠️  Skip (toolchain not installed)"
if [ -f target/aarch64-apple-darwin/release/budget ]; then
    cp target/aarch64-apple-darwin/release/budget ../releases/budget-macos-arm64
fi

# macOS x64
echo "📦 Building macOS x64..."
cargo build --release --target x86_64-apple-darwin 2>/dev/null || echo "⚠️  Skip (toolchain not installed)"
if [ -f target/x86_64-apple-darwin/release/budget ]; then
    cp target/x86_64-apple-darwin/release/budget ../releases/budget-macos-x64
fi

# Linux x64
echo "📦 Building Linux x64..."
cargo build --release --target x86_64-unknown-linux-gnu 2>/dev/null || cargo build --release
if [ -f target/x86_64-unknown-linux-gnu/release/budget ]; then
    cp target/x86_64-unknown-linux-gnu/release/budget ../releases/budget-linux-x64
elif [ -f target/release/budget ]; then
    cp target/release/budget ../releases/budget-linux-x64
fi

# Windows x64
echo "📦 Building Windows x64..."
cargo build --release --target x86_64-pc-windows-gnu 2>/dev/null || echo "⚠️  Skip (toolchain not installed)"
if [ -f target/x86_64-pc-windows-gnu/release/budget.exe ]; then
    cp target/x86_64-pc-windows-gnu/release/budget.exe ../releases/budget-windows-x64.exe
fi

echo ""
echo "✅ Build complete!"
echo "📦 Binaries in: releases/"
ls -lh ../releases/
