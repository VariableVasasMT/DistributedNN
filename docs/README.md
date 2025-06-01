# 🌐 Live Demo - GitHub Pages

This directory contains the GitHub Pages deployment files for the Distributed Neural Network live demo.

## 🎮 Access the Demo

**[👉 Launch Live Demo](https://variablevasasmt.github.io/DistributedNN/)**

## 📁 Directory Structure

```
docs/
├── index.html          # Main demo page
├── pkg/                # WASM build output (auto-generated)
│   ├── distributed_neural_wasm.js
│   ├── distributed_neural_wasm_bg.wasm
│   └── package.json
└── README.md           # This file
```

## 🚀 How It Works

1. **GitHub Actions** automatically builds the WASM module on every push
2. **WASM files** are generated in `docs/pkg/` directory
3. **GitHub Pages** serves the `docs/` directory as a static website
4. **Demo runs** entirely in the browser using WebAssembly

## 🔧 Features Available in Live Demo

- ✅ **Neural Network Processing** - Real threshold-gating nodes
- ✅ **Blockchain Operations** - Mining, validation, transactions
- ✅ **Vector Database** - Semantic memory search
- ✅ **Real-time Visualization** - Network activity charts
- ✅ **Interactive Controls** - Inputs, errors, parameters
- ✅ **Auto Demo** - Guided tour for new users

## 📝 Limitations of Live Demo

- ❌ **No P2P Networking** - Requires local signaling server
- ❌ **No File Persistence** - Data resets on page reload
- ❌ **Limited Memory** - Browser memory constraints

## 🔗 Full Local Setup

For complete functionality including P2P networking, see:
- **[Main Repository](../README.md)**
- **[P2P Deployment Guide](../P2P_DEPLOYMENT_GUIDE.md)**
- **[Local Demo](../demo.html)**

## 🛠️ Development

To update the live demo:

1. Make changes to the Rust code or `docs/index.html`
2. Push to the `main` branch
3. GitHub Actions will automatically rebuild and deploy
4. Changes will be live at the GitHub Pages URL

## 📊 Demo Usage Analytics

The live demo helps showcase:
- **Biological neural computation** in the browser
- **Blockchain integration** for distributed AI
- **WebAssembly performance** for ML workloads
- **Real-time visualization** of neural networks

---

*Built with ❤️ for accessible distributed AI* 