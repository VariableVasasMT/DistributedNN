#!/bin/bash

echo "🚀 Building Distributed Neural Network with True P2P WebRTC..."

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;36m'
NC='\033[0m' # No Color

# Function to log with colors
log_info() {
    echo -e "${BLUE}ℹ️  $1${NC}"
}

log_success() {
    echo -e "${GREEN}✅ $1${NC}"
}

log_warning() {
    echo -e "${YELLOW}⚠️  $1${NC}"
}

log_error() {
    echo -e "${RED}❌ $1${NC}"
}

# Step 1: Clean previous builds
log_info "Cleaning previous builds..."
rm -rf pkg/
rm -rf docs/pkg/

# Step 2: Build WASM module with correct package name
log_info "Building WASM module with WebRTC support..."
if wasm-pack build --target web --out-dir pkg; then
    log_success "WASM module built successfully in pkg/"
else
    log_error "WASM build failed"
    exit 1
fi

# Step 3: Copy WASM module to docs for GitHub Pages
log_info "Copying WASM module to docs for GitHub Pages..."
if cp -r pkg/ docs/pkg/; then
    log_success "WASM module copied to docs/pkg/"
else
    log_error "Failed to copy WASM module to docs"
    exit 1
fi

# Step 4: Update docs from Heroku signaling serverhtml (ensure docs/index.html is the main page)
log_info "Updating GitHub Pages documentation..."
if cp demo.html docs/index.html; then
    log_success "Documentation updated from demo.html"
else
    log_error "Failed to update documentation"
    exit 1
fi

# Step 5: Start signaling server in background for local testing
log_info "Starting signaling server for local testing..."
if command -v node &> /dev/null; then
    if [ -f "signaling-server.js" ]; then
        log_info "Signaling server available at: ws://localhost:8080"
        log_warning "Run 'node signaling-server.js' in another terminal for P2P functionality"
    else
        log_warning "signaling-server.js not found"
    fi
else
    log_warning "Node.js not installed - signaling server won't be available"
fi

# Step 6: Update README with correct GitHub Pages URL
log_info "Updating README.md with latest features and correct URLs..."
cat > README.md << 'EOF'
# 🧠⛓️ Distributed Neural Network with True P2P WebRTC

> **Real peer-to-peer neural network collaboration with WebRTC data channels and blockchain incentives**

## 🌟 **New: True Peer-to-Peer Architecture**

Direct device-to-device communication with **NO data relay** through servers!

### **🚀 Quick Start - Join the Global Network**

1. **Visit the live demo:** https://variablevasasmt.github.io/distributedNN
2. **Click "🚀 Create Cluster"** to initialize your neural network  
3. **Connect to Signaling Server** (only for peer discovery)
4. **Start Peer Discovery** to find other devices
5. **Initiate WebRTC Connection** for direct P2P communication
6. **Share neural data directly** between devices!

### **🌐 True P2P Architecture**
```
Device A ←──WebRTC Data Channel──→ Device B
    ↑                                 ↑
    └──Signaling Server (handshake)───┘
       (Discovery ONLY - No data relay)
```

### **🔧 What Makes This True P2P:**
- ✅ **Signaling Server**: Used ONLY for discovery and WebRTC handshake
- ✅ **Data Transfer**: Direct peer-to-peer via WebRTC data channels  
- ✅ **No Relay**: After handshake, ALL data flows directly between devices
- ✅ **Security**: WebRTC provides built-in DTLS encryption
- ✅ **Scalable**: No bandwidth limitations from central servers

## 🎯 **Core Features**

### **🤝 WebRTC P2P Neural Networking**
- **Real WebRTC connections** using browser APIs
- **Direct memory sharing** via data channels
- **Node borrowing** over P2P links  
- **Collaborative learning** sessions
- **Error propagation** across direct connections
- **End-to-end encryption** via WebRTC DTLS

### **⛓️ Blockchain Integration**
- **Node borrowing marketplace** with smart contracts
- **Memory capsule verification** on blockchain
- **Automatic incentive distribution**
- **Reputation-based peer ranking**
- **Tamper-proof training records**

### **🧠 Advanced Architecture**
- **Forward-only threshold gating** for efficiency
- **Semantic memory search** with vector database
- **Long-term memory consolidation**
- **Dynamic cluster formation**
- **Multi-device synchronization**

