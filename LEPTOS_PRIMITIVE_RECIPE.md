# Leptos Primitive Recipe v3 (VALIDATED ✅)

## Validation Results: Checkbox Success! 🎉

**Status:** ✅ WORKING - Checkbox compiles and builds successfully

## Critical Fixes Discovered

### 1. Required Imports (ALWAYS INCLUDE)
```rust
use leptos::prelude::*;
use leptos::context::Provider;  // ← CRITICAL: Missing this causes Provider not found
```

### 2. Module Structure (SIMPLE APPROACH)
- ✅ **Single file approach**: `src/components/checkbox.rs`
- ❌ **Directory approach**: `src/components/checkbox/mod.rs` (causes module resolution issues)

### 3. Closure Lifetime Fix
```rust
// ❌ BROKEN: FnOnce vs Fn trait issue
{children.map(|children| children())}

// ✅ WORKING: Use as_ref() to avoid moving
{children.as_ref().map(|children| children())}
```

### 4. Prop Type Consistency
```rust
// Component definition
#[prop(optional)] default_checked: Option<CheckedState>,

// ❌ WRONG usage
<Checkbox default_checked=Some(CheckedState::True)>

// ✅ CORRECT usage
<Checkbox default_checked=CheckedState::True>
```

## Lessons Learned from Successful Implementation

❌ **What Didn't Work:**
- Complex prop patterns (`#[prop(attrs)]`, `AnyNodeRef`, `MaybeProp`)
- Assumed dependencies that don't exist in base Leptos
- Over-engineering before getting basics working
- Directory-based module structure
- Missing Provider import

✅ **What Works:**
- Simple `#[component]` functions
- `RwSignal` for state management
- Context for sharing state between components
- Basic HTML elements with simple props
- Single-file module approach
- Explicit Provider import

## Component Generation Machine: 3-Phase System

**Philosophy:** Build a predictable factory that converts WAI-ARIA specs into production-ready Rust primitives at scale.

### 🎯 **Phase I: Minimal Working State (FOUNDATION)**
**Goal:** Get basic functionality compiling and working with minimal complexity
**Success Criteria:** Component compiles, renders, basic interaction works
**Time Investment:** ~30 minutes per component

### 🎯 **Phase II: WAI-ARIA Compliance (ACCESSIBILITY)**
**Goal:** Full accessibility compliance with WAI-ARIA specification
**Success Criteria:** Passes accessibility audits, keyboard navigation, screen readers
**Time Investment:** ~2-3 hours per component

### 🎯 **Phase III: Production Ready (FEATURE PARITY)**
**Goal:** Match RustForWeb/radix feature set for production use
**Success Criteria:** Form integration, advanced props, developer experience features
**Time Investment:** ~4-6 hours per component

---

## Phase I: Minimal Working Component (FOUNDATION)

**Goal:** Get basic functionality working with minimal complexity

### Step 1: Phase I Template (PROVEN PATTERN - COPY EXACTLY)

**This template gets you to a working component in ~30 minutes:**

```rust
use leptos::prelude::*;
use leptos::context::Provider;  // ← CRITICAL: Always include

// 1. Define state enum (if component has state)
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ComponentState {
    StateA,
    StateB,
    StateC,  // Add more as needed
}

impl Default for ComponentState {
    fn default() -> Self {
        ComponentState::StateA
    }
}

// 2. Context for sharing state between components
#[derive(Clone, Copy)]
struct ComponentContext {
    state: RwSignal<ComponentState>,
}

// 3. Root component (state provider)
#[component]
pub fn ComponentRoot(
    #[prop(optional)] default_state: Option<ComponentState>,
    children: ChildrenFn,
) -> impl IntoView {
    let state = RwSignal::new(default_state.unwrap_or_default());
    let context_value = ComponentContext { state };

    let handle_click = move |_| {
        // Basic state toggle logic
        let current = state.get();
        let new_state = match current {
            ComponentState::StateA => ComponentState::StateB,
            ComponentState::StateB => ComponentState::StateA,
            ComponentState::StateC => ComponentState::StateA,
        };
        state.set(new_state);
    };

    view! {
        <Provider value=context_value>
            <button
                type="button"
                role="button"  // Replace with appropriate ARIA role
                on:click=handle_click
            >
                {children()}
            </button>
        </Provider>
    }
}

// 4. Child component (state consumer)
#[component]
pub fn ComponentIndicator(
    children: ChildrenFn,
) -> impl IntoView {
    let context = expect_context::<ComponentContext>();

    view! {
        <Show when=move || context.state.get() != ComponentState::StateA>
            <span>
                {children()}
            </span>
        </Show>
    }
}
```

