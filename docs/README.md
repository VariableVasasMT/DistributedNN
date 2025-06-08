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

