## 🎯 **Mission: Build Professional Radix UI Components for Leptos using Tailwind CSS 4**
 

**Goal**: Create a complete Radix UI component library for Leptos using Tailwind CSS 4 by reverse engineering the code for working Leptos components in the RustForWeb/radix repository and Leptix components repository. Build a predictable factory that converts WAI-ARIA specs into production-ready, professionally polished Rust primitives at scale. Create a rust based presentation page for the components inspired by https://ant.design/components/overview/

## 🏗️ **Architecture Foundation**

### **Tech Stack**
- **Leptos 0.8.2** - Latest reactive framework
- **Tailwind CSS 4** - Latest utility-first CSS (stable as of Jan 2025)
- **Professional Utilities** - Rust for Web and Leptix-inspired patterns
- **Rust-only** - No JavaScript dependencies

### **Design System**
- **Dark theme default** - Professional appearance
- **Compact layouts** - Ant Design inspired
- **Consistent spacing** - Tailwind's spacing scale
- **Professional polish** - Micro-transitions, stateful styling

**Primary Source:** [Radix UI Primitives](https://github.com/radix-ui/primitives/tree/main) (50+ primitives)
**Additional Source:** [WAI-ARIA Specification](https://www.w3.org/TR/wai-aria-1.2/) (for component behavior)
**Translation Reference:** [RustForWeb/radix](https://github.com/RustForWeb/radix/tree/main/packages/primitives/leptos) and [Leptix](https://github.com/leptix/leptix/tree/master/crates/primitives/src/components) (Rosetta Stone)

### 🗺️ Translation Strategy:
1. **Radix UI = Source of Truth** - Official React implementation, complete API, battle-tested
2. **RustForWeb and/or Leptix = Rosetta Stone** - Shows React → Rust translation patterns (when available)
3. **Our Implementation = Production Ready** - Leptos 0.8.2, optimized, complete

### 📊 Coverage Analysis:
- **Radix UI Primitives:** 50+ components (Accordion, Alert Dialog, Avatar, Badge, etc.)
- **RustForWeb Coverage:** ~15 components (partial implementations)
- **Our Opportunity:** 35+ primitives to implement from scratch using proven patterns

## 🔄 **Development Phases**
Develop each component through the following 6 phases, taking one phas at a time and validating success criteria before moving to the next. 

### 🎯 **Phase I: Basic Functionality**
**Goal:** Get compiling and basic rendering with core props
**Primary Reference:** Radix UI React source
**Success Criteria:** Component renders with basic functionality
**Key Learning:** TBD

### 🎯 **Phase II: WAI-ARIA Compliance**
**Goal:** Build accessibility into Phase I (not separate phase)
**Strategy:** ARIA compliance achieved from initial implementation
**Success Criteria:** Full accessibility compliance with WAI-ARIA specification
**Key Learning:** TBD

### 🎯 **Phase III: Production Features**
**Goal:** Controllable state, form integration, advanced props
**Key Pattern:** Reuse proven patterns from previous components (80% code reuse)
**Success Criteria:** Production-ready functionality with error handling
**Key Learning:** TBD

### 🎯 **Phase IV: Visual Styling - TAILWIND ONLY**
**Goal:** Professional visual styling using ONLY Tailwind CSS utility classes
**Strategy:** Match official Radix UI styling exactly using Tailwind utilities
**Success Criteria:** Pixel-perfect visual behavior with responsive design
**MANDATORY:** No external CSS files, no custom CSS, no CSS custom properties - ONLY Tailwind CSS 4
**Key Learning:** TBD

### 🎯 **Phase V: Professional Polish - THE FINAL 10%**
**Goal:** Transform functional components into premium ones (This 10% separates good components from truly professional ones)
**Focus:** Micro-transitions, stateful styling, data-state styling, pixel-perfect spacing
**Success Criteria:** Component feels premium and indistinguishable from Radix UI
**Key Learning:** TBD

### 🎯 **Phase VI: Testing & Documentation - MANDATORY**
**Goal:** Comprehensive test suite and complete API documentation
**Testing Strategy:** leptos-test crate with 100% coverage
**Documentation Strategy:** Documentation-Driven Development approach
**Success Criteria:** Production-ready quality gates passed
**Key Learning:** TBD

### **Technical Blueprint to Convert Radix UI React Components to Leptos 0.8.2 with Tailwind CSS 4**
Begin with the checkbox, switch, and progress components. Use the existing Leptos repositories for reference (i.e., RustForWeb/radix and Leptix). As we build each component, we will document the process and patterns used. Continue to build out the 6-phase recipe as we go for each component, and with each identify learnings and optimizations for the next component. With each copmponent, we should be less dependent on the existing Leptos repositories for reference. After completing 10 components, we will have a solid understanding of the process and can begin to build out the remaining components more quickly without relying on the existing Leptos repositories for reference. This will serve as a blueprint for building the remaining 35+ components.

Create a separate md document called 'BLUEPRINT.md' that documents the process and patterns used to build each component. This will serve as a blueprint for building the remaining components. The blueprint will evolve as each component is built. 

## 📚 **Reference Materials**

- **Leptix Analysis**: `COMPARATIVE_ANALYSIS_RESULTS.md`
- **Original Recipe**: `_archive/LEPTOS_PRIMITIVE_RECIPE_v6_ARCHIVED.md`
- **Radix UI Source**: https://github.com/radix-ui/primitives/tree/main/packages/react
- **RustForWeb Radix Components**: https://github.com/RustForWeb/radix/tree/main/packages/primitives/leptos
WAI ARIA Patterns**: https://www.w3.org/WAI/ARIA/apg/patterns/
- **Leptix Components**: https://github.com/leptix/leptix/tree/master/crates/primitives/src/components
- **Tailwind CSS 4**: https://tailwindcss.com/docs

**Ready to build the future of Leptos UI components!** ⚡