## 🛠️ **Development Setup**

### **Prerequisites**
```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# Install wasm-pack
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

# Install Node.js (for signaling server)
# Visit: https://nodejs.org/
```

### **Build & Run Locally**
```bash
# Clone the repository
git clone https://github.com/variablevasasmt/distributedNN
cd distributedNN

# Build everything (WASM + docs)
./build.sh

# Start signaling server (separate terminal)
node signaling-server.js

# Start local development server
python3 -m http.server 8000

# Visit: http://localhost:8000/demo.html

# When ready, push to deploy both services:
git push origin main
```

### **GitHub Pages Deployment**
The project automatically deploys to GitHub Pages on every push to `main`:
- **Live Demo:** https://variablevasasmt.github.io/distributedNN
- **Auto-build:** GitHub Actions compiles WASM and deploys
- **Source:** `docs/` directory serves the web content

### **Heroku Signaling Server**
The signaling server automatically deploys to Heroku from the same repository:
- **Server URL:** https://neural-signaling-server.herokuapp.com
- **Auto-deploy:** Heroku deploys from root directory on push to `main`
- **Source:** `signaling-server.js`, `package.json`, `Procfile` in root

### **Single Branch Deployment**
```bash
# One push deploys everything:
git push origin main

# ✅ GitHub Pages: Deploys docs/ to https://variablevasasmt.github.io/distributedNN
# ✅ Heroku: Deploys signaling server to https://neural-signaling-server.herokuapp.com
```

## 🌍 **Network Architecture Details**

### **Signaling Server Role:**
- 📡 **Peer discovery** and registration
- 🤝 **WebRTC handshake** (offer/answer/ICE candidates)
- 🚫 **NO data relay** - all neural data goes direct P2P

### **WebRTC Data Channels:**
- 📊 **Neural activations** shared directly
- 💾 **Memory capsules** transferred P2P
- 🔄 **Error gradients** propagated direct
- 🤝 **Node borrowing** requests via P2P

### **How It Works:**
1. **Discovery:** Peers register with signaling server
2. **Handshake:** WebRTC offer/answer exchange via signaling
3. **Connection:** Direct P2P data channel established
4. **Communication:** All neural data flows directly between devices
5. **Blockchain:** Transactions recorded for incentives

## 📊 **Network Stats & Monitoring**

The demo provides real-time visualization of:
- **WebRTC connection status** and data channel health
- **Direct P2P message** transmission
- **Peer discovery** and connection establishment
- **Blockchain transactions** and incentive distribution
- **Neural network collaboration** metrics

## 🔧 **API Reference**

### **P2P WebRTC Networking**
```javascript
// Initialize with WebRTC support
const network = new DistributedNeuralNetwork("my_device_id");

// Configure signaling server (discovery only)
network.configure_signaling_server("ws://localhost:8080");

// Discover peers
network.start_peer_discovery();

// Establish direct WebRTC connection
await network.initiate_webrtc_connection("target_peer_id");

// Send data directly via WebRTC data channel
network.send_direct_message("Hello P2P!");

// Close direct connection
network.close_webrtc_connection("peer_id");
```

### **Blockchain Integration**
```javascript
// Request computational resources (sent via P2P)
network.request_node_borrowing(owner_id, node_id, duration_hours);

// Mine blocks and earn rewards
network.mine_block();

// Validate the blockchain
network.validate_blockchain();
```

### **Memory & Learning**
```javascript
// Process distributed input
const outputs = network.process_input(cluster_id, input_data);

// Share memory directly via P2P
network.share_memory_with_peer(peer_id, cluster_id);

// Start collaborative learning session
network.start_collaborative_learning([peer1, peer2], "task_description");
```

## 🔒 **Security & Privacy**

- **WebRTC DTLS encryption** secures all P2P communication
- **No data passes through signaling server** after handshake
- **Blockchain verification** ensures data integrity
- **Direct device connections** minimize attack surface
- **Decentralized architecture** prevents single points of failure

## 🚀 **Performance Benefits**

- **No bandwidth bottlenecks** from central servers
- **Lower latency** with direct device communication  
- **Scalable** - network grows stronger with more peers
- **Efficient** - no unnecessary data relay hops
- **Resilient** - works even if signaling server goes down (existing connections persist)

