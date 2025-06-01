#!/bin/bash

echo "Building Distributed Neural Network WASM module..."

# Source Rust environment
source ~/.cargo/env

# Install wasm-pack if not already installed
if ! command -v wasm-pack &> /dev/null; then
    echo "Installing wasm-pack..."
    curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
fi

# Build the WASM module
echo "Compiling Rust to WASM..."
wasm-pack build --target web --out-dir pkg

if [ $? -eq 0 ]; then
    echo "✅ Build successful!"
    echo "📦 WASM module generated in ./pkg/"
    echo ""
    echo "Generated files:"
    ls -la pkg/
    echo ""
    echo "🚀 You can now use the module in a web browser!"
    echo "📄 See demo.html for an example"
else
    echo "❌ Build failed!"
    exit 1
fi 