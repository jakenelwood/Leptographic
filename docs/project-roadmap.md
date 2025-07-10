# shadcn/ui for Leptos - Project Roadmap 🗺️

## 🎯 Mission Statement

**Create the complete shadcn/ui ecosystem for Leptos**, bringing the beloved copy-paste component philosophy from React to Rust with full feature parity and Leptos-native patterns.

## 📊 Progress Overview

### **Overall Progress: 6.9% Complete**
- **Phase 1**: 🔄 In Progress (4/58 Radix primitives with Professional Polish complete)
- **Phase 2**: 📋 Planned (shadcn foundation)
- **Phase 3**: 📋 Planned (CLI ecosystem)

### **Timeline: 19-28 weeks total**
- **Phase 1**: 9-14 weeks (Radix primitives with Professional Polish)
- **Phase 2**: 4-6 weeks (shadcn foundation)
- **Phase 3**: 6-8 weeks (CLI & ecosystem)

## 🏗️ Phase 1: Radix Primitives Foundation (Current)

### **Goal**: Complete behavioral primitive library
**Status**: 🔄 In Progress (4/58 components - Checkbox, Switch, Progress, Separator, Label with Professional Polish complete)
**Timeline**: 9-14 weeks
**Repository**: `leptos-radix-ui` → `packages/radix-leptos`

### **🎯 Current Strategy: Components First, Documentation Later**
**Focus**: Building component ecosystem before creating comprehensive documentation site
**Rationale**: Establish robust library foundation, then showcase with professional documentation as capstone project

### **Milestones**

#### **Milestone 1: Tier 1 Complete** (Week 3-4) - 🔄 **71% Complete**
- [x] **Checkbox** - ✅ Complete with Professional Polish (6-phase recipe)
- [x] **Switch** - ✅ Complete with Professional Polish (6-phase recipe)
- [x] **Progress** - ✅ Complete with Professional Polish (6-phase recipe)
- [x] **Separator** - ✅ Complete with Professional Polish (6-phase recipe)
- [x] **Label** - ✅ Complete with Professional Polish (6-phase recipe)
- [ ] Toggle - Toggle button states
- [ ] Aspect Ratio - Responsive containers

**Success Criteria**: 7 core components, Professional Polish standards established
**Current Status**: 5/7 complete, 6-phase recipe with Professional Polish established, 80% code reuse achieved

### **🎯 Immediate Next Steps**

#### **Next Component: Toggle** (Priority 1)
- **Complexity**: Medium (2-3 hours with Professional Polish)
- **Purpose**: Toggle button states with ARIA support
- **Value**: Complete Tier 1 milestone
- **Reference**: [Radix Toggle](https://www.radix-ui.com/primitives/docs/components/toggle)

#### **Following Components** (Priority Order)
1. **Label** - Form association patterns
2. **Toggle** - Button state management
3. **Aspect Ratio** - Responsive containers

#### **Milestone 2: Tier 2 Complete** (Week 7-9)
- [ ] Accordion - Collapsible content sections
- [ ] Tabs - Tab navigation with keyboard support
- [ ] Dialog - Modal dialogs with focus trapping
- [ ] Popover - Floating content positioning
- [ ] Dropdown Menu - Context menus
- [ ] Navigation Menu - Site navigation
- [ ] Scroll Area - Custom scrollbars
- [ ] Collapsible - Show/hide content

**Success Criteria**: 8 layout components, 50% direct translation achieved

#### **Milestone 3: Independence** (Week 12-14)
- [ ] All 58 Radix primitives complete
- [ ] 100% direct React → Leptos translation
- [ ] Zero RustForWeb dependency
- [ ] Full accessibility compliance

**Success Criteria**: Complete primitive library, production-ready

## 🎨 Phase 2: shadcn Foundation (Future)

### **Goal**: Build shadcn component system
**Status**: 📋 Planned
**Timeline**: 4-6 weeks
**Repository**: `packages/shadcn-leptos`

### **Milestones**

#### **Milestone 4: Styling Foundation** (Week 15-16)
- [ ] Tailwind CSS integration patterns
- [ ] Class utility system (`clsx` + `tailwind-merge`)
- [ ] Animation utilities (`tailwindcss-animate`)
- [ ] Theme system and design tokens

#### **Milestone 5: Core Components** (Week 17-18)
- [ ] Button - All variants and sizes
- [ ] Input - Text inputs with validation
- [ ] Label - Enhanced form labels
- [ ] Card - Content containers
- [ ] Badge - Status indicators

#### **Milestone 6: Advanced Components** (Week 19-20)
- [ ] Dialog - Styled modal dialogs
- [ ] Select - Dropdown selects
- [ ] Tabs - Styled tab navigation
- [ ] Accordion - Styled collapsible content
- [ ] Form - Complete form system

**Success Criteria**: 20+ shadcn components, theme system working

## 🚀 Phase 3: CLI & Complete Ecosystem (Future)

### **Goal**: Complete developer experience
**Status**: 📋 Planned  
**Timeline**: 6-8 weeks
**Repository**: `packages/cli`

### **Milestones**

#### **Milestone 7: CLI Foundation** (Week 21-23)
- [ ] CLI tool architecture
- [ ] Component template system
- [ ] Project initialization (`shadcn-leptos init`)
- [ ] Basic component installation

#### **Milestone 8: Advanced CLI** (Week 24-26)
- [ ] Full component library available
- [ ] Configuration customization
- [ ] Theme switching
- [ ] Dependency management

#### **Milestone 9: Ecosystem Complete** (Week 27-28)
- [ ] Documentation site
- [ ] Component playground
- [ ] Example applications
- [ ] Community adoption

**Success Criteria**: Full shadcn/ui experience in Leptos

## 🎯 Target Developer Experience

### **End Goal: This Should Work**

```bash
# Install CLI
cargo install shadcn-leptos-cli

# Initialize project
cargo leptos new my-app
cd my-app
shadcn-leptos init

# Add components
shadcn-leptos add button dialog select

# Use in code
```

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
                <h2>"Perfect shadcn/ui experience in Leptos!"</h2>
            </DialogContent>
        </Dialog>
    }
}
```

## 📈 Success Metrics

### **Phase 1 Success** (Radix Complete)
- [ ] 58/58 Radix primitives ported
- [ ] 100% API compatibility with Radix UI
- [ ] Full accessibility compliance (WCAG 2.1 AA)
- [ ] Production performance benchmarks met

### **Phase 2 Success** (shadcn Foundation)  
- [ ] 30+ shadcn components working
- [ ] Tailwind integration seamless
- [ ] Theme system functional
- [ ] Developer experience smooth

### **Phase 3 Success** (Complete Ecosystem)
- [ ] CLI tool fully functional
- [ ] All shadcn components available via CLI
- [ ] Documentation site complete
- [ ] Community adoption growing

## 🔄 Current Focus: Switch Component

**Next Immediate Steps:**
1. Research Radix UI Switch and RustForWeb Switch
2. Implement Switch using proven Checkbox patterns
3. Add switch-specific ARIA attributes
4. Test accessibility compliance
5. Document patterns for future components

**This roadmap transforms Leptos into a first-class citizen of the shadcn/ui ecosystem!** 🦀✨
