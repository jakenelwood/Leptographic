# Leptos shadcn/ui Ecosystem Implementation Plan v3.0 🚀

## 🎯 Strategic Vision: Complete shadcn/ui Ecosystem for Leptos

**We're not just porting Radix UI - we're building the foundation for the entire shadcn/ui ecosystem in Leptos!**

### **The 4 Pillars of shadcn/ui → Leptos**

1. **🏗️ Foundation: Radix Primitives** (Phase 1 - Current)
2. **🎨 Styling Engine: Tailwind CSS Integration** (Phase 2)
3. **🔧 Utilities: Class Merging & Conditional Styling** (Phase 3)
4. **🚀 CLI Tool: Copy-Paste Component System** (Phase 4)

## 🎯 Phase 1: Radix UI → Leptos Translation (Current Focus)

### **Primary Source of Truth**
- **Radix UI Primitives**: https://github.com/radix-ui/primitives/tree/main (50+ primitives)
- **Official Documentation**: https://www.radix-ui.com/primitives/docs/components
- **Target Structure**: Mirror the complete Radix UI ecosystem in Leptos 0.8.2

### **Translation Strategy**
- **RustForWeb as Rosetta Stone**: https://github.com/RustForWeb/radix/tree/main/packages/primitives/leptos
- **Systematic Weaning Process**: Reduce dependency on RustForWeb over 12 components
- **Direct Translation Goal**: React → Leptos without intermediate references

### **MCP Server Resources**
- **Context7 MCP**: React (/context7/react_dev), Rust (/rust-lang/rust), Leptos (/leptos-rs/book)
- **Octocode MCP**: GitHub code search, NPM packages, repository analysis
- **Streamlined Research**: Leverage both servers for comprehensive information gathering

## � **The Button Question: Why No Button Primitive?**

**This is one of the most common questions in the Radix community.** Here's the definitive answer:

### **Radix Philosophy: Solve Hard Problems, Not Easy Ones**

Radix Primitives exist to handle **complex behavioral and accessibility patterns** that are difficult to implement correctly. The native HTML `<button>` element already provides:
- ✅ Focusable by default
- ✅ Keyboard-activatable with `Enter` and `Space`
- ✅ Correct `role="button"` for screen readers
- ✅ Form integration built-in

**A Button primitive would violate core Radix principles:**
- **Unstyled**: Button styling (variants, sizes, icons) is a design concern, not behavioral
- **Behavioral Focus**: Radix solves state management, focus control, ARIA patterns
- **Composition Over Configuration**: Use `as_child` prop to integrate custom buttons

### **The Leptos Pattern: Custom Button + `as_child`**

```rust
// User creates their own styled Button (NOT in our library)
#[component]
fn StyledButton(
    children: ChildrenFn,
    #[prop(attrs)] attrs: Vec<(&'static str, Attribute)>,
) -> impl IntoView {
    view! {
        <button
            class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded"
            attrs=attrs  // Receives ARIA attributes and handlers from primitives
        >
            {children()}
        </button>
    }
}

// User integrates with our primitives via as_child
#[component]
fn App() -> impl IntoView {
    view! {
        <Dialog>
            <DialogTrigger as_child=true>
                <StyledButton>"Open Dialog"</StyledButton>  // Gets all Dialog behavior
            </DialogTrigger>
            <DialogContent>"Dialog content"</DialogContent>
        </Dialog>
    }
}
```

