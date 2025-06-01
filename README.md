# 🧠 Distributed Neural Network with P2P Discovery

A revolutionary distributed neural network implementation that combines **biological plausibility**, **privacy-first design**, **blockchain permanence**, and **true peer-to-peer networking**.

## 🎮 **Live Demo - Try It Now!**

### **🌐 [Launch Live Demo](https://kritivasas.github.io/distributedNN)**

**No installation required!** The demo runs entirely in your browser using WebAssembly.

**Features in the live demo:**
- ✅ **Real neural network** with threshold-gating nodes
- ✅ **Blockchain operations** (mining, validation, transactions)
- ✅ **Vector database** semantic memory search
- ✅ **Real-time visualization** of network activity
- ✅ **Interactive controls** for inputs, errors, and parameters
- ✅ **Guided auto-demo** for newcomers

*Note: The full P2P networking requires local setup with the signaling server. See the [P2P Deployment Guide](P2P_DEPLOYMENT_GUIDE.md) for complete setup.*

## 🌟 **New Feature: Real P2P Discovery Beyond Local Network!**

Your distributed neural network now supports **real peer discovery** across the internet using:
- ✅ **Signaling Server** for WebRTC coordination
- ✅ **Direct peer-to-peer connections** 
- ✅ **NAT traversal** support
- ✅ **Blockchain-based peer registry**
- ✅ **Community network** participation

---

## 🚀 **Quick Start**

### **Option 1: One-Command Startup (Recommended)**
```bash
./start.sh
```

This will:
1. Build the WASM module
2. Start the signaling server (port 8080)
3. Launch the demo interface (port 8000)
4. Show you all the URLs to access

### **Option 2: Manual Setup**
```bash
# 1. Build WASM
wasm-pack build --target web

# 2. Start signaling server
npm install
node signaling-server.js

# 3. In another terminal, start demo
python3 -m http.server 8000
```

Then open `http://localhost:8000/demo.html`

---

## 🌐 **P2P Discovery Options**

### **1. Signaling Server (Browser-Compatible)**
- **Best for:** Browser-based deployments, development, testing
- **Setup:** Use the included `signaling-server.js`
- **Pros:** Easy setup, works behind NAT/firewalls
- **Cons:** Requires central server for discovery (connections are still P2P)

### **2. Blockchain-Based Discovery**
- **Best for:** Fully decentralized networks
- **Setup:** Uses existing blockchain as peer registry
- **Pros:** No central server needed
- **Cons:** Slower discovery, blockchain dependency

### **3. Community Networks**
- **Best for:** Joining existing neural networks
- **Setup:** Connect to community signaling servers
- **Pros:** Instant access to global network
- **Cons:** Depends on community infrastructure

---

## 🏗️ **Architecture Overview**

```
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│   Device A      │    │ Signaling Server │    │   Device B      │
│                 │◄──►│                 │◄──►│                 │
│ ┌─────────────┐ │    │  - Discovery    │    │ ┌─────────────┐ │
│ │Threshold    │ │    │  - WebRTC       │    │ │Threshold    │ │
│ │Gating Nodes │ │    │  - Coordination │    │ │Gating Nodes │ │
│ └─────────────┘ │    └─────────────────┘    │ └─────────────┘ │
│ ┌─────────────┐ │                           │ ┌─────────────┐ │
│ │Local Memory │ │    ┌─────────────────┐    │ │Local Memory │ │
│ │& Blockchain │ │    │   Blockchain    │    │ │& Blockchain │ │
│ └─────────────┘ │◄──►│   Ledger        │◄──►│ └─────────────┘ │
│ ┌─────────────┐ │    │ - Auditability  │    │ ┌─────────────┐ │
│ │Vector       │ │    │ - Incentives    │    │ │Vector       │ │
│ │Database     │ │    │ - Permanence    │    │ │Database     │ │
│ └─────────────┘ │    └─────────────────┘    │ └─────────────┘ │
└─────────────────┘                           └─────────────────┘
         │                                             │
         └─────────────── Direct P2P ─────────────────┘
                      WebRTC Connection
```

## 🧠 **Core Features**

### **Biologically Inspired Neural Architecture**
- **Threshold-gating nodes** with adaptive firing
- **Forward-only learning** (no backpropagation)
- **Eligibility traces** for temporal credit assignment
- **Three-level memory hierarchy** (node → cluster → global)
- **Temporal plasticity** with adaptive timers

### **Privacy-First Design**
- **Raw data never leaves device**
- **Semantic masking** of sensitive information
- **Encrypted memory capsules** for sharing
- **User-controlled privacy** levels

### **Blockchain Integration**
- **Immutable audit trail** of all learning
- **Smart contract incentives** for participation
- **Decentralized memory storage** with IPFS integration
- **Transparent credit system** for contributions

### **P2P Networking**
- **Real-time node borrowing** between devices
- **Direct memory sharing** with encryption
- **Collaborative learning** sessions
- **Error propagation** across network
- **Dynamic topology** with fault tolerance

---

## 📋 **Demo Interface Features**

### **🌐 Signaling Server Configuration**
- Configure signaling server URL
- Real-time connection status
- Automatic reconnection

### **🔍 P2P Discovery & Connection**
- Start peer discovery
- View discovered peers with capabilities
- Connect via WebRTC
- Send heartbeats

### **🤝 Direct P2P Operations**
- Borrow nodes from peers
- Share memory capsules
- Start collaborative learning
- Propagate error signals

