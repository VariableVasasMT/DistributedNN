# 🌐 GitHub Pages Setup Guide

This guide shows how to set up GitHub Pages for the Distributed Neural Network live demo.

## 🚀 Quick Setup

### Step 1: Enable GitHub Pages

1. Go to your repository on GitHub
2. Click **Settings** tab
3. Scroll down to **Pages** in the left sidebar
4. Under **Source**, select **"GitHub Actions"**
5. Save the settings

### Step 2: Push Your Code

The GitHub Actions workflow (`.github/workflows/deploy-pages.yml`) will automatically:
- ✅ Build the WASM module
- ✅ Deploy to GitHub Pages
- ✅ Make your demo live!

## 🎮 Accessing Your Demo

Once deployed, your demo will be available at:
```
https://YOUR_USERNAME.github.io/YOUR_REPOSITORY_NAME
```

For example:
```
https://kritivasas.github.io/distributedNN
```

## 🔧 How It Works

### 1. **GitHub Actions Workflow**
```yaml
# .github/workflows/deploy-pages.yml
- Triggers on push to main branch
- Installs Rust and wasm-pack
- Builds WASM to docs/pkg/
- Deploys docs/ to GitHub Pages
```

### 2. **Demo Files Structure**
```
docs/
├── index.html              # Main demo page
├── pkg/                    # Auto-generated WASM files
│   ├── distributed_neural_wasm.js
│   └── distributed_neural_wasm_bg.wasm
└── README.md
```

### 3. **Features Available Online**
- ✅ **Neural Network** with threshold-gating nodes
- ✅ **Blockchain** operations (mining, validation)
- ✅ **Vector Database** semantic search
- ✅ **Real-time Visualization**
- ✅ **Interactive Controls**

## 🛠️ Customization

### Update the Demo
1. Edit `docs/index.html` for UI changes
2. Edit Rust code for functionality changes
3. Push to main branch
4. GitHub Actions automatically rebuilds

### Custom Domain (Optional)
1. Add a `CNAME` file to `docs/` directory:
   ```
   your-custom-domain.com
   ```
2. Configure DNS at your domain provider
3. Enable HTTPS in GitHub Pages settings

## 🔍 Troubleshooting

### Build Failures
- Check GitHub Actions logs in the **Actions** tab
- Ensure Rust code compiles locally first
- Verify WASM target is installed: `rustup target add wasm32-unknown-unknown`

### Demo Not Loading
- Check browser console for JavaScript errors
- Ensure WASM files are being served correctly
- Verify the import path in `index.html` matches the generated files

### CORS Issues
- GitHub Pages automatically serves with correct CORS headers
- If using custom domain, ensure proper SSL configuration

## 📊 Analytics & Monitoring

### GitHub Pages Analytics
- View traffic in repository **Insights** → **Traffic**
- Monitor visitor statistics and popular pages

### Demo Usage Tracking (Optional)
Add to `docs/index.html`:
```html
<!-- Google Analytics -->
<script async src="https://www.googletagmanager.com/gtag/js?id=GA_TRACKING_ID"></script>
<script>
  window.dataLayer = window.dataLayer || [];
  function gtag(){dataLayer.push(arguments);}
  gtag('js', new Date());
  gtag('config', 'GA_TRACKING_ID');
</script>
```

## 🌍 Sharing Your Demo

### Embed in README
```markdown
## 🎮 Live Demo
**[Try it now!](https://your-username.github.io/your-repo)**
```

### Social Media
```
🧠 Check out my distributed neural network running in the browser!
🌐 Live demo: https://your-username.github.io/your-repo
⚡ Features: Blockchain, P2P networking, biological AI
#AI #Blockchain #WebAssembly #NeuralNetworks
```

### Academic/Research Use
```
Interactive demonstration available at:
https://your-username.github.io/your-repo

Features:
- Biologically-inspired threshold-gating nodes
- Blockchain-based distributed memory
- Real-time visualization of neural activity
- Browser-based WebAssembly implementation
```

## 🎯 Next Steps

1. **Test the demo** thoroughly before sharing
2. **Monitor performance** using browser dev tools
3. **Gather feedback** from users
4. **Iterate and improve** based on usage patterns
5. **Add more features** as needed

## 🔗 Related Resources

- **[Main README](README.md)** - Full project documentation
- **[P2P Deployment Guide](P2P_DEPLOYMENT_GUIDE.md)** - Local setup with networking
- **[Research Paper](researchPaper.pdf)** - Theoretical foundation
- **[GitHub Pages Documentation](https://docs.github.com/en/pages)** - Official guide

---

**Your distributed neural network is now globally accessible! 🌍🧠** 