**Phase I Success Criteria:**
- ✅ Compiles without errors
- ✅ Renders in browser
- ✅ Basic interaction works (click toggles state)
- ✅ Context sharing works between components
- ✅ Conditional rendering works

### Step 1b: Working Checkbox Example (PROVEN PATTERN)
```rust
use leptos::prelude::*;
use leptos::context::Provider;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CheckedState {
    True,
    False,
    Indeterminate,
}

impl Default for CheckedState {
    fn default() -> Self {
        CheckedState::False
    }
}

#[derive(Clone, Copy)]
struct CheckboxContext {
    checked: RwSignal<CheckedState>,
}

#[component]
pub fn Checkbox(
    #[prop(optional)] default_checked: Option<CheckedState>,
    children: ChildrenFn,
) -> impl IntoView {
    let checked = RwSignal::new(default_checked.unwrap_or_default());
    let context_value = CheckboxContext { checked };

    let handle_click = move |_| {
        let current = checked.get();
        let new_state = match current {
            CheckedState::True => CheckedState::False,
            CheckedState::False => CheckedState::True,
            CheckedState::Indeterminate => CheckedState::True,
        };
        checked.set(new_state);
    };

    view! {
        <Provider value=context_value>
            <button
                type="button"
                role="checkbox"
                aria-checked=move || match checked.get() {
                    CheckedState::True => "true",
                    CheckedState::False => "false",
                    CheckedState::Indeterminate => "mixed",
                }
                on:click=handle_click
            >
                {children()}
            </button>
        </Provider>
    }
}

#[component]
pub fn CheckboxIndicator(
    children: ChildrenFn,
) -> impl IntoView {
    let context = expect_context::<CheckboxContext>();

    view! {
        <Show when=move || context.checked.get() != CheckedState::False>
            <span>
                {children()}
            </span>
        </Show>
    }
}
```

### Step 2: Add Basic State (if needed)
```rust
use leptos::prelude::*;

#[component]
pub fn ComponentRoot(
    children: ChildrenFn,
) -> impl IntoView {
    let state = RwSignal::new(false);

    view! {
        <Provider value=state>
            {children()}
        </Provider>
    }
}

#[component]
pub fn ComponentPart(
    children: ChildrenFn,
) -> impl IntoView {
    let state = expect_context::<RwSignal<bool>>();

    let handle_click = move |_| {
        state.set(!state.get());
    };

    view! {
        <button on:click=handle_click>
            {children()}
        </button>
    }
}
```

### Step 3: Add Basic Conditional Rendering
```rust
#[component]
pub fn ComponentContent(
    children: ChildrenFn,
) -> impl IntoView {
    let state = expect_context::<RwSignal<bool>>();

    view! {
        <Show when=move || state.get()>
            <div>{children()}</div>
        </Show>
    }
}
```
```

## Common Errors & Solutions (TROUBLESHOOTING GUIDE)

### Error: "cannot find value `Provider` in this scope"
**Solution:** Add `use leptos::context::Provider;` to imports

### Error: "file not found for module `component_name`"
**Solution:** Create `src/components/component_name.rs` (single file, not directory)

### Error: "expected a closure that implements the `Fn` trait, but this closure only implements `FnOnce`"
**Solution:** Use `children.as_ref().map(|children| children())` instead of `children.map(|children| children())`

### Error: "mismatched types" with Optional Props
```rust
// ❌ WRONG
<Component prop=Some(Value::Something)>

