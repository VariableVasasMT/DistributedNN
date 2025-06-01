#!/bin/bash

echo "🧠 Distributed Neural Network P2P Startup Script"
echo "================================================="

# Check if Node.js is installed
if ! command -v node &> /dev/null; then
    echo "❌ Node.js is not installed. Please install Node.js first."
    exit 1
fi

# Check if npm is installed
if ! command -v npm &> /dev/null; then
    echo "❌ npm is not installed. Please install npm first."
    exit 1
fi

# Check if wasm-pack is installed
if ! command -v wasm-pack &> /dev/null; then
    echo "❌ wasm-pack is not installed. Installing now..."
    curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
fi

echo "🔧 Building WASM module..."
wasm-pack build --target web

if [ $? -ne 0 ]; then
    echo "❌ WASM build failed. Please check your Rust installation."
    exit 1
fi

echo "✅ WASM build successful!"

# Install npm dependencies if they don't exist
if [ ! -d "node_modules" ]; then
    echo "📦 Installing Node.js dependencies..."
    npm install
fi

echo "🌐 Starting signaling server..."
echo "📊 Server stats will be available at: http://localhost:8080/stats"

# Start the signaling server in the background
node signaling-server.js &
SIGNALING_PID=$!

# Wait a moment for the server to start
sleep 2

echo "🚀 Starting demo server..."
echo "🌍 Demo will be available at: http://localhost:8000/demo.html"

# Start a simple HTTP server for the demo
python3 -m http.server 8000 &
DEMO_PID=$!

# Function to cleanup background processes
cleanup() {
    echo ""
    echo "🛑 Shutting down servers..."
    kill $SIGNALING_PID 2>/dev/null
    kill $DEMO_PID 2>/dev/null
    echo "✅ Cleanup complete"
    exit 0
}

# Set up signal handlers
trap cleanup SIGINT SIGTERM

echo ""
echo "✅ All services started successfully!"
echo "================================================="
echo "📡 Signaling Server: http://localhost:8080"
echo "📊 Server Stats: http://localhost:8080/stats"
echo "🌍 Demo Interface: http://localhost:8000/demo.html"
echo "================================================="
echo ""
echo "💡 Tips:"
echo "  • Open multiple browser tabs to test P2P discovery"
echo "  • Each tab will get a unique device ID"
echo "  • Use the signaling server for peer discovery"
echo "  • Check the browser console for detailed logs"
echo ""
echo "Press Ctrl+C to stop all services"

# Wait for user to stop the script
wait 