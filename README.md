# Clouds

An unofficial, static, client-side **WebGPU** and **WebAssembly** port of **Minecraft: Java Edition 26.3** written in Rust. Clouds runs directly in modern WebGPU-enabled browsers with no backend server and no Microsoft account login required.

## Features

- **Zero Server Backend:** Designed for static hosting (such as GitHub Pages) with `coi-serviceworker.js` providing Cross-Origin Isolation when custom HTTP headers are unavailable.
- **WebGPU & WebAssembly:** Native Rust engine (`crates/clouds-wasm`) compiled to `wasm32-unknown-unknown` with WGSL shaders and `fflate.js` for fast client-side zip handling.
- **Local Asset Privacy:** No copyrighted game assets are hosted or distributed. You provide your own `resources.zip` once, and it is cached locally in your browser via **OPFS** (Origin Private File System) with an **IndexedDB** fallback.
- **Persistent World Saves:** Worlds are saved locally in OPFS/IndexedDB and can be exported or imported as `.zip` archives.
- **Non-Intrusive HUD & Pointer Capture:** Starts with a free cursor for UI navigation, captures the pointer on left-click in the 3D viewport, and releases on `Esc`.

## Browser Requirements

- A **WebGPU-enabled browser** (Chrome, Edge, or Firefox Nightly with WebGPU enabled)
- **Cross-Origin Isolation (COI)** for `SharedArrayBuffer` / multithreading (handled automatically via `serve.py` locally or `coi-serviceworker.js` on static hosts)
- **OPFS** or **IndexedDB** support for asset and world storage

## Quick Start (Playing on the Web)

1. Open `index.html` from your GitHub Pages deployment or local COOP/COEP server.
2. Confirm the status chips in the top bar:
   - `COI Active` (Cross-Origin Isolation enabled)
   - `WebGPU Ready` (WebGPU adapter detected)
   - `OPFS Ready` or `IndexedDB Ready` (local storage initialized)
3. Upload or drag-and-drop your `resources.zip` when prompted on first launch.
4. Set your username, render distance, and world name, then click **Play Clouds**.
5. **Controls:** Left-click the 3D viewport to capture the mouse; press `Esc` at any time to release the cursor and access **Options**, **Logs**, or **Fullscreen**.

## Local Development & Testing

Browsers require `Cross-Origin-Opener-Policy: same-origin` and `Cross-Origin-Embedder-Policy: require-corp` headers for `SharedArrayBuffer`. A lightweight Python server (`serve.py`) is included in the repository root:

```bash
python3 serve.py 8080
```

Then open `http://127.0.0.1:8080/` in a WebGPU-enabled browser.

## Building from Source

### Prerequisites

- **Rust 1.84+** (nightly or standard with `RUSTC_BOOTSTRAP=1`) with the `wasm32-unknown-unknown` target
- **`wasm-bindgen-cli`**:
  ```bash
  cargo install -f wasm-bindgen-cli
  ```

### Build Steps

```bash
RUSTC_BOOTSTRAP=1 cargo build --release -Z build-std=std,panic_abort --target wasm32-unknown-unknown -p clouds-wasm
wasm-bindgen --target web --out-dir pkg target/wasm32-unknown-unknown/release/clouds_wasm.wasm
cp pkg/clouds_wasm.js clouds.js
cp pkg/clouds_wasm_bg.wasm clouds_bg.wasm
```

## Deploying to GitHub Pages

1. Push your changes (including `index.html`, `clouds.js`, `clouds_bg.wasm`, `coi-serviceworker.js`, and `fflate.js`) to the `main` branch.
2. In your GitHub repository, go to **Settings → Pages**.
3. Under **Build and deployment**, set the source to **Deploy from a branch**, select `main` and `/ (root)`, and save.

## Project Structure

| Path | Description |
| :--- | :--- |
| `index.html` | Web client shell, HUD, OPFS/IndexedDB VFS, and pointer-lock controls |
| `clouds.js`, `clouds_bg.wasm` | Generated `wasm-bindgen` bindings and WebAssembly binary |
| `coi-serviceworker.js` | Service worker enabling Cross-Origin Isolation on static hosts |
| `fflate.js` | Client-side zip/unzip library for `resources.zip` and world exports |
| `serve.py` | Local development HTTP server with COOP/COEP headers |
| `crates/` | Rust workspace crates (including `clouds-wasm`) |
| `mods/horizon/src/` | Mod and engine integration sources |
| `launcher/`, `packaging/`, `nix/` | Launcher, packaging, and Nix flake (`flake.nix`) configurations |
| `benchmarks/`, `tests/` | Performance benchmarks and test suites |
| `docs/`, `roadmap/` | Technical documentation and `ROADMAP.md` |

## License

Licensed under the **MIT License**. See `LICENSE` and `THIRD_PARTY_NOTICES.md` for details.

> **Disclaimer:** Clouds is an unofficial port and is not affiliated with, endorsed by, or associated with Mojang Studios or Microsoft.
