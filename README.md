# Shorlark 
[![codecov](https://codecov.io/gh/niclaslind/shorelark/branch/main/graph/badge.svg?token=DctFt73rMr)](https://codecov.io/gh/niclaslind/shorelark)
[![dependency status](https://deps.rs/repo/github/niclaslind/shorelark/status.svg)](https://deps.rs/repo/github/niclaslind/shorelark)


Implementation of NLP with RUST + WASM by following https://pwy.io/en/posts/learning-to-fly-pt1/ tutorial. 

## ✅ Prerequisites

Make sure you have the following installed:

- [Rust](https://www.rust-lang.org/tools/install)
- [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/)
- [Node.js](https://nodejs.org/) (v14 or later)

Install `wasm-pack` if you haven't already:

```bash
cargo install wasm-pack
```

## 🛠️ Build & Run

### 1. Build the WASM package

Navigate to the Rust crate and compile it using `wasm-pack`:

```bash
cd libs/simulation-wasm
wasm-pack build 
```

### 2. Install frontend dependencies

Go to the www folder and install JavaScript dependencies:

```bash
cd www
npm install
```

### 3. Start the development server

#### Using npm

```bash
npm run start
```

#### Using npx

```bash
npx webpack serve
```

## 🔄 Rebuilding WASM After Changes

If you make changes to the Rust code, rebuild the WebAssembly package:

```bash
cd libs/simulation-wasm
wasm-pack build
```