EOF

log_success "README.md updated with True P2P WebRTC documentation"

# Step 7: Verify deployment configuration
log_info "Verifying deployment configuration..."

# Check if necessary files exist for Heroku
if [ ! -f "package.json" ]; then
    log_error "package.json not found - needed for Heroku deployment"
    exit 1
fi

if [ ! -f "Procfile" ]; then
    log_error "Procfile not found - needed for Heroku deployment"
    exit 1
fi

if [ ! -f "signaling-server.js" ]; then
    log_error "signaling-server.js not found - needed for Heroku deployment"
    exit 1
fi

# Check if docs directory exists for GitHub Pages
if [ ! -d "docs" ]; then
    log_error "docs/ directory not found - needed for GitHub Pages"
    exit 1
fi

if [ ! -f "docs/index.html" ]; then
    log_error "docs/index.html not found - needed for GitHub Pages"
    exit 1
fi

if [ ! -d "docs/pkg" ]; then
    log_error "docs/pkg/ directory not found - needed for GitHub Pages WASM"
    exit 1
fi

log_success "✅ Repository configured for dual deployment:"
log_success "   📄 GitHub Pages: docs/ folder"
log_success "   📡 Heroku: root folder (signaling server)"

# Step 8: Ensure docs directory structure is correct for GitHub Pages
log_info "Ensuring docs structure is correct for GitHub Pages..."
if [ ! -d "docs/pkg" ]; then
    mkdir -p docs/pkg
fi

# Check if we need to commit changes
if git diff --quiet && git diff --cached --quiet; then
    log_info "No changes to commit"
else
    log_info "Committing changes..."
    
    # Add all changes
    git add .
    
    # Create commit message with timestamp
    TIMESTAMP=$(date '+%Y-%m-%d %H:%M:%S')
    COMMIT_MSG="🚀 Auto-build: WASM + docs update - $TIMESTAMP"
    
    if git commit -m "$COMMIT_MSG"; then
        log_success "Changes committed: $COMMIT_MSG"
        
        # Ask if user wants to push
        echo ""
        read -p "🌐 Push changes to GitHub? This will deploy both GitHub Pages AND Heroku signaling server (y/N): " -n 1 -r
        echo ""
        
        if [[ $REPLY =~ ^[Yy]$ ]]; then
            log_info "Pushing to GitHub..."
            if git push origin main; then
                log_success "🌐 Changes pushed to GitHub!"
                log_success "📄 GitHub Actions will build and deploy GitHub Pages automatically"
                log_success "🔗 Live demo: https://variablevasasmt.github.io/distributedNN"
                log_info "⏱️  Deployment usually takes 2-3 minutes to complete"
                
            else
                log_error "Failed to push to GitHub"
            fi
            
            if git push heroku main; then
                log_success "🌐 Changes pushed to Heroku!"
                heroku logs -a neural-signaling-server
                heroku ps -a neural-signaling-server
            else
                log_error "Failed to push to Heroku"
            fi
        else
            log_info "Skipped GitHub push (you can push manually later)"
        fi
    else
        log_error "Failed to commit changes"
    fi
fi

# Step 9: Start local server if requested
echo ""
read -p "🖥️  Start local development server? (y/N): " -n 1 -r
echo ""

if [[ $REPLY =~ ^[Yy]$ ]]; then
    log_info "Starting local server on http://localhost:8000"
    log_success "🎉 Build complete! Opening demo..."
    
    # Try to open browser (works on macOS and Linux)
    if command -v open >/dev/null 2>&1; then
        open http://localhost:8000/demo.html
    elif command -v xdg-open >/dev/null 2>&1; then
        xdg-open http://localhost:8000/demo.html
    fi
    
    python3 -m http.server 8000
else
    echo ""
    log_success "🎉 Build complete!"
    log_info "📁 Files updated:"
    log_info "   • docs/pkg/ - WASM module for GitHub Pages"
    log_info "   • docs/index.html - GitHub Pages entry point"
    log_info "   • README.md - Documentation with correct URLs"
    echo ""
    log_info "🖥️  To test locally: python3 -m http.server 8000"
    log_info "🌐 GitHub Pages: https://variablevasasmt.github.io/distributedNN"
    log_info "🔄 GitHub Actions: Automatic deployment on push to main"
fi 