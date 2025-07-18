# Phase III Transition Status: Leptos-Radix Checkbox Component

**Date:** July 11, 2025  
**Project:** Leptos-Radix UI Component Library  
**Current Phase:** Phase II → Phase III Transition  
**Repository:** https://github.com/jakenelwood/themachine  

---

## 🎯 **Current Status: READY FOR PHASE III**

### ✅ **Accomplished (Phase II Complete)**

#### **1. Tailwind CSS v4 + Leptos 0.8.2 Setup (WORKING)**
- ✅ **Tailwind v4.1.11** properly installed with `@tailwindcss/cli`
- ✅ **Processing pipeline** established: `style/main.scss` → `style/output.css`
- ✅ **Cargo.toml configuration** pointing to processed CSS
- ✅ **Watch mode setup** for hot-reloading development
- ✅ **Real CSS generation** (no more raw `@tailwind` directives)

**Build Commands:**
```bash
# Terminal 1: Tailwind watch
npx @tailwindcss/cli -i style/main.scss -o style/output.css --watch

# Terminal 2: Leptos serve
cargo leptos serve
```

#### **2. Phase II Checkbox Features (PRODUCTION-READY UI)**
- ✅ **WAI-ARIA compliance** (role="checkbox", aria-checked, keyboard navigation)
- ✅ **Form integration** (hidden inputs, name/value/required props)
- ✅ **Disabled states** with proper visual feedback
- ✅ **Professional styling** (shadows, focus rings, transitions)
- ✅ **Dark theme support** with proper contrast
- ✅ **Context system** (CheckboxContext for state sharing)
- ✅ **Compound components** (Checkbox + CheckboxIndicator)

**Current API:**
```rust
<Checkbox 
    default_checked=CheckedState::True
    aria_label="Accept terms".to_string()
    name="terms".to_string()
    disabled=false
>
    <CheckboxIndicator>"✓"</CheckboxIndicator>
</Checkbox>
```

#### **3. Infrastructure in Place**
- ✅ **Component structure** (`src/components/checkbox.rs`)
- ✅ **Demo system** (`ComponentTestPage` with live examples)
- ✅ **Theme system** (dark/light mode toggle)
- ✅ **Build pipeline** (Tailwind + Leptos integration)
- ✅ **Development workflow** (watch mode, hot reload)

---

## 🚀 **Phase III Requirements: Production Features**

### **The 5 Universal Production Patterns**

#### **Pattern 1: Advanced Prop System**
**Current:** Basic optional props  
**Target:** Production-grade prop system with `MaybeProp<T>`

```rust
// BEFORE (Phase II)
#[prop(optional)] disabled: Option<bool>,

// AFTER (Phase III)
#[prop(into, optional)] disabled: MaybeProp<bool>,
#[prop(into, optional)] as_child: MaybeProp<bool>,
#[prop(optional)] node_ref: NodeRef<AnyElement>,
#[prop(attrs)] attrs: Vec<(&'static str, Attribute)>,
```

#### **Pattern 2: Controllable State Hook**
**Current:** Internal state only (`RwSignal<CheckedState>`)  
**Target:** External + internal state management

```rust
// BEFORE (Phase II)
let checked = RwSignal::new(default_checked.unwrap_or_default());

// AFTER (Phase III)
#[prop(into, optional)] checked: MaybeProp<CheckedState>,
#[prop(into, optional)] on_checked_change: Option<Callback<CheckedState>>,

let (checked_state, set_checked) = use_controllable_state(
    checked, default_checked, on_checked_change
);
```

#### **Pattern 3: Primitive Component System**
**Current:** Direct HTML elements  
**Target:** Composable primitive wrapper

```rust
// BEFORE (Phase II)
<button type="button" role="checkbox">

// AFTER (Phase III)
<Primitive
    element=html::button
    as_child=as_child
    node_ref=composed_refs
    attrs=attrs
>
```

#### **Pattern 4: Event Composition**
**Current:** Simple event handlers  
**Target:** Composed user + internal callbacks

```rust
// BEFORE (Phase II)
let handle_click = move |_| { /* toggle logic */ };

// AFTER (Phase III)
let handle_click = compose_callbacks(
    user_on_click,
    move |_| { /* internal logic */ }
);
```

#### **Pattern 5: Enhanced Form Integration**
**Current:** Basic hidden input  
**Target:** BubbleInput component for proper event bubbling

```rust
// BEFORE (Phase II)
<input type="checkbox" name=name />

// AFTER (Phase III)
<BubbleInput
    control=checkbox_ref
    bubbles=["click", "focus", "blur"]
/>
```

---

## 🔧 **Infrastructure Components to Build**

### **Core Hooks (Must Build Once)**

#### **1. `use_controllable_state` Hook**
**File:** `src/hooks/use_controllable_state.rs`  
**Lines:** ~70  
**Time:** 1-2 hours  
**Purpose:** External + internal state management

```rust
fn use_controllable_state<T>(
    controlled_value: MaybeProp<T>,
    default_value: MaybeProp<T>, 
    on_change: Option<Callback<T>>,
) -> (Signal<Option<T>>, Callback<Option<T>>)
```

#### **2. `use_composed_refs` Hook**
**File:** `src/hooks/use_composed_refs.rs`  
**Lines:** ~40  
**Time:** 30-60 min  
**Purpose:** Merge multiple NodeRef instances

#### **3. `compose_callbacks` Utility**
**File:** `src/utils/compose_callbacks.rs`  
**Lines:** ~30  
**Time:** 30 min  
**Purpose:** Combine user and internal event handlers

