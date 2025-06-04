# 🧠⛓️ Distributed Neural Network with Blockchain & P2P

> **Real-time neural network collaboration across the internet with blockchain incentives**

## 🌟 **New: Global P2P Networking**

Connect your neural network to others worldwide! No local network required.

### **🚀 Quick Start - Join the Global Network**

1. **Visit the live demo:** https://variablevasasmt.github.io/distributedNN
2. **Click "🚀 Create Cluster"** to initialize your neural network
3. **Click "🌐 Connect to Global Network"** - uses our Heroku signaling server
4. **Click "🔍 Find Peers"** to discover other neural networks worldwide
5. **Start collaborative learning!**

### **🌐 Live Signaling Server**
- **URL:** `wss://neural-signaling-server-6900d4b1d1c9.herokuapp.com`
- **Status:** ✅ Live and ready for connections
- **Global reach:** Connect with peers anywhere in the world

## 🎯 **Core Features**

### **🤝 P2P Neural Networking**
- **Global peer discovery** via WebRTC signaling
- **Direct memory sharing** between devices
- **Collaborative learning** sessions
- **Error propagation** across the network
- **Real-time synchronization**

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

# Start local development server
python3 -m http.server 8000

# Visit: http://localhost:8000/demo.html
```

### **GitHub Pages Deployment**
The project automatically deploys to GitHub Pages on every push to `main`:
- **Live Demo:** https://variablevasasmt.github.io/distributedNN
- **Auto-build:** GitHub Actions compiles WASM and deploys
- **Source:** `docs/` directory serves the web content

### **Deploy Your Own Signaling Server**
```bash
# Install dependencies
npm install

# Deploy to Heroku
heroku create your-neural-signaling-server
git push heroku main

# Or run locally
npm start
```

## 🌍 **Global Network Architecture**

```
Your Browser ←→ Signaling Server ←→ Other Peers
     ↓              (Heroku)              ↑
     └─────── Direct WebRTC Connection ──┘
```

### **How It Works:**
1. **Discovery:** Signaling server helps peers find each other
2. **Connection:** Direct WebRTC connections established
3. **Collaboration:** Neural networks share memory and gradients
4. **Incentives:** Blockchain tracks contributions and rewards

## 📊 **Network Stats & Monitoring**

The demo provides real-time visualization of:
- **Connected peers** across the globe
- **Network health** and latency metrics
- **Memory sharing** activity
- **Blockchain transactions** and blocks
- **Learning progress** and error propagation

## 🔧 **Configuration Options**

### **P2P Networking**
```javascript
// Configure your signaling server
network.configure_signaling_server("wss://your-server.herokuapp.com");

// Start peer discovery
network.start_peer_discovery();

// Connect to specific peer
network.connect_to_peer(peer_id, peer_info);
```

### **Blockchain Integration**
```javascript
// Request computational resources
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

// Semantic memory search
const memories = network.semantic_memory_search(query_vector, tags, max_results);

// Start collaborative learning
const session_id = network.start_collaborative_learning([peer1, peer2], task_description);
```

## 🎮 **Interactive Demo Features**

- **🚀 Create Cluster:** Initialize your neural network
- **🌐 Connect to Global Network:** Join the worldwide network
- **🔍 Find Peers:** Discover other neural networks
- **⚡ Process Input:** Run inference across the network
- **🤝 Borrow Nodes:** Rent computational power from peers
- **⛏️ Mine Blocks:** Earn credits by validating transactions
- **📊 Real-time Visualization:** Monitor network activity

## 🌟 **Use Cases**

### **🎓 Distributed Learning**
- Train models across multiple devices
- Share knowledge without sharing raw data
- Collaborative research networks

### **💡 Edge AI Networks**
- IoT device collaboration
- Mobile neural network clusters
- Resource-constrained environments

### **🏢 Enterprise AI**
- Federated learning systems
- Cross-organization collaboration
- Privacy-preserving AI networks

### **🎮 Gaming & Simulation**
- Multi-player AI experiences
- Distributed game intelligence
- Real-time strategy networks

## 🔒 **Security & Privacy**

- **End-to-end encryption** for all P2P communications
- **Blockchain verification** for memory integrity
- **Reputation systems** to prevent malicious behavior
- **Privacy-preserving** gradient sharing
- **Secure WebRTC** connections

## 🤝 **Contributing**

We welcome contributions! Here's how to get involved:

1. **Fork the repository**
2. **Create a feature branch:** `git checkout -b feature/amazing-feature`
3. **Make your changes** and test thoroughly
4. **Run the build script:** `./build.sh`
5. **Submit a pull request**

### **Development Areas**
- 🧠 **Neural Architecture:** Improve threshold gating algorithms
- ⛓️ **Blockchain:** Enhance smart contract functionality
- 🌐 **P2P Networking:** Optimize peer discovery and connection
- 📊 **Visualization:** Create better monitoring dashboards
- 🔒 **Security:** Strengthen encryption and validation

## 📚 **Documentation**

- **[P2P Deployment Guide](P2P_DEPLOYMENT_GUIDE.md)** - Complete P2P setup instructions
- **[Live Demo](https://variablevasasmt.github.io/distributedNN)** - Try it now!
- **[GitHub Repository](https://github.com/variablevasasmt/distributedNN)** - Source code

## 🐛 **Troubleshooting**

### **Common Issues**

**WASM Module Not Loading:**
```bash
# Rebuild the WASM module
./build.sh
```

**P2P Connection Failed:**
```javascript
// Check signaling server status
network.configure_signaling_server("wss://neural-signaling-server-6900d4b1d1c9.herokuapp.com");
```

**Blockchain Validation Errors:**
```javascript
// Validate and repair if needed
const isValid = network.validate_blockchain();
```

## 📄 **License**

MIT License - see [LICENSE](LICENSE) file for details.

## 🚀 **What's Next?**

- **🌍 Mobile app** for neural network participation
- **🎯 Specialized algorithms** for different domains
- **🔗 Integration** with existing ML frameworks
- **📈 Advanced analytics** and insights
- **🤖 Autonomous peer discovery** and optimization

---

**Ready to join the distributed neural network revolution?** 
🌐 **[Try the live demo now!](https://variablevasasmt.github.io/distributedNN)**