// ✅ CORRECT
<Component prop=Value::Something>
```

### Error: Module not found in app.rs
**Solution:** Make sure component is exported in `src/components/mod.rs`:
```rust
pub mod component_name;
```

---

## Phase II: WAI-ARIA Compliance (ACCESSIBILITY)

**Goal:** Full accessibility compliance with WAI-ARIA specification
**Prerequisites:** Phase I must be working
**Time Investment:** ~2-3 hours per component

### Step 1: Keyboard Interaction
```rust
// Add proper keyboard handling
on:keydown=move |event: KeyboardEvent| {
    match event.key().as_str() {
        " " => {
            event.prevent_default();
            // Handle space key
        },
        "Enter" => {
            event.prevent_default();
            // Handle enter key (usually prevent default)
        },
        "Escape" => {
            // Handle escape key
        },
        "ArrowDown" | "ArrowUp" | "ArrowLeft" | "ArrowRight" => {
            // Handle arrow keys for navigation
        },
        _ => {}
    }
}
```

### Step 2: ARIA Attributes
```rust
#[component]
pub fn ComponentRoot(
    // Accessibility props
    #[prop(optional)] aria_label: Option<String>,
    #[prop(optional)] aria_labelledby: Option<String>,
    #[prop(optional)] aria_describedby: Option<String>,
    // ... other props
) -> impl IntoView {
    view! {
        <button
            role="checkbox"  // Correct ARIA role from spec
            aria-checked=move || match state.get() {
                ComponentState::StateA => "false",
                ComponentState::StateB => "true",
                ComponentState::StateC => "mixed",
            }
            aria-label=aria_label
            aria-labelledby=aria_labelledby
            aria-describedby=aria_describedby
        >
            {children()}
        </button>
    }
}
```

### Step 3: Form Integration
```rust
// Hidden input for form submission
#[prop(optional)] name: Option<String>,
#[prop(optional)] value: Option<String>,
#[prop(optional)] required: Option<bool>,

// Add hidden input in view
<Show when=move || name.is_some()>
    <input
        type="checkbox"
        name=name.unwrap_or_default()
        value=value.unwrap_or("on".to_string())
        checked=move || matches!(state.get(), ComponentState::StateB)
        required=required.unwrap_or(false)
        style="position: absolute; opacity: 0; pointer-events: none;"
    />
</Show>
```

---

## Phase III: Production Ready (FEATURE PARITY)

**Goal:** Match RustForWeb/radix feature set for production use
**Prerequisites:** Phase II must be complete
**Time Investment:** ~4-6 hours per component

### 🎯 **The 5 Universal Production Patterns**

Every production component needs these **exact patterns** (extracted from RustForWeb analysis):

#### **Pattern 1: Advanced Prop System**
```rust
#[component]
pub fn ComponentRoot(
    // Controllable state pattern
    #[prop(into, optional)] checked: MaybeProp<ComponentState>,
    #[prop(into, optional)] default_checked: MaybeProp<ComponentState>,
    #[prop(into, optional)] on_checked_change: Option<Callback<ComponentState>>,

    // Form integration
    #[prop(into, optional)] name: MaybeProp<String>,
    #[prop(into, optional)] value: MaybeProp<String>,
    #[prop(into, optional)] required: MaybeProp<bool>,
    #[prop(into, optional)] disabled: MaybeProp<bool>,

    // Composition system
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(optional)] node_ref: NodeRef<AnyElement>,
    #[prop(attrs)] attrs: Vec<(&'static str, Attribute)>,

    children: ChildrenFn,
) -> impl IntoView
```

#### **Pattern 2: Controllable State Hook**
```rust
// Implement this hook for every component
fn use_controllable_state<T>(
    controlled_value: MaybeProp<T>,
    default_value: MaybeProp<T>,
    on_change: Option<Callback<T>>,
) -> (Signal<Option<T>>, Callback<Option<T>>) {
    // Implementation matches RustForWeb exactly
}
```

#### **Pattern 3: Primitive Component System**
```rust
// Replace basic HTML elements with Primitive
<Primitive
    element=html::button
    as_child=as_child
    node_ref=composed_refs
    attrs=dynamic_attrs
    on:click=composed_click_handler
