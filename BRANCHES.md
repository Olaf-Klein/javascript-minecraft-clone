# 🎮 Minecraft Clone Branch Overview

This repository now defaults to the **Rust rewrite** for native performance. The legacy JavaScript implementation remains available on a separate branch for reference.

## 📦 Branches

### `main` — Rust Edition (Current Default)
Native Rust + `wgpu` renderer delivering high performance and a lightweight binary.

**Highlights:**
- ✅ Native binary (~10 MB)
- ✅ 2–3× faster rendering compared to Electron build
- ✅ Modern graphics pipeline (WebGPU/Vulkan/Metal/DX12)
- ✅ Procedural world with chunk streaming
- ❌ Multiplayer not yet reimplemented

**Quick Start:**
```bash
git clone https://github.com/Olaf-Klein/javascript-minecraft-clone.git
cd javascript-minecraft-clone
cargo run --release
```

See [README.md](README.md) and [QUICKSTART.md](QUICKSTART.md) for full documentation.

---

### `legacy/javascript` (or similar) — Electron Edition
The original JavaScript/Electron implementation with dedicated server and modding support. Check the branch list if you need this version.

**Highlights:**
- ✅ Multiplayer (LAN + dedicated server)
- ✅ Modding API with hot reload
- ✅ Pterodactyl deployment tooling
- ❌ Large install size (~100 MB+) and higher resource usage

**Quick Start (on the legacy branch):**
```bash
git checkout <legacy-branch-name>
npm run install:all
npm run start:client
```

Refer to that branch’s README for details.

---

## 🤔 Which Version Should I Use?

- **Rust (main)**: choose for best performance, native feel, and slimmer distribution.
- **JavaScript (legacy)**: choose for multiplayer, modding, or if you rely on the Electron toolchain.

## 📊 Quick Comparison

| Feature | Rust (main) | JavaScript (legacy) |
|---------|--------------|----------------------|
| Performance | Excellent | Good |
| Binary Size | ~10 MB | ~100 MB+ |
| Startup Time | < 1 s | 2–3 s |
| Memory Usage | 50–100 MB | 200–300 MB |
| Multiplayer | ❌ Pending | ✅ Built-in |
| Mod Support | 🔲 Planned | ✅ Available |

## 📝 License

MIT License

---

**Happy gaming! 🎮**

Pick the branch that matches your needs and enjoy exploring the blocky world!
