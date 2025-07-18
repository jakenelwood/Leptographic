# 🎯 **MASTER WORKFLOW: Leptographic Component Factory**

> **Single Source of Truth** for creating production-ready Leptos 0.8.3 components from Radix React primitives using our **Hook-First Architecture**

## 🚨 **CRITICAL STYLING POLICY: TAILWIND CSS 4 ONLY**

**✅ ALLOWED:**
- Tailwind CSS 4 utility classes ONLY
- Data-driven selectors: `data-[state=checked]:`, `data-[disabled]:`
- Responsive/state variants: `sm:`, `hover:`, `focus:`

**❌ FORBIDDEN:**
- Custom CSS files or `<style>` tags
- External CSS libraries (Bootstrap, Bulma, etc.)
- Inline styles with `style=` attribute
- Any non-Tailwind CSS

**🎯 This policy will be enforced during every component build and review.**

### **Enforcement & Validation**
- **During Development**: Check every `class=` attribute contains only Tailwind utilities
- **Code Review**: Verify no `style=` attributes or custom CSS files
- **Testing**: Ensure all styling works through data attributes only
- **Documentation**: All examples must use Tailwind CSS 4 only

## 🚀 **Simplified 3-Phase Process**

> **Philosophy**: Build components by composing our proven hook library, not by writing complex state management from scratch.

### **Phase 0: Hook Selection (5-15 minutes)**
**Goal**: Choose and configure the right hooks for your component

**Quick Decision Tree:**
```
Does component need state management?
├─ YES → Use existing hooks from our library:
│   ├─ Checkbox/Switch → use_checkbox_state / use_switch_state ✅
│   ├─ Radio Group → use_radio_group_state (TODO)
│   ├─ Slider → use_slider_state (TODO)
│   └─ Custom → use_controllable_state + component logic
│
├─ Does component need interactions?
│   ├─ Escape key → use_escape_key ✅
│   ├─ Outside click → use_outside_click (TODO)
│   ├─ Focus trap → use_focus_trap (TODO)
│   └─ Tooltip behavior → use_tooltip_behavior (TODO)
│
└─ Does component need IDs?
    └─ YES → use_id_generator / use_related_ids ✅
```

**Augment Code Prompt:**
```
# Phase 0: Hook Selection - {COMPONENT_NAME}

## Research
@octocode Search Radix UI "{COMPONENT_NAME}" React implementation
@context7 Get WAI-ARIA patterns for "{COMPONENT_NAME}"

## Hook Selection
Based on component needs, select from our hook library:

**State Management:**
- [ ] use_controllable_state (universal pattern)
- [ ] use_{component}_state (if exists)
- [ ] Custom state hook needed?

**Interactions:**
- [ ] use_escape_key (close on escape)
- [ ] use_outside_click (close on outside click)
- [ ] use_focus_trap (modal/dialog focus)

**Utilities:**
- [ ] use_id_generator (accessibility IDs)
- [ ] use_previous (animations/transitions)

## Output
List of hooks to use and their configuration.
```

### **Phase I: Component Composition (15-30 minutes)**
**Goal**: Compose hooks into a working component with proper ARIA

**Augment Code Prompt:**
```
# Phase I: Component Composition - {COMPONENT_NAME}

## Hook Integration
Using hooks selected in Phase 0, create component:

**Template:**
```rust
#[component]
pub fn {COMPONENT_NAME}(
    // Props based on selected hooks
    #[prop(into, optional)] checked: MaybeProp<T>,
    #[prop(into, optional)] default_checked: MaybeProp<T>,
    #[prop(into, optional)] on_checked_change: Option<Callback<T>>,
    #[prop(into, optional)] disabled: MaybeProp<bool>,
    children: ChildrenFn,
) -> impl IntoView {
    // Phase 0: Compose hooks
    let state = use_{component}_state(checked, default_checked, on_checked_change);
    let ids = use_related_ids("{component}");
    let escape_key = use_escape_key(Callback::new(move |_| /* close logic */));

    // Context for child components
    let context_value = {COMPONENT_NAME}ContextValue {
        state: state.checked,
        disabled: disabled.into(),
    };

    view! {
        <Provider value=context_value>
            <button
                id=ids.trigger_id
                role="{aria_role}"
                aria-checked=move || state.get_aria_checked.get()
                data-state=move || state.get_state_attr.get()
                data-disabled=move || disabled.get().then_some("")
                disabled=move || disabled.get().unwrap_or(false)
                on:click=move |_| state.toggle.run(())
            >
                {children()}
            </button>
        </Provider>
    }
}
```

## Success Criteria:
- [ ] Component compiles and renders
- [ ] Hooks provide all necessary functionality
- [ ] ARIA attributes are correct
- [ ] Context system works for child components
- [ ] Basic interactions work (click, keyboard)
```

