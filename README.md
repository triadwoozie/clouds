# Clouds — Minecraft: Java Edition 26.3 in WebGPU & WebAssembly

**Clouds** is a static, client-side WebGPU port of Minecraft: Java Edition 26.3 rewritten in Rust and compiled to WebAssembly.

- 🌐 **Zero Server Backend**: Hosted statically (e.g., directly on GitHub Pages).
- ⚡ **WebGPU & WebAssembly**: Pure client-side native rendering and gameplay in modern browsers (Chrome, Edge, Firefox Nightly).
- 🔒 **Asset Privacy & Anti-Piracy**: No copyrighted game assets are hosted on the website. Textures, sounds, and models stay 100% on the player's device using high-performance browser storage (OPFS / IndexedDB). Drop `resources.zip` once, and Clouds caches it locally forever.
- 🔓 **No Microsoft Account Required**: Patched out the launcher and Microsoft authentication requirement for direct local play.
- 💾 **Persistent World Saves**: World saves are stored locally in OPFS/IndexedDB and can be exported as `.zip` or imported anytime.

---

## Quick Start (Playing on the Web)

1. Open `index.html` via GitHub Pages or a local COOP/COEP HTTP server.
2. When prompted on first launch, upload or drag-and-drop your `resources.zip`.
3. Choose your username, render distance, and world name, then click **Play Clouds**.
4. **Mouse Control**:
   - The game starts with a free OS cursor so you can click Options, Worlds, Logs, or Fullscreen anytime.
   - **Left-click** inside the 3D viewport canvas to capture mouse control.
   - Press <kbd>Esc</kbd> anytime to release the cursor.

---

## Local Development & Testing

WebGPU and multithreaded WASM require Cross-Origin Isolation (`Cross-Origin-Opener-Policy: same-origin` and `Cross-Origin-Embedder-Policy: require-corp`). A Python test server is included:

```bash
# Start local development server with COOP/COEP headers
python3 serve.py 8080
```

Then visit `http://127.0.0.1:8080/` in a WebGPU-enabled browser.

---

## Building from Source

### Prerequisites
- Rust 1.84+ (nightly or standard with `wasm32-unknown-unknown` target)
- `wasm-bindgen-cli` (`cargo install -f wasm-bindgen-cli`)

### Build Steps

```bash
# 1. Compile clouds-wasm to WebAssembly
RUSTC_BOOTSTRAP=1 cargo build --release \
  -Z build-std=std,panic_abort \
  --target wasm32-unknown-unknown \
  -p clouds-wasm

# 2. Run wasm-bindgen
wasm-bindgen --target web \
  --out-dir pkg \
  target/wasm32-unknown-unknown/release/clouds_wasm.wasm

# 3. Deploy wasm and JS bindings to root
cp pkg/clouds_wasm.js clouds.js
cp pkg/clouds_wasm_bg.wasm clouds_bg.wasm
```

---

## GitHub Pages Deployment

1. Fork or push this repository to GitHub.
2. In your repository on GitHub, navigate to **Settings** → **Pages**.
3. Under **Build and deployment** → **Source**, select **Deploy from a branch**.
4. Choose Branch: `main` and Folder: `/ (root)`, then click **Save**.
5. Your client will be live at `https://<username>.github.io/<repo>/`!

---

## License

MIT License. See [LICENSE](LICENSE) for details.
This project is an unofficial port and is not affiliated with or endorsed by Mojang or Microsoft.