### **🧠 Learning & Memory**
- Process input through neural clusters
- Update error signals
- Search vector database
- Consolidate long-term memory

### **⛓️ Blockchain & Vector Database**
- Mine blocks
- Validate blockchain
- Semantic memory search
- View memory trends

### **📊 Real-time Monitoring**
- Connected peers count
- Network health percentage
- Average latency
- Account balance
- Memory capsule count

---

## 🔧 **Development**

### **Project Structure**
```
├── src/
│   ├── lib.rs              # Main API
│   ├── threshold_node.rs   # Threshold-gating nodes
│   ├── memory.rs           # Memory management
│   ├── cluster.rs          # Device clusters
│   ├── blockchain.rs       # Blockchain ledger
│   ├── vector_db.rs        # Vector database
│   ├── p2p_network.rs      # P2P networking
│   └── utils.rs            # Utilities
├── signaling-server.js     # Node.js signaling server
├── demo.html              # Web interface
├── start.sh               # Startup script
└── P2P_DEPLOYMENT_GUIDE.md # Detailed deployment guide
```

### **Building and Testing**
```bash
# Build WASM
cargo build
wasm-pack build --target web

# Run tests
cargo test

# Start development environment
./start.sh
```

### **Signaling Server API**
```javascript
// Message types
{
  "register": { device_id, peer_info },
  "discover": { filters },
  "signal": { target_device_id, signaling_data },
  "heartbeat": { device_status, resources },
  "node_request": { target_device_id, request },
  "memory_share": { target_device_id, capsule }
}
```

---

## 🌍 **Deployment Options**

### **Local Development**
```bash
./start.sh
# Opens on localhost:8000
```

### **Production Deployment**

#### **Heroku**
```bash
heroku create your-neural-signaling-server
git push heroku main
```

#### **VPS/Cloud**
```bash
# Install dependencies
sudo apt update && sudo apt install nodejs npm
git clone your-repo && cd your-repo
npm install

# Production start
npm install -g pm2
pm2 start signaling-server.js --name "neural-signaling"
pm2 startup && pm2 save
```

#### **Docker**
```dockerfile
FROM node:18
WORKDIR /app
COPY package*.json ./
RUN npm install
COPY . .
EXPOSE 8080
CMD ["node", "signaling-server.js"]
```

---

## 🔐 **Security Features**

### **Network Security**
- **WebRTC DTLS encryption** for all P2P connections
- **Device authentication** with cryptographic signatures
- **Message integrity** verification

### **Privacy Protection**
- **Automatic data masking** (names → `[PERSON]`)
- **Local computation** only
- **Encrypted memory capsules**
- **User-controlled sharing** permissions

### **Blockchain Security**
- **Immutable audit logs**
- **Smart contract validation**
- **Decentralized verification**
- **Reputation-based trust**

---

## 📊 **Performance & Scalability**

### **Browser Performance**
- **WASM-optimized** neural computation
- **Efficient memory management**
- **Progressive loading** of large networks
- **Real-time statistics** dashboard

### **Network Scalability**
- **Dynamic peer discovery**
- **Adaptive routing** through intermediaries
- **Load balancing** across available nodes
- **Graceful degradation** on failures

### **Memory Efficiency**
- **Compressed memory capsules**
- **Hierarchical storage** (hot/warm/cold)
- **Automatic cleanup** of stale data
- **Vector database optimization**

---

## 🤝 **Contributing**

### **Research Areas**
- **Neuroscience-inspired** learning algorithms
- **Distributed systems** optimization
- **Privacy-preserving** machine learning
- **Blockchain consensus** mechanisms

### **Development Areas**
- **WebRTC optimization** for neural data
- **Mobile device** support
- **GPU acceleration** with WebGL
- **Advanced UI/UX** for neural networks

### **Community Building**
- **Deployment guides** for different platforms
- **Tutorial content** for distributed AI
- **Integration examples** with other systems
- **Performance benchmarks**

---

## 📚 **Resources**

- **[🎮 Live Demo](https://kritivasas.github.io/distributedNN)** - Try it in your browser!
- **[P2P Deployment Guide](P2P_DEPLOYMENT_GUIDE.md)** - Detailed setup instructions
- **[Research Paper](researchPaper.pdf)** - Theoretical foundation
- **[Live Demo](demo.html)** - Interactive interface
- **[API Documentation](#)** - Complete API reference

---

## 🎯 **Use Cases**

### **Research & Education**
- **Distributed AI** experiments
- **Neuroscience modeling**
- **Privacy-preserving** learning
- **Blockchain applications**

### **Production Applications**
- **Edge computing** networks
- **Collaborative filtering**
- **Federated learning**
- **IoT neural networks**

### **Community Networks**
- **Global brain** simulations
- **Collective intelligence**
- **Decentralized AI** services
- **Open research** platforms

---

## 🏆 **What Makes This Special**

1. **First browser-based** distributed neural network with real P2P discovery
2. **Biologically plausible** forward-only learning
3. **Complete privacy** - raw data never leaves your device
4. **Blockchain permanence** - all learning is auditable forever
5. **True decentralization** - no single point of failure
6. **Community incentives** - get rewarded for contributing
7. **Production ready** - real deployment options included

**Your brain-inspired AI network is ready to span the globe! 🌍🧠**

---

*Built with ❤️ for the future of decentralized AI* 