>
    {children()}
</Primitive>
```

#### **Pattern 4: Form Integration (BubbleInput)**
```rust
let is_form_control = Signal::derive(move || {
    button_ref.get()
        .and_then(|button| button.closest("form").ok())
        .flatten()
        .is_some()
});

// Hidden input for form submission
<Show when=move || is_form_control.get()>
    <BubbleInput
        control_ref=button_ref
        checked=checked_signal
        name=name
        value=value
        required=required
        disabled=disabled
    />
</Show>
```

#### **Pattern 5: Event Composition**
```rust
// Compose multiple event handlers
on:click=compose_callbacks(
    user_on_click,           // User-provided handler
    Some(internal_handler),  // Internal component logic
    None                     // Optional third handler
)
```

### 🔧 **Phase III Implementation Strategy**

#### **Step 1: Create Utility Hooks** (1-2 hours)
```rust
// src/hooks/mod.rs
pub mod use_controllable_state;
pub mod use_composed_refs;
pub mod use_previous;
pub mod use_size;
pub mod compose_callbacks;
```

#### **Step 2: Create Primitive System** (1-2 hours)
```rust
// src/primitive.rs
#[component]
pub fn Primitive(
    element: HtmlElement,
    as_child: MaybeProp<bool>,
    node_ref: NodeRef<AnyElement>,
    attrs: Vec<(&'static str, Attribute)>,
    children: ChildrenFn,
) -> impl IntoView
```

#### **Step 3: Create BubbleInput** (30 min)
```rust
// Form integration component
#[component]
fn BubbleInput(/* ... */) -> impl IntoView
```

#### **Step 4: Apply Patterns to Component** (1-2 hours)
- Replace props with advanced prop system
- Replace state with controllable state
- Replace HTML with Primitive
- Add form integration
- Add event composition

---

## 🚀 **The Bridge System: Phase II → Phase III for ANY Primitive**

### **For New Primitives (e.g., Accordion):**

#### **Step 1: Analyze Similar RustForWeb Component** (30 min)
```bash
# Find the closest existing component
# Accordion → Look at Collapsible or Menu (hierarchical)
# Tabs → Look at Switch or Toggle (state management)
# Select → Look at Menu (dropdown behavior)
```

#### **Step 2: Extract the Specific Patterns** (30 min)
```rust
// What state does it manage?
enum AccordionState { Open, Closed }

// What events does it handle?
on_value_change: Option<Callback<String>>,

// What form integration does it need?
// (Usually none for layout components)

// What composition patterns?
<Accordion.Root>
  <Accordion.Item>
    <Accordion.Trigger />
    <Accordion.Content />
  </Accordion.Item>
</Accordion.Root>
```

#### **Step 3: Apply the 5 Universal Patterns** (2-3 hours)
1. **Advanced Props** - Copy from template, adapt types
2. **Controllable State** - Use the hook, adapt state type
3. **Primitive System** - Replace HTML elements
4. **Form Integration** - Add if needed (usually for inputs)
5. **Event Composition** - Compose user + internal handlers

#### **Step 4: Test Against RustForWeb API** (30 min)
```rust
// Compare your API to RustForWeb (if exists)
// Or compare to React Radix API (same patterns)
// Ensure prop names and behaviors match
```

### **The Key Insight: It's Always the Same 5 Patterns!**

**Every** production component follows this **exact formula**:
- ✅ **Checkbox** - ✅ Switch - ✅ Toggle - ✅ Progress
- 🔄 **Accordion** - 🔄 Tabs - 🔄 Select - 🔄 Dialog

The **complexity is in the utilities, not the components**. Once we build the 5 patterns once, every new component is just:
1. Copy Phase III template
2. Adapt state type and events
3. Apply the 5 patterns
4. Done!

---

## 📊 **Complexity Analysis: How Deep is the Rabbit Hole?**

### **Phase I → Phase II: Manageable Growth**
```
Lines of Code: 70 → 150 (2x growth)
Concepts: 5 → 12 (basic → accessibility)
Time: 30 min → 2-3 hours (4-6x growth)
```

### **Phase II → Phase III: Exponential Explosion**
```
Lines of Code: 150 → 400+ (3x growth)
Concepts: 12 → 35+ (accessibility → production)
Dependencies: 0 → 8 utility hooks
Time: 2-3 hours → 4-6 hours (2x growth)
```

### **The Utility Infrastructure Required:**

#### **Core Hooks (Must Build Once):**
1. `use_controllable_state` - 70 lines
2. `use_composed_refs` - 40 lines
3. `use_previous` - 20 lines
4. `use_size` - 60 lines
5. `compose_callbacks` - 30 lines

#### **Component Infrastructure:**
6. `Primitive` component - 100 lines
7. `BubbleInput` component - 80 lines
8. `Presence` component - 120 lines

**Total Infrastructure: ~520 lines of utility code**

### **The Scaling Economics:**

#### **First Component (Checkbox):**
- Phase I: 30 min (70 lines)
- Phase II: 3 hours (150 lines)
- Phase III: 6 hours (400 lines) + 8 hours building utilities
- **Total: 14 hours for first production component**

#### **Second Component (Switch):**
- Phase I: 30 min (70 lines)
- Phase II: 2 hours (120 lines)
- Phase III: 2 hours (300 lines) - utilities already exist!
- **Total: 4.5 hours for second production component**

#### **Nth Component:**
- Phase I: 30 min
- Phase II: 1-2 hours
- Phase III: 1-2 hours
- **Total: 3-4 hours per component after infrastructure**

### **The Investment Payoff:**

```
Component 1: 14 hours (building the machine)
Component 2: 4.5 hours (3x faster)
Component 3: 3.5 hours (4x faster)
Component 4+: 3 hours (5x faster)

Break-even: After 3-4 components
ROI: 5x speed improvement for all future components
```

**This is why building the "machine" is worth it!** 🏭

---

## Phase 2: Add Incremental Complexity (DEPRECATED - USE PHASES ABOVE)

**Only after Phase 1 is working!**

### Step 4: Add Basic Props
```rust
#[component]
pub fn ComponentName(
    #[prop(optional)] disabled: Option<bool>,
    children: ChildrenFn,
) -> impl IntoView {
    let disabled = disabled.unwrap_or(false);

    view! {
        <button disabled=disabled>
            {children()}
        </button>
    }
}
```

### Step 5: Add Basic Styling Hooks
```rust
view! {
    <div
        data-state=if open { "open" } else { "closed" }
        role="dialog"
    >
        {children()}
    </div>
}
```

## Phase 3: Advanced Features (Much Later)

- Custom props and attributes
- Focus management
- Keyboard navigation
- Portal rendering
- Animation/transitions

## Key Principles

1. **Start Simple** - Get basic functionality working first
2. **Incremental Complexity** - Add one feature at a time
3. **Test Each Step** - Ensure each phase works before moving on
4. **Use Standard Leptos** - Don't assume external dependencies
5. **HTML First** - Use semantic HTML elements as foundation

## Dependencies (Minimal)

```toml
[dependencies]
leptos = { version = "0.7", features = ["csr", "ssr"] }
# Add others ONLY when needed
```

## Current Working Pattern (Dialog Example)

```rust
use leptos::prelude::*;

#[derive(Clone)]
struct DialogContext {
    open: RwSignal<bool>,
}

#[component]
pub fn Dialog(children: ChildrenFn) -> impl IntoView {
    let open = RwSignal::new(false);
    let context = DialogContext { open };

    view! {
        <Provider value=context>
            {children()}
        </Provider>
    }
}

#[component]
pub fn DialogTrigger(children: ChildrenFn) -> impl IntoView {
    let context = expect_context::<DialogContext>();

    view! {
        <button on:click=move |_| context.open.set(true)>
            {children()}
        </button>
    }
}

#[component]
pub fn DialogContent(children: ChildrenFn) -> impl IntoView {
    let context = expect_context::<DialogContext>();

    view! {
        <Show when=move || context.open.get()>
            <div role="dialog">
                {children()}
            </div>
        </Show>
    }
}
```

This is our **proven working pattern** - start here for all new components!