**References:**
- [GitHub Issue #892](https://github.com/radix-ui/primitives/issues/892) - "Closed as not planned"
- [Community Discussion #1560](https://github.com/radix-ui/primitives/discussions/1560)
- [Radix Styling Philosophy](https://radix-ui.com/primitives/docs/guides/styling)

---

## �📋 Complete Primitive Catalog (58 Components)

### ✅ **Completed (2/58)**
- **Checkbox** - Phase III production-ready
- **Arrow** - Complete implementation

### 🎯 **Priority Tier 1: Core UI (6 components)**
**Goal: Wean off RustForWeb by component 5-7**

**⚠️ IMPORTANT: No Button Primitive by Design**
Following Radix UI philosophy, we intentionally **do not** provide a Button primitive. The native HTML `<button>` element is already accessible and powerful. Our primitives focus on complex behavioral patterns that require sophisticated state management and ARIA compliance.

**Why No Button?**
- Native `<button>` already provides accessibility (focusable, keyboard-activatable, correct ARIA role)
- Button styling is a design concern, not a behavioral primitive concern
- Radix philosophy: "unstyled, accessible, behavioral primitives"
- Users integrate custom buttons via `as_child` prop pattern
- See: [GitHub Issue #892](https://github.com/radix-ui/primitives/issues/892) - officially "Closed as not planned"

**Tier 1 Components:**
1. **Switch** - Toggle component (RustForWeb available)
2. **Progress** - Progress indicators (RustForWeb available)
3. **Separator** - Dividers (RustForWeb available)
4. **Label** - Form labels (RustForWeb available)
5. **Toggle** - Toggle button (RustForWeb available)
6. **Aspect Ratio** - Responsive containers (RustForWeb available)

### 🎯 **Priority Tier 2: Layout & Navigation (8 components)**
**Goal: 50% direct translation, 50% RustForWeb reference**

7. **Accordion** - Collapsible content (RustForWeb available)
8. **Tabs** - Tab navigation (RustForWeb available)
9. **Dialog** - Modal dialogs (RustForWeb available)
10. **Popover** - Floating content (RustForWeb available)
11. **Dropdown Menu** - Context menus (RustForWeb available)
12. **Navigation Menu** - Site navigation (RustForWeb available)
13. **Scroll Area** - Custom scrollbars (RustForWeb available)
14. **Collapsible** - Show/hide content (RustForWeb available)

### 🎯 **Priority Tier 3: Form Components (8 components)**
**Goal: 75% direct translation**

15. **Radio Group** - Radio button groups (RustForWeb available)
16. **Select** - Dropdown selects (RustForWeb available)
17. **Slider** - Range inputs (RustForWeb available)
18. **Toggle Group** - Button groups (RustForWeb available)
19. **Form** - Form validation (No RustForWeb - Direct translation)
20. **Toolbar** - Tool collections (RustForWeb available)
21. **One Time Password Field** - OTP input (No RustForWeb - Direct translation)
22. **Password Toggle Field** - Password visibility (No RustForWeb - Direct translation)

### 🎯 **Priority Tier 4: Advanced Components (15 components)**
**Goal: 90% direct translation**

23. **Alert Dialog** - Confirmation dialogs (RustForWeb available)
24. **Context Menu** - Right-click menus (RustForWeb available)
25. **Hover Card** - Hover previews (RustForWeb available)
26. **Menubar** - Application menus (RustForWeb available)
27. **Toast** - Notifications (RustForWeb available)
28. **Tooltip** - Hover help (RustForWeb available)
29. **Avatar** - User avatars (No RustForWeb - Direct translation)
30. **Menu** - Base menu component (RustForWeb available)
31. **Portal** - Render elsewhere (RustForWeb available)
32. **Presence** - Animation presence (RustForWeb available)
33. **Primitive** - Base primitive (RustForWeb available)
34. **Roving Focus** - Focus management (RustForWeb available)
35. **Slot** - Composition utility (RustForWeb available)
36. **Visually Hidden** - Screen reader only (RustForWeb available)
37. **Direction** - RTL support (RustForWeb available)

### 🎯 **Priority Tier 5: Utility & Advanced (19 components)**
**Goal: 100% direct translation**

38. **Accessible Icon** - Icon accessibility (No RustForWeb)
39. **Announce** - Screen reader announcements (No RustForWeb)
40. **Collection** - Collection utilities (No RustForWeb)
41. **Compose Refs** - Ref composition (No RustForWeb)
42. **Context** - Context utilities (No RustForWeb)
43. **Dismissable Layer** - Click outside (No RustForWeb)
44. **Focus Guards** - Focus management (No RustForWeb)
45. **Focus Scope** - Focus trapping (No RustForWeb)
46. **ID** - ID generation (No RustForWeb)
47. **Popper** - Positioning engine (No RustForWeb)
48. **Use Callback Ref** - Callback refs (No RustForWeb)
49. **Use Controllable State** - State management (No RustForWeb)
50. **Use Effect Event** - Effect utilities (No RustForWeb)
51. **Use Escape Keydown** - Escape handling (No RustForWeb)
52. **Use Is Hydrated** - Hydration detection (No RustForWeb)
53. **Use Layout Effect** - Layout effects (No RustForWeb)
54. **Use Previous** - Previous value hook (No RustForWeb)
55. **Use Rect** - Element measurements (No RustForWeb)
56. **Use Size** - Size tracking (No RustForWeb)
57. **Radix UI** - Meta package (No RustForWeb)
58. **All Components** - Complete bundle (No RustForWeb)

## 🔄 Systematic Weaning Process

### **Phase 1: RustForWeb Dependency (Components 1-6)**
- **Research**: 70% RustForWeb analysis, 30% Radix UI
- **Implementation**: Direct translation of RustForWeb patterns
- **Goal**: Master Leptos 0.8.2 patterns and build confidence

### **Phase 2: Balanced Approach (Components 7-12)**
- **Research**: 50% RustForWeb reference, 50% Radix UI analysis
- **Implementation**: Compare both sources, prefer Radix UI API
- **Goal**: Develop direct translation skills

### **Phase 3: Radix UI Primary (Components 13-18)**
- **Research**: 25% RustForWeb reference, 75% Radix UI analysis
- **Implementation**: Radix UI first, RustForWeb for complex patterns only
- **Goal**: Build confidence in direct translation

### **Phase 4: Direct Translation (Components 19+)**
- **Research**: 100% Radix UI analysis
- **Implementation**: Direct React → Leptos translation
- **Goal**: Complete independence from RustForWeb

## 🛠️ Implementation Workflow

### **Step 1: MCP-Powered Research (15-30 min)**
```bash
# 1. Analyze Radix UI source
Context7: /context7/react_dev - React patterns and hooks
Octocode: radix-ui/primitives - Official React implementation

# 2. Check RustForWeb reference (Phase 1-3 only)
Octocode: RustForWeb/radix - Translation patterns

# 3. Study documentation
Context7: Official Radix UI docs for API understanding
```

### **Step 2: Extract Core Patterns (15 min)**
```typescript
// From Radix UI React source, identify:
// 1. Component structure (Root, Trigger, Content, etc.)
// 2. State management (controlled/uncontrolled)
// 3. Event handling patterns
// 4. ARIA implementation
// 5. Keyboard interactions
// 6. Form integration needs
```

### **Step 3: Apply Proven 0.8.2 Patterns (30 min - 6 hours)**
```rust
// Use our validated patterns:
// 1. Phase I: Basic functionality (30 min)
// 2. Phase II: WAI-ARIA compliance (2-3 hours)  
// 3. Phase III: Production features (4-6 hours)
```

## 📊 Success Metrics

### **Weaning Progress Tracking**
- **Component 1-6**: RustForWeb dependency acceptable
- **Component 7-12**: 50% reduction in RustForWeb references
- **Component 13-18**: 75% reduction in RustForWeb references  
- **Component 19+**: Zero RustForWeb dependency

### **Quality Standards**
- **API Compatibility**: 100% match with Radix UI React API
- **Accessibility**: Full WAI-ARIA compliance
- **Performance**: Leptos 0.8.2 optimized
- **Documentation**: Complete examples and API docs

### **Timeline Estimates**
- **Tier 1 (7 components)**: 2-3 weeks (learning curve)
- **Tier 2 (8 components)**: 2-3 weeks (efficiency gains)
- **Tier 3 (8 components)**: 1-2 weeks (direct translation)
- **Tier 4 (15 components)**: 2-3 weeks (advanced features)
- **Tier 5 (19 components)**: 2-3 weeks (utilities)

**Total Estimated Timeline**: 9-14 weeks for complete library

## 🚀 Next Actions

### **Phase 1: Radix Primitives (Current - 9-14 weeks)**
1. **Complete Switch Component**: First Tier 1 primitive using proven patterns
2. **Track Weaning Progress**: Monitor RustForWeb dependency reduction
3. **Scale Production**: Apply THE MACHINE to mass-produce primitives
4. **Document Patterns**: Refine workflow based on early components

### **Phase 2: shadcn/ui Foundation (Future - 4-6 weeks)**
1. **Tailwind Integration**: Document setup requirements for users
2. **Class Utilities**: Integrate `tailwind-merge-rs` and conditional styling
3. **Icon System**: Port or integrate Lucide icons for Leptos
4. **Component Templates**: Create shadcn-style component templates

### **Phase 3: CLI Tool (Future - 6-8 weeks)**
1. **CLI Architecture**: Design copy-paste component system
2. **Template Engine**: Build component generation system
3. **Configuration**: Project setup and customization
4. **Documentation**: Complete developer experience

---

*This plan creates the complete shadcn/ui ecosystem for Leptos - from behavioral primitives to copy-paste components with CLI tooling.* 🎯
