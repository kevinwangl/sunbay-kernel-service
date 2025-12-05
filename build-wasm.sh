#!/bin/bash

# Build WASM kernel for sunbay-kernel-service
# This script compiles the Rust code to WebAssembly

set -e

echo "==========================================="
echo "Building WASM Kernel"
echo "==========================================="
echo ""

# Check if wasm32-unknown-unknown target is installed
if ! rustup target list | grep -q "wasm32-unknown-unknown (installed)"; then
    echo "📦 Installing wasm32-unknown-unknown target..."
    rustup target add wasm32-unknown-unknown
fi

# Check if wasm-bindgen-cli is installed
if ! command -v wasm-bindgen &> /dev/null; then
    echo "📦 Installing wasm-bindgen-cli..."
    cargo install wasm-bindgen-cli
fi

# Build the WASM module
echo "🔨 Building WASM module..."
cargo build --lib --target wasm32-unknown-unknown --release --no-default-features

# Get the WASM file
WASM_FILE="target/wasm32-unknown-unknown/release/sunbay_kernel_service.wasm"

if [ ! -f "$WASM_FILE" ]; then
    echo "❌ WASM build failed: $WASM_FILE not found"
    exit 1
fi

echo "✅ WASM build successful"
echo ""

# Run wasm-bindgen to generate JavaScript bindings
echo "🔧 Generating JavaScript bindings..."
mkdir -p pkg
wasm-bindgen "$WASM_FILE" \
    --out-dir pkg \
    --target web \


echo "✅ JavaScript bindings generated in pkg/"
echo ""

# Copy the WASM file to static directory for publishing
echo "📦 Copying WASM to static directory..."
mkdir -p static
cp pkg/sunbay_kernel_service_bg.wasm static/mock_kernel.wasm

# Display file info
WASM_SIZE=$(du -h static/mock_kernel.wasm | cut -f1)
echo "✅ WASM kernel ready: static/mock_kernel.wasm"
echo "📏 File size: $WASM_SIZE"
echo ""

# Optional: Optimize with wasm-opt if available
if command -v wasm-opt &> /dev/null; then
    echo "🚀 Optimizing WASM with wasm-opt..."
    wasm-opt -Oz static/mock_kernel.wasm -o static/mock_kernel.wasm
    OPTIMIZED_SIZE=$(du -h static/mock_kernel.wasm | cut -f1)
    echo "✅ Optimized size: $OPTIMIZED_SIZE"
    echo ""
fi

echo "==========================================="
echo "✅ Build Complete!"
echo "==========================================="
echo "WASM file: static/mock_kernel.wasm"
echo "JS bindings: pkg/"
echo ""
