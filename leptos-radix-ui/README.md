# shadcn/ui for Leptos 🚀

**The complete shadcn/ui ecosystem for Leptos** - from Radix primitives to copy-paste components with CLI tooling.

## 🎯 Project Vision

We're building the **complete shadcn/ui experience for Leptos**:

1. **🏗️ Radix Primitives** - Unstyled, accessible behavioral components (Phase 1 - Current)
2. **🎨 Tailwind Integration** - Seamless styling with class utilities (Phase 2)
3. **🔧 Component Library** - Beautiful, customizable shadcn components (Phase 2)
4. **🚀 CLI Tool** - Copy-paste component system like the original (Phase 3)

## 📋 Current Status: Phase 1 - Radix Foundation

**Building the behavioral foundation that powers shadcn/ui**

### 🎯 **Strategy: Components First, Documentation Later**
We're focusing on building the component ecosystem before creating comprehensive documentation. This ensures a robust foundation before showcasing it as a capstone project.

## ✅ Completed Components (3/58)

- **Checkbox** ✅ - Phase IV complete with variants, states, and accessibility
- **Switch** ✅ - Phase IV complete with ARIA switch role and smooth transitions
- **Progress** ✅ - Phase IV complete with animations and visual feedback

## 🚀 Next Up: Separator Component

**Target**: Simple visual divider component (1-2 hour implementation)
**Purpose**: Maintain momentum while validating our 4-phase recipe on simpler components

## 🏗️ The 4 Pillars We're Building

### **Pillar 1: Radix Primitives** (Current Phase)
- 58 unstyled, accessible behavioral components
- Full WAI-ARIA compliance and keyboard navigation
- Complex state management (Dialog, Select, Tooltip)
- **Timeline**: 9-14 weeks

### **Pillar 2: Tailwind Integration** (Phase 2)
- Seamless Tailwind CSS setup for Leptos
- Class merging utilities (`clsx` + `tailwind-merge`)
- Animation and transition system
- **Timeline**: 4-6 weeks

### **Pillar 3: shadcn Components** (Phase 2)
- Beautiful, customizable components built on Radix
- Complete shadcn/ui component library
- Theme system and design tokens
- **Timeline**: Concurrent with Pillar 2

### **Pillar 4: CLI Tool** (Phase 3)
- `shadcn-leptos add button` copy-paste system
- Project initialization and configuration
- Template customization
- **Timeline**: 6-8 weeks

## Running your project

```bash
cargo leptos watch
```

## Installing Additional Tools

By default, `cargo-leptos` uses `nightly` Rust, `cargo-generate`, and `sass`. If you run into any trouble, you may need to install one or more of these tools.

1. `rustup toolchain install nightly --allow-downgrade` - make sure you have Rust nightly
2. `rustup target add wasm32-unknown-unknown` - add the ability to compile Rust to WebAssembly
3. `cargo install cargo-generate` - install `cargo-generate` binary (should be installed automatically in future)
4. `npm install -g sass` - install `dart-sass` (should be optional in future
5. Run `npm install` in end2end subdirectory before test

## Compiling for Release
```bash
cargo leptos build --release
```

Will generate your server binary in target/server/release and your site package in target/site

## Testing Your Project
```bash
cargo leptos end-to-end
```

```bash
cargo leptos end-to-end --release
```

Cargo-leptos uses Playwright as the end-to-end test tool.  
Tests are located in end2end/tests directory.

## Executing a Server on a Remote Machine Without the Toolchain
After running a `cargo leptos build --release` the minimum files needed are:

1. The server binary located in `target/server/release`
2. The `site` directory and all files within located in `target/site`

Copy these files to your remote server. The directory structure should be:
```text
leptos-radix-ui
site/
```
Set the following environment variables (updating for your project as needed):
```sh
export LEPTOS_OUTPUT_NAME="leptos-radix-ui"
export LEPTOS_SITE_ROOT="site"
export LEPTOS_SITE_PKG_DIR="pkg"
export LEPTOS_SITE_ADDR="127.0.0.1:3000"
export LEPTOS_RELOAD_PORT="3001"
```
Finally, run the server binary.

## Licensing

This template itself is released under the Unlicense. You should replace the LICENSE for your own application with an appropriate license if you plan to release it publicly.
