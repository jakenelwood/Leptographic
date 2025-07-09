### Set-Up

### **Phase 0: The Foundation - A Clean Workshop**

Before we start cooking, we need to set up the kitchen. This phase is about creating a perfect, repeatable Leptos development environment.

1. **Initialize the Project:** Do not start from scratch. Use `cargo-leptos` to create a new, full-stack project from the official template. This gives you Server-Side Rendering (SSR) and all the necessary tooling out of the box. Follow the setup guide here: [**https://book.leptos.dev/ssr/21_cargo_leptos.html**](https://book.leptos.dev/ssr/21_cargo_leptos.html).
2. **Configure for Developer Experience (DX):** Your time is valuable. Make these small changes to your project to enable faster and more ergonomic development, as recommended by the Leptos team. This includes setting up the nightly toolchain for function-call signals (`count()` instead of `count.get()`). Follow the steps here: [**https://book.leptos.dev/getting_started/leptos_dx.html**](https://book.leptos.dev/getting_started/leptos_dx.html).
3. **Integrate Tailwind CSS:** Your `shadcn-ui` styling depends on Tailwind. You must integrate it correctly. Follow the official Leptos guide for this. The key is to configure your `tailwind.config.js` to scan your `.rs` and `.html` files for classes.JavaScript
    
    `// tailwind.config.js
    /** @type {import('tailwindcss').Config} */
    module.exports = {
      content: [
        "*.html",
        "./src/**/*.rs", // Scans all your Rust files
      ],
      // ... rest of your config
    }`
    
4. **Create Your Component Hub:** Inside your `src` directory, create a new `components` folder and a `mod.rs` file inside it (`src/components/mod.rs`). This is where your new, custom-built UI library will live. Every component you build will be a new module in this directory.

**Goal for Phase 0:** A blank Leptos application running locally via `cargo leptos watch`. You should be able to add a Tailwind class like `bg-red-500` to an element in your `app.rs` file and see the change in your browser.
