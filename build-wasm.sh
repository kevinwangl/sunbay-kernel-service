#!/bin/bash

# Build WASM kernel with wasm-pack
# This generates proper JavaScript bindings for console.log and other web APIs

set -e

echo "=========================================="
echo "Building WASM Kernel with wasm-pack"
echo "=========================================="

# Check if wasm-pack is installed
if ! command -v wasm-pack &> /dev/null; then
    echo "❌ wasm-pack not found. Installing..."
    cargo install wasm-pack
fi

# Clean previous builds
echo "🧹 Cleaning previous builds..."
rm -rf pkg/
rm -rf target/wasm32-unknown-unknown/
cargo clean

# Build with wasm-pack (no default features to exclude server dependencies)
echo "🔨 Building WASM with wasm-pack..."
wasm-pack build --target web --out-dir pkg --no-typescript --no-default-features --release

if [ $? -ne 0 ]; then
    echo "❌ Build failed"
    exit 1
fi

echo ""
echo "✅ Build successful!"
echo "📦 Output directory: pkg/"
echo ""
echo "Generated files:"
ls -lh pkg/

echo ""
echo "=========================================="
echo "✅ WASM build complete!"
echo "=========================================="
echo ""
echo "Next steps:"
echo "1. Copy pkg/sunbay_kernel_service_bg.wasm to backend"
echo "2. Copy pkg/sunbay_kernel_service.js to backend"
echo "3. Update backend to serve both files"
echo ""
