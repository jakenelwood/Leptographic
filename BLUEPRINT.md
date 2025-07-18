# 🏗️ **Leptographic Hook-First Blueprint**

> **Technical Reference** for building Leptos 0.8.3 components using our proven hook library

**🎯 For Complete Workflow**: See [`master-workflow.md`](./master-workflow.md) - the single source of truth for component generation.

**🎯 This Document**: Hook patterns, code examples, and technical reference for Augment Code.

## 🚀 **Hook-First Architecture: The Revolution**

Instead of building components from scratch, we **compose proven hooks** to create production-ready primitives in minutes, not hours.

### **The Power of Composition:**
```rust
// Before: 50+ lines of complex state management
#[component]
pub fn Switch(/* ... */) -> impl IntoView {
    // Complex controllable state logic...
    // Manual ARIA attribute generation...
    // Form integration boilerplate...
    // Event handling complexity...
}

// After: 5 lines using our hook library
#[component] 
pub fn Switch(/* ... */) -> impl IntoView {
    let switch_state = use_switch_state(checked, default_checked, on_checked_change);
    let switch_id = use_id_with_prefix("switch");
    
    view! {
        <button
            id=switch_id
            role="switch"
            aria-checked=move || switch_state.get_aria_checked.get()
            data-state=move || switch_state.get_state_attr.get()
            on:click=move |_| switch_state.toggle.run(())
        >
            {children()}
        </button>
    }
}
```

## 🎯 **Available Hook Library**

Our production-ready hooks eliminate the need for manual state management:

### **Layer 1: Core Utilities**
```rust
use leptos_radix_ui::hooks::{
    // Universal state management
    use_controllable_state,     // ✅ Controlled/uncontrolled pattern
    
    // Utilities
    use_id_generator,           // ✅ Unique IDs for accessibility
    use_related_ids,            // ✅ Related ID sets (trigger, content, etc.)
    use_escape_key,             // ✅ Handle escape key presses
    use_previous,               // ✅ Track previous values for animations
    
    // TODO: Fix NodeRef issues
    // use_outside_click,       // 🚧 Detect clicks outside elements
    // use_focus_trap,          // 🚧 Trap focus within element
    // use_composed_refs,       // 🚧 Combine multiple refs
};
```

### **Layer 2: Component-Specific**
```rust
use leptos_radix_ui::hooks::{
    // Ready to use
    use_checkbox_state,         // ✅ Complete checkbox state management
    use_switch_state,           // ✅ Complete switch state management
    
    // TODO: Implement
    // use_radio_group_state,   // 📋 Radio group with keyboard nav
    // use_slider_state,        // 📋 Slider with range and steps
    // use_progress_state,      // 📋 Progress with indeterminate
};
```

### **Layer 3: Behavior Hooks (Coming Soon)**
```rust
// TODO: Implement complex interaction patterns
// use_tooltip_behavior,       // 📋 Hover/focus with delays
// use_dialog_behavior,        // 📋 Modal with focus trap
// use_dropdown_behavior,      // 📋 Positioning and keyboard nav
```

## 🤖 **For Augment Code: Hook-First Development**

When you receive a prompt from `master-workflow.md`:

1. **Always use existing hooks first** - Check our hook library before creating custom state management
2. **Follow hook composition patterns** - Combine hooks rather than writing complex logic
3. **Reference the hook examples** below for correct API usage
4. **Use proper Leptos 0.8.3 patterns** - `.run()` for callbacks, `Memo` for computed values
5. **Include proper ARIA attributes** - Our hooks provide these automatically
6. **🚨 TAILWIND CSS 4 ONLY** - No custom CSS, no external stylesheets, only Tailwind utilities

## 📋 **Component Status**

| Component | Hook Available | Status |
|-----------|----------------|--------|
| Checkbox  | ✅ `use_checkbox_state` | Production Ready |
| Switch    | ✅ `use_switch_state` | Production Ready |
| Radio Group | 📋 TODO | Needs Implementation |
| Slider    | 📋 TODO | Needs Implementation |
| Dialog    | 📋 TODO | Needs Behavior Hook |
| Tooltip   | 📋 TODO | Needs Behavior Hook |

## 🎯 **Hook-First Component Patterns**

### **Pattern 1: Use Existing Hooks (Recommended)**
```rust
#[component]
pub fn Switch(
    #[prop(into, optional)] checked: MaybeProp<bool>,
    #[prop(into, optional)] default_checked: MaybeProp<bool>,
    #[prop(into, optional)] on_checked_change: Option<Callback<bool>>,
    #[prop(into, optional)] disabled: MaybeProp<bool>,
    children: ChildrenFn,
) -> impl IntoView {
    // Compose existing hooks - no manual state management!
    let switch_state = use_switch_state(checked, default_checked, on_checked_change);
    let switch_id = use_id_with_prefix("switch");
    
    // Context for child components
    let context_value = SwitchContextValue {
        checked: switch_state.checked,
        disabled: disabled.into(),
    };
    
    view! {
        <Provider value=context_value>
            <button
                id=switch_id
                type="button"
                role="switch"
                aria-checked=move || switch_state.get_aria_checked.get()
                data-state=move || switch_state.get_state_attr.get()
                data-disabled=move || disabled.get().then_some("")
                disabled=move || disabled.get().unwrap_or(false)
                on:click=move |_| switch_state.toggle.run(())
            >
                {children()}
            </button>
        </Provider>
    }
}
```