### **Component Infrastructure**

#### **4. `Primitive` Component**
**File:** `src/primitive.rs`  
**Lines:** ~100  
**Time:** 1-2 hours  
**Purpose:** Composable element wrapper with as_child support

#### **5. `BubbleInput` Component**
**File:** `src/components/bubble_input.rs`  
**Lines:** ~80  
**Time:** 1 hour  
**Purpose:** Form integration with proper event bubbling

**Total Infrastructure:** ~320 lines, 4-6 hours one-time investment

---

## 📋 **Implementation Plan**

### **Step 1: Create Hook Infrastructure (2-3 hours)**

1. **Create hooks module structure**
   ```bash
   mkdir -p src/hooks
   touch src/hooks/mod.rs
   touch src/hooks/use_controllable_state.rs
   touch src/hooks/use_composed_refs.rs
   ```

2. **Implement `use_controllable_state`**
   - Handle controlled vs uncontrolled state
   - Callback composition for state changes
   - Default value fallback logic

3. **Implement `use_composed_refs`**
   - Merge multiple NodeRef instances
   - Handle optional refs gracefully

### **Step 2: Create Primitive System (1-2 hours)**

1. **Create `src/primitive.rs`**
   - Polymorphic element rendering
   - `as_child` prop support
   - Attribute forwarding system

2. **Create `src/utils/mod.rs`**
   - `compose_callbacks` utility
   - Event handler composition

### **Step 3: Create Form Infrastructure (1 hour)**

1. **Create `BubbleInput` component**
   - Hidden input with event bubbling
   - Form submission integration
   - Accessibility preservation

### **Step 4: Upgrade Checkbox Component (1-2 hours)**

1. **Replace props with advanced system**
2. **Integrate controllable state hook**
3. **Replace HTML with Primitive**
4. **Add event composition**
5. **Update demo with external state examples**

### **Step 5: Testing & Validation (30 min)**

1. **Test controllable state**
2. **Verify form integration**
3. **Check accessibility preservation**
4. **Validate against Phase III criteria**

---

## 📁 **Technical Context**

### **Key File Locations**
```
leptos-radix-ui/
├── src/
│   ├── components/
│   │   └── checkbox.rs          # Current Phase II implementation
│   ├── hooks/                   # TO CREATE: Phase III hooks
│   ├── utils/                   # TO CREATE: Utility functions
│   ├── primitive.rs             # TO CREATE: Primitive component
│   └── lib.rs                   # Module exports
├── style/
│   ├── main.scss               # Tailwind input (processed)
│   └── output.css              # Generated CSS (used by Leptos)
├── Cargo.toml                  # Points to style/output.css
├── tailwind.config.js          # Scans .rs files
└── LEPTOS_PRIMITIVE_RECIPE.md  # Complete implementation guide
```

### **Build Configuration**
- **Leptos:** 0.8.2 with cargo-leptos
- **Tailwind:** v4.1.11 with @tailwindcss/cli
- **CSS Processing:** `style-file = "style/output.css"` in Cargo.toml
- **Content Scanning:** `"./src/**/*.rs"` in tailwind.config.js

### **Development Workflow**
```bash
# Start development (2 terminals)
npx @tailwindcss/cli -i style/main.scss -o style/output.css --watch
cargo leptos serve

# Access: http://127.0.0.1:3000
```

---

## ✅ **Success Criteria for Phase III**

### **Functional Requirements**
- [ ] External state control works (`checked` prop)
- [ ] Callback system works (`on_checked_change`)
- [ ] `as_child` prop enables composition
- [ ] Form integration maintains accessibility
- [ ] All Phase II features preserved

### **API Requirements**
```rust
// Target Phase III API
<Checkbox 
    checked=external_state
    on_checked_change=handle_change
    as_child=false
    node_ref=checkbox_ref
    disabled=is_disabled
>
    <CheckboxIndicator />
</Checkbox>
```

### **Infrastructure Requirements**
- [ ] `use_controllable_state` hook working
- [ ] `Primitive` component rendering correctly
- [ ] `compose_callbacks` utility functional
- [ ] All hooks properly exported in `lib.rs`

---

## 🎯 **Next Steps: Immediate Actions**

### **1. FIRST TASK: Create Hook Infrastructure**
```bash
cd leptos-radix-ui
mkdir -p src/hooks src/utils
```

**Start with:** `src/hooks/use_controllable_state.rs`  
**Reference:** Lines 693-703 in `LEPTOS_PRIMITIVE_RECIPE.md`

### **2. Validation Strategy**
- Test each hook independently before integration
- Maintain Phase II functionality during transition
- Use existing demo page for testing

### **3. Connection to Broader Strategy**
- **Phase III infrastructure** enables 80% code reuse across all components
- **Next components:** Switch, Progress, Separator (all follow same patterns)
- **Weaning goal:** Reduce RustForWeb dependency from 70% to 30%
- **Timeline:** Phase III checkbox validates the entire system for mass production

---

## 🚂 **The Machine Strategy Context**

**Current Position:** Phase II checkbox proves the Tailwind + Leptos pipeline works  
**Phase III Goal:** Build reusable infrastructure for 50+ components  
**Economics:** 4-6 hour investment enables 5x speed improvement for all future components  
**Outcome:** Production-ready component library competitive with shadcn/ui for React

**Ready to build the infrastructure that powers the entire component ecosystem!** 🚀
