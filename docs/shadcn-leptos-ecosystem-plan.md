# shadcn/ui → Leptos Ecosystem Plan 🚀

## 🎯 Vision: Complete shadcn/ui Experience for Leptos

**Goal**: Create the definitive shadcn/ui ecosystem for Leptos, replicating the entire developer experience from React to Rust.

## 📋 The 4 Pillars of shadcn/ui

### **Pillar 1: Radix Primitives Foundation** 🏗️
**Status**: 🔄 In Progress (Phase 1)
**Timeline**: 9-14 weeks
**Repository**: `leptos-radix-ui` (current)

**What it provides:**
- Unstyled, accessible behavioral primitives
- Complex state management (Dialog, Select, Tooltip)
- ARIA compliance and keyboard navigation
- Focus management and event handling

**Dependencies:**
- Leptos 0.8.2+
- Web APIs (DOM, ARIA)

**Deliverables:**
- 58 Radix UI primitives ported to Leptos
- Production-ready components with Phase III patterns
- Comprehensive documentation and examples

### **Pillar 2: Tailwind CSS Integration** 🎨
**Status**: 📋 Planned (Phase 2)
**Timeline**: 4-6 weeks
**Repository**: `shadcn-leptos-styles` (future)

**What it provides:**
- Tailwind CSS configuration for Leptos
- Animation utilities (`tailwindcss-animate` equivalent)
- Design tokens and theme system
- Responsive design patterns

**Dependencies:**
- Tailwind CSS 3.0+
- `tailwindcss-animate` plugin
- Leptos CSS integration

**Deliverables:**
- Tailwind config templates for Leptos projects
- Animation and transition utilities
- Theme customization system
- Documentation for styling setup

### **Pillar 3: Styling Utilities** 🔧
**Status**: 📋 Planned (Phase 2)
**Timeline**: 2-3 weeks
**Repository**: `leptos-class-utils` (future)

**What it provides:**
- Conditional class name construction (`clsx` equivalent)
- Intelligent Tailwind class merging (`tailwind-merge` equivalent)
- Variant-based styling patterns
- Type-safe class composition

**Dependencies:**
- `tailwind-merge-rs` (existing crate)
- Leptos reactive system

**Deliverables:**
```rust
// Conditional classes
let classes = class_names![
    "base-class",
    ("conditional-class", condition),
    variant_class
];

// Tailwind merging
let merged = tw_merge!["p-2 p-4 bg-red-500", user_classes];
```

### **Pillar 4: CLI Tool & Copy-Paste System** 🚀
**Status**: 📋 Planned (Phase 3)
**Timeline**: 6-8 weeks
**Repository**: `shadcn-leptos-cli` (future)

**What it provides:**
- Component installation via CLI
- Project configuration and setup
- Template customization system
- Dependency management

**Dependencies:**
- Clap (CLI framework)
- Tokio (async runtime)
- Serde (configuration)
- Git integration

**Deliverables:**
```bash
# Install CLI
cargo install shadcn-leptos-cli

# Initialize project
shadcn-leptos init

# Add components
shadcn-leptos add button
shadcn-leptos add dialog
shadcn-leptos add select
```

## 🗂️ Project Structure Reorganization

### **Current Structure** (Radix Focus)
```
leptos-radix-ui/
├── src/components/          # Radix primitives
├── docs/                    # Implementation docs
└── examples/                # Component examples
```

### **Future Structure** (shadcn Ecosystem)
```
shadcn-leptos/
├── packages/
│   ├── radix-leptos/        # Radix primitives (current leptos-radix-ui)
│   ├── shadcn-leptos/       # shadcn components built on Radix
│   ├── class-utils/         # Styling utilities
│   └── cli/                 # CLI tool
├── apps/
│   ├── docs/                # Documentation site
│   ├── playground/          # Component playground
│   └── examples/            # Example applications
└── tools/
    ├── build/               # Build tools
    └── templates/           # Component templates
```

## 📊 Implementation Phases

### **Phase 1: Radix Foundation** (Current - 9-14 weeks)
**Repository**: `leptos-radix-ui` → `packages/radix-leptos`

**Milestones:**
- ✅ Checkbox, Arrow (Complete)
- 🔄 Switch, Progress, Separator (Tier 1)
- 📋 Dialog, Tabs, Select (Tier 2)
- 📋 All 58 primitives (Complete)

**Success Criteria:**
- All Radix primitives working in Leptos
- 100% API compatibility with Radix UI
- Full accessibility compliance
- Production-ready performance

### **Phase 2: shadcn Foundation** (4-6 weeks)
**Repository**: `packages/shadcn-leptos`

**Milestones:**
- Tailwind CSS integration patterns
- Class utility system (`clsx` + `tailwind-merge`)
- Icon system (Lucide integration)
- First 10 shadcn components

**Success Criteria:**
- Button, Input, Label components working
- Tailwind class merging functional
- Icon system integrated
- Theme customization working

### **Phase 3: CLI & Ecosystem** (6-8 weeks)
**Repository**: `packages/cli`

**Milestones:**
- CLI tool architecture
- Component template system
- Project initialization
- Full component library

**Success Criteria:**
- `shadcn-leptos add button` working
- Project setup automation
- All shadcn components available
- Documentation site complete

## 🎯 Target Developer Experience

### **Installation**
```bash
# Install CLI
cargo install shadcn-leptos-cli

# Initialize new project
cargo leptos new my-app
cd my-app
shadcn-leptos init
```

### **Adding Components**
```bash
# Add individual components
shadcn-leptos add button
shadcn-leptos add dialog
shadcn-leptos add select

# Add multiple components
shadcn-leptos add button dialog select
```

### **Usage in Code**
```rust
use leptos::*;
use crate::components::{Button, Dialog, DialogContent, DialogTrigger};

#[component]
fn App() -> impl IntoView {
    view! {
        <Dialog>
            <DialogTrigger as_child=true>
                <Button variant="outline">"Open Dialog"</Button>
            </DialogTrigger>
            <DialogContent>
                <h2>"Dialog Title"</h2>
                <p>"This is a dialog built with shadcn-leptos!"</p>
            </DialogContent>
        </Dialog>
    }
}
```

## 🔗 Ecosystem Integration

### **Existing Crates We'll Use**
- `tailwind-merge-rs` - Tailwind class merging
- `lucide-leptos` - Icon system (if exists, or we'll create it)
- `leptos-use` - Utility hooks
- `leptos-router` - Routing integration

### **New Crates We'll Create**
- `radix-leptos` - Radix primitives
- `shadcn-leptos` - shadcn components  
- `leptos-class-utils` - Class utilities
- `shadcn-leptos-cli` - CLI tool

## 🎉 Success Metrics

### **Phase 1 Success** (Radix Complete)
- [ ] 58 Radix primitives ported
- [ ] 100% API compatibility
- [ ] Full accessibility compliance
- [ ] Production performance

### **Phase 2 Success** (shadcn Foundation)
- [ ] 20+ shadcn components working
- [ ] Tailwind integration seamless
- [ ] Class utilities functional
- [ ] Theme system working

### **Phase 3 Success** (Complete Ecosystem)
- [ ] CLI tool fully functional
- [ ] All shadcn components available
- [ ] Documentation site complete
- [ ] Community adoption growing

---

**This plan transforms Leptos into a first-class citizen of the shadcn/ui ecosystem, bringing the beloved copy-paste component philosophy to Rust!** 🦀✨