### **Pattern 2: Custom Hook Creation (When Needed)**
```rust
// Only create custom hooks when existing ones don't fit
pub fn use_custom_component_state(
    // Component-specific parameters
) -> UseCustomComponentStateReturn {
    // Use use_controllable_state as foundation
    let base_state = use_controllable_state(/* ... */);
    
    // Add component-specific logic
    let custom_behavior = move || {
        // Component-specific state transformations
    };
    
    UseCustomComponentStateReturn {
        // Expose clean API
    }
}
```

## 🔧 **Critical Leptos 0.8.3 Patterns (VALIDATED ✅)**

### **Required Dependencies**
```toml
[dependencies]
leptos = { version = "0.8.3", features = ["csr", "ssr"] }
leptos_meta = { version = "0.8.3", default-features = false }
leptos_router = { version = "0.8.3", default-features = false }
web-sys = "0.3"
wasm-bindgen = "0.2"
```

### **Hook Composition Pattern (Recommended)**
```rust
#[component]
pub fn MyComponent(
    #[prop(into, optional)] checked: MaybeProp<bool>,
    #[prop(into, optional)] default_checked: MaybeProp<bool>,
    #[prop(into, optional)] on_checked_change: Option<Callback<bool>>,
    children: ChildrenFn,
) -> impl IntoView {
    // Use our proven hooks instead of manual state management
    let state = use_switch_state(checked, default_checked, on_checked_change);
    let ids = use_related_ids("switch");
    
    view! {
        <button
            id=ids.trigger_id
            role="switch"
            aria-checked=move || state.get_aria_checked.get()
            data-state=move || state.get_state_attr.get()
            on:click=move |_| state.toggle.run(())
        >
            {children()}
        </button>
    }
}
```

### **Context Pattern (Simplified)**
```rust
// Use hooks to power context values
#[derive(Clone, Debug)]
struct SwitchContextValue {
    checked: Signal<bool>,
    disabled: Signal<bool>,
}

#[component]
pub fn Switch(/* ... */) -> impl IntoView {
    let switch_state = use_switch_state(checked, default_checked, on_checked_change);
    
    let context_value = SwitchContextValue {
        checked: switch_state.checked,
        disabled: disabled.into(),
    };
    
    view! {
        <Provider value=context_value>
            {children()}
        </Provider>
    }
}

// Child components use context
#[component]
pub fn SwitchThumb() -> impl IntoView {
    let context = expect_context::<SwitchContextValue>();
    
    view! {
        <span data-state=move || if context.checked.get() { "checked" } else { "unchecked" }>
            // Thumb content
        </span>
    }
}
```

### **Styling Pattern (Tailwind CSS 4 ONLY)**
```rust
// 🎯 CRITICAL: ONLY Tailwind CSS 4 - NO custom CSS, NO external stylesheets
// Hooks provide styling attributes automatically for data-driven styling
view! {
    <button
        // Hooks handle all the complexity
        aria-checked=move || switch_state.get_aria_checked.get()
        data-state=move || switch_state.get_state_attr.get()
        data-disabled=move || disabled.get().then_some("")

        // ✅ TAILWIND CSS 4 ONLY - Data-driven utility classes
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
    >
        {children()}
    </button>
}
```

### **🚨 STYLING REQUIREMENTS - TAILWIND CSS 4 ONLY**

**✅ ALLOWED:**
- Tailwind CSS 4 utility classes ONLY
- Data-driven selectors: `data-[state=checked]:`, `data-[disabled]:`
- Responsive prefixes: `sm:`, `md:`, `lg:`
- State variants: `hover:`, `focus:`, `active:`
- Animation utilities: `transition-*`, `duration-*`, `ease-*`

**❌ FORBIDDEN:**
- Custom CSS files or `<style>` tags
- External CSS libraries (Bootstrap, Bulma, etc.)
- CSS-in-JS or styled components
- Inline styles with `style=` attribute
- CSS custom properties/variables (unless Tailwind-generated)
- Any non-Tailwind CSS

## 🎯 **Essential Patterns Reference**

### **State Enum Pattern**
```rust
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Default)]
pub enum CheckedState {
    #[default]
    False,
    True,
    Indeterminate,
}
```

### **ARIA Attributes (Handled by Hooks)**
```rust
// Our hooks provide these automatically
aria-checked=move || switch_state.get_aria_checked.get()  // "true" | "false" | "mixed"
aria-disabled=move || disabled.get().then_some("true")
data-state=move || switch_state.get_state_attr.get()     // "checked" | "unchecked"
```

### **Form Integration (Handled by Hooks)**
```rust
// Hidden input for form submission (hooks handle this)
view! {
    <input
        type="hidden"
        name=move || name.get()
        value=move || switch_state.get_form_value.get()  // "on" | ""
    />
}
```