### **Phase II: Polish & Production (15-30 minutes)**
**Goal**: Add Tailwind CSS 4 styling, form integration, and final polish

**🚨 CRITICAL**: Use ONLY Tailwind CSS 4 utility classes. NO custom CSS allowed.

**Augment Code Prompt:**
```
# Phase II: Polish & Production - {COMPONENT_NAME}

## 🚨 STYLING INTEGRATION - TAILWIND CSS 4 ONLY
**CRITICAL**: Use ONLY Tailwind CSS 4 utility classes. NO custom CSS allowed.

```rust
// ✅ CORRECT: Tailwind CSS 4 data-driven styling
class="
    relative inline-flex h-5 w-5 items-center justify-center
    rounded border-2 border-gray-300 bg-white
    data-[state=checked]:border-blue-500 data-[state=checked]:bg-blue-500
    data-[state=unchecked]:border-gray-300 data-[state=unchecked]:bg-white
    data-[disabled]:opacity-50 data-[disabled]:cursor-not-allowed
    focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2
    transition-colors duration-150 ease-in-out
    hover:border-gray-400 data-[state=checked]:hover:bg-blue-600
"

// ❌ FORBIDDEN: Custom CSS, inline styles, external libraries
// style="custom: styles"  // NO
// class="custom-class"     // NO (unless Tailwind)
```

## Form Integration
Add hidden input for form submission:

```rust
// Hidden input for form submission
view! {
    <input
        type="hidden"
        name=move || name.get()
        value=move || state.get_form_value.get()
        form=move || form.get()
    />
}
```

## Child Components
Create indicator/content child components using context:

```rust
#[component]
pub fn {COMPONENT_NAME}Indicator(children: ChildrenFn) -> impl IntoView {
    let context = expect_context::<{COMPONENT_NAME}ContextValue>();

    view! {
        <Show when=move || context.state.get() == SomeState::Active>
            <div data-state=move || /* context-driven state */>
                {children()}
            </div>
        </Show>
    }
}
```

## Success Criteria:
- [ ] 🚨 TAILWIND CSS 4 ONLY - No custom CSS anywhere
- [ ] Styling responds to all states using data-[state=*]: selectors
- [ ] Form integration works
- [ ] Child components use context correctly
- [ ] Component is production-ready
```
```

**Quality Gate**: `cargo fmt && cargo clippy && cargo test`

---

## 🤖 **Automation: One-Command Component Generation**
Use our enhanced automation script:

```bash
# Generate complete component with hook-first approach
./scripts/blueprintautomate.sh switch

# Generate specific phase only
./scripts/blueprintautomate.sh switch 0    # Hook selection
./scripts/blueprintautomate.sh switch I    # Component composition
./scripts/blueprintautomate.sh switch II   # Polish & production
```

### **Hook Library Integration**
Our automation now leverages our proven hook library:

```rust
// Available hooks for immediate use:
use leptos_radix_ui::hooks::{
    // State management
    use_controllable_state,
    use_checkbox_state,
    use_switch_state,

    // Utilities
    use_id_generator,
    use_related_ids,
    use_escape_key,
    use_previous,
};
```

### **Quality Pipeline**
```bash
# After each phase
cargo fmt && cargo clippy && cargo test

# Full quality check
cargo audit && cargo machete && cargo doc
```

---

## 🎯 **Why This Approach Works**

### **Before (Complex 6-Phase System):**
- ❌ 6+ hours per component
- ❌ Complex state management from scratch
- ❌ Repetitive ARIA implementation
- ❌ Manual form integration
- ❌ Inconsistent patterns

### **After (Hook-First 3-Phase System):**
- ✅ 1-2 hours per component
- ✅ Compose proven hooks
- ✅ Built-in ARIA support
- ✅ Automatic form integration
- ✅ Consistent, tested patterns

### **The Secret Sauce:**
Our hook library transforms component development from **complex manual work** into **simple composition**. Instead of writing 50+ lines of state management, you write 5 lines using our hooks.

---

## 📚 **Documentation Architecture**

### **Single Source of Truth:**
- **`master-workflow.md`** (THIS FILE) - Complete workflow and automation
- **`BLUEPRINT.md`** - Technical patterns and hook examples
- **`scripts/blueprintautomate.sh`** - One-command automation

### **Eliminated Complexity:**
- ❌ Removed redundant documentation files
- ❌ Simplified from 6 phases to 3 phases
- ❌ Eliminated manual state management patterns
- ✅ Focus on hook composition and automation

**🎯 Everything you need is in these three files. No more complexity, just results.** 🚀
