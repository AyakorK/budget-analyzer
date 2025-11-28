#!/bin/bash

set -e

echo "🧪 Budget Analyzer - Semantic Detection Tests"
echo "=============================================="
echo ""

# Build
echo "📦 Building..."
cargo build --quiet
echo "✅ Build complete"
echo ""

# Run tests
echo "🔬 Running unit tests..."
cargo test --lib --quiet
echo "✅ Unit tests passed"
echo ""

echo "🧩 Running integration tests..."
cargo test --test integration_test --quiet
echo "✅ Integration tests passed"
echo ""

# Analyze fixtures
echo "📊 Analyzing fixtures by language..."
echo ""

for lang in typescript python ruby javascript; do
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    echo "  Language: $lang"
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

    for file in tests/fixtures/$lang/*.*; do
        if [ -f "$file" ]; then
            cargo run --quiet -- analyze "$file" 2>/dev/null || true
        fi
    done
    echo ""
done

echo "=============================================="
echo "✅ All tests completed successfully!"
echo ""

echo "📋 Configuration:"
cargo run --quiet -- config