---

## 🚀 **The Revolution: From Complex to Simple**

### **Before (Manual Implementation):**
- ❌ 50+ lines of state management per component
- ❌ Manual ARIA attribute generation
- ❌ Complex form integration logic
- ❌ Repetitive event handling
- ❌ Inconsistent patterns across components

### **After (Hook-First Architecture):**
- ✅ 5-10 lines using proven hooks
- ✅ Automatic ARIA compliance
- ✅ Built-in form integration
- ✅ Consistent event handling
- ✅ Reusable, tested patterns

### **Impact:**
**10x faster development** • **Consistent quality** • **Zero boilerplate** • **Production-ready**

---

## 🎓 **Production Deployment Learnings**

### **Responsive Design Best Practices (Validated ✅)**

From our successful leptographic.com deployment, these patterns ensure bulletproof responsive behavior:

#### **Protected Navigation Pattern**
```rust
// ✅ PROVEN: Fixed sidebar + flexible content
view! {
    <div class="flex min-h-screen">
        // Navigation: Fixed width, never collapses
        <div class="w-48 flex-shrink-0 p-2">
            // Navigation content
        </div>

        // Content: Flexible, responsive grid
        <div class="flex-1 grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-3">
            // Component cards
        </div>
    </div>
}
```

#### **Component Card Sizing Strategy**
```rust
// ✅ PROVEN: Responsive card sizing with optimal spacing
view! {
    <div class="w-5/6 h-40 sm:h-44 lg:h-48 mx-auto">
        // 83.33% width provides perfect balance:
        // - Substantial enough for content
        // - 17% breathing room for elegance
        // - Responsive height scaling
    </div>
}
```

#### **Text Scaling Hierarchy**
```rust
// ✅ PROVEN: 30% size increases for accessibility
// Card titles: text-sm sm:text-base (30% larger)
// Body text: text-base (30% larger than text-sm)
// Navigation: text-sm (30% larger than text-xs)
// Component sizes: h-6 w-6 (30% larger than h-5 w-5)
```

### **Theme System Anti-Patterns (Learned from Production)**

#### **❌ AVOID: Theme-dependent container backgrounds**
```rust
// This causes flashing during theme transitions
<div class=move || format!("{}",
    match theme.get() {
        Theme::Light => "bg-white",
        Theme::Dark => "bg-dark-bg",
    }
)>
```

#### **✅ SOLUTION: Transparent containers with themed content**
```rust
// Containers are transparent, only content has theme colors
<div>  // No background - prevents flashing
    <ComponentCard theme=theme>  // Theme handled internally
        // Content with stable backgrounds
    </ComponentCard>
</div>
```

### **Component Sizing Best Practices**

#### **Interactive Element Sizing**
```rust
// ✅ PROVEN: 30% larger for better accessibility
// Checkbox: h-6 w-6 (was h-5 w-5)
// Icons: width="19" height="19" (was 15x15)
// Focus rings: ring-2 (maintained for visibility)
```

#### **Icon Scaling Strategy**
```rust
// ✅ PROVEN: 33% reduction for refined appearance
// Theme toggle: text-lg (was text-2xl)
// Maintains functionality while reducing visual dominance
```

### **Production Deployment Architecture**

#### **Server Configuration (Validated ✅)**
```nginx
# SSL + Security Headers + Performance
server {
    listen 443 ssl http2;
    ssl_certificate /etc/letsencrypt/live/domain/fullchain.pem;

    # Security headers prevent XSS, clickjacking
    add_header Strict-Transport-Security "max-age=31536000";
    add_header X-Frame-Options DENY;

    # Performance: Gzip + Caching
    gzip on;
    location /pkg/ {
        expires 1y;
        add_header Cache-Control "public, immutable";
    }

    # Proxy to Leptos app
    location / {
        proxy_pass http://127.0.0.1:3000;
        proxy_set_header X-Forwarded-Proto $scheme;
    }
}
```

#### **Systemd Service Pattern**
```ini
[Unit]
Description=Leptos UI System
After=network.target

[Service]
Type=simple
User=www-data
WorkingDirectory=/var/www/app
ExecStart=/var/www/app/leptos-app
Environment=LEPTOS_SITE_ADDR=127.0.0.1:3000
Restart=always

[Install]
WantedBy=multi-user.target
```

### **Performance Optimizations (Production-Tested)**

#### **Asset Optimization**
- ✅ Wasm-release profile with `opt-level = 'z'`
- ✅ Static asset caching (1 year)
- ✅ Gzip compression for all text assets
- ✅ HTTP/2 for multiplexed requests

#### **Responsive Breakpoint Strategy**
```rust
// ✅ PROVEN: Mobile-first responsive grid
// Mobile: grid-cols-1 (single column)
// Small: sm:grid-cols-2 (two columns)
// Large: lg:grid-cols-3 (three columns)
// XL: xl:grid-cols-4 (four columns)
```

---

**🎯 This blueprint provides everything needed to build production-ready Leptos primitives using our hook-first approach. For complete workflow, see [`master-workflow.md`](./master-workflow.md).**
