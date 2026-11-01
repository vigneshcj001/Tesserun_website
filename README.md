# Tesserun website

The download website for the Tesserun native Windows application. Its interactive interface uses **Rust 2024, Leptos 0.8.23, WebAssembly, and Trunk 0.22.1**. Semantic HTML and responsive CSS preserve the original minimalist design.

Page content, FAQs, and downloads arrive as static HTML. Rust components enhance navigation and the workspace tour after WebAssembly loads. No production Node server, database, or model API connection is required.

## Stack and modules

| Path | Responsibility |
| --- | --- |
| `index.html` | Static content, metadata, download links, and loading fallbacks |
| `styles.css` | Responsive design, typography, and reduced-motion support |
| `src/main.rs` | Mount the Leptos client components |
| `src/navigation.rs` | Reactive mobile navigation and Escape handling |
| `src/tour.rs` | Workspace preview, tabs, and keyboard focus |
| `src/tour/content.rs` | Illustrative development, review, and deployment views |
| `src/tour/step.rs` | Typed tour states and keyboard transitions |
| `public/` | Branding, Windows download, and SHA-256 checksum |
| `dist/` | Release output for static hosting |
| `scripts/` | Local build, preview, and output verification helpers |

Leptos and resolved dependencies are pinned in `Cargo.toml` and `Cargo.lock`. Trunk generates the JavaScript loader required to start WebAssembly; there is no handwritten browser JavaScript application.

## Preview without a Rust installation

Install Node.js, then run:

```powershell
cd D:\APp\Tesserun-website
node scripts/serve.mjs
```

Open http://127.0.0.1:4173. The checked-in `dist/` contains the website and Windows download. Run `node scripts/check.mjs` to verify references, the download checksum, and WebAssembly output.

## Build from Rust source

Install current stable Rust through rustup and Trunk 0.22.1. Node.js is used for convenience scripts and verification.

```powershell
rustup target add wasm32-unknown-unknown
cargo install trunk --version 0.22.1 --locked
node scripts/build.mjs
node scripts/check.mjs
```

The build uses locked dependencies, release optimization, and wasm-opt. Trunk downloads matching wasm-bindgen and wasm-opt tools when needed. First builds require network access and may take several minutes.

On this Windows workspace, the helper reuses the sibling `Tesserun/.tools` Rust/LLVM toolchain and the website's `.tools/bin/trunk.exe`. These tools are ignored by Git. Fresh clones use system tools. On a fresh Windows machine, use rustup's standard MSVC toolchain with Visual Studio C++ Build Tools. The local GNU compatibility helper only applies when the existing sibling toolchain is found.

```powershell
node scripts/build.mjs dev     # Live-reloading development server
node scripts/build.mjs check   # Compile-check WASM
node scripts/build.mjs lint    # Clippy with warnings rejected
node scripts/build.mjs format  # Format Rust
node scripts/build.mjs test    # Native unit tests
```

## Deploy to Vercel

Import **vigneshcj001/Tesserun_website**, choose framework **Other**, leave build and install commands empty, and set output directory to **dist**. `vercel.json` declares these settings. No environment variables or Rust server are needed on Vercel.

The repository includes a release bundle so Vercel can serve it directly. After changing Rust, HTML, or CSS, rebuild and verify locally, then commit both the source and updated `dist/`. Vercel does not compile Rust automatically with this configuration.

`dist/` is generated output, not a second source implementation. Do not edit it directly. Trunk replaces it on each build.

## Windows download

`public/downloads/tesserun-0.3.0-windows-x64.exe` is copied from the successful native Tesserun release build. It is a portable executable, not an installer. The page identifies it as an unsigned early release with compatibility on other machines unverified. `SHA256SUMS.txt` identifies the exact binary.

For a new release, replace the binary and checksum in `public/downloads/`, update version/size text and links in the HTML and tour component, rebuild, and verify. Test on a clean Windows machine before announcing broad compatibility.

The tour is illustrative: it never calls providers or publishes projects. The website uses no analytics, cookies, credentials, or backend. Google Fonts supplies optional typography with system-font fallbacks. Downloads and FAQs work without WebAssembly; the interactive tour requires it.
