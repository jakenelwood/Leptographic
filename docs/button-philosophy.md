# The Button Question: A Philosophical Deep Dive 🤔

## TL;DR: Why We Don't Have a Button Primitive

**Radix UI intentionally does not provide a Button primitive, and neither do we.** This isn't an oversight—it's a fundamental design philosophy that shapes the entire library.

## 🔍 The Discovery

During our Button research phase, we discovered:

1. **Radix UI has no Button primitive** - [GitHub Issue #892](https://github.com/radix-ui/primitives/issues/892) was "Closed as not planned"
2. **RustForWeb mirrors this decision** - No Button in their Leptos port either
3. **This is intentional philosophy** - Not a missing feature

## 🎯 The Radix Philosophy

### **Solve Hard Problems, Not Easy Ones**

Radix Primitives exist to handle **complex behavioral and accessibility patterns** that are:
- Difficult to implement correctly
- Error-prone when done from scratch  
- Require sophisticated state management
- Need complex ARIA compliance

Examples of "hard problems":
- **Dialog**: Focus trapping, escape handling, backdrop clicks, scroll locking
- **Select**: Keyboard navigation, typeahead, option virtualization
- **Tooltip**: Positioning, hover delays, focus management

### **The Button is Already Perfect**

The native HTML `<button>` element provides:
- ✅ **Focusable** by default
- ✅ **Keyboard-activatable** with `Enter` and `Space`
- ✅ **Correct ARIA role** (`role="button"`) for screen readers
- ✅ **Form integration** built-in
- ✅ **Event handling** standardized

**There's no complex behavior to abstract!**

### **Styling ≠ Behavioral Primitives**

What people usually want from a "Button component":
- Variants (`primary`, `secondary`, `destructive`)
- Sizes (`small`, `medium`, `large`)
- Loading states with spinners
- Icon positioning
- Custom styling

**These are all styling concerns, not behavioral primitives.** Radix is intentionally unstyled and focuses purely on behavior.

## 🛠️ The Leptos Pattern: Custom Button + `as_child`

Instead of providing a Button primitive, we enable the **composition pattern**:

### 1. User Creates Custom Styled Button

```rust
// In the user's application (NOT in our library)
#[component]
fn StyledButton(
    children: ChildrenFn,
    #[prop(attrs)] attrs: Vec<(&'static str, Attribute)>,
    #[prop(optional)] variant: Option<ButtonVariant>,
) -> impl IntoView {
    let css_class = match variant.unwrap_or(ButtonVariant::Primary) {
        ButtonVariant::Primary => "bg-blue-500 hover:bg-blue-700 text-white",
        ButtonVariant::Secondary => "bg-gray-500 hover:bg-gray-700 text-white",
        ButtonVariant::Destructive => "bg-red-500 hover:bg-red-700 text-white",
    };
    
    view! {
        <button 
            class=format!("font-bold py-2 px-4 rounded {}", css_class)
            attrs=attrs  // Receives ARIA attributes and handlers from primitives
        >
            {children()}
        </button>
    }
}
```

### 2. User Integrates with Our Primitives via `as_child`

```rust
#[component]
fn App() -> impl IntoView {
    view! {
        <Dialog>
            <DialogTrigger as_child=true>
                {/* Our DialogTrigger merges its behavior onto the user's button */}
                <StyledButton variant=ButtonVariant::Primary>
                    "Open Dialog"
                </StyledButton>
            </DialogTrigger>
            <DialogContent>
                "This dialog was opened by a custom styled button!"
            </DialogContent>
        </Dialog>
    }
}
```

### 3. The Magic: Behavioral Merging

When `as_child=true`, our `DialogTrigger` component:
- **Doesn't render its own `<button>`**
- **Merges its props** (onClick, ARIA attributes) onto the user's `StyledButton`
- **Preserves the user's styling** while adding our behavior

## 🎨 Benefits of This Approach

### **For Library Authors (Us)**
- ✅ **Focused scope**: We only handle complex behavioral patterns
- ✅ **Unopinionated**: No styling decisions forced on users
- ✅ **Maintainable**: Less code, fewer edge cases
- ✅ **True to Radix**: Authentic port of the philosophy

### **For Library Users**
- ✅ **Full control**: Complete ownership of button styling
- ✅ **Consistent branding**: Buttons match their design system
- ✅ **Flexible**: Can create any button variant they need
- ✅ **Composable**: Works seamlessly with our behavioral primitives

### **For the Ecosystem**
- ✅ **Clear separation**: Behavioral libraries vs. styling libraries
- ✅ **Interoperable**: Works with any CSS framework (Tailwind, styled-components, etc.)
- ✅ **Sustainable**: Reduces scope creep and feature requests

## 🚫 What We Avoid

By not providing a Button primitive, we avoid:
- ❌ **Scope creep**: "Can you add loading states? Icons? Variants?"
- ❌ **Styling opinions**: "Your button styles don't match our design system"
- ❌ **Maintenance burden**: Styling bugs, browser inconsistencies
- ❌ **Philosophy violations**: Mixing behavioral and visual concerns

## 📚 References

- [Radix UI Issue #892](https://github.com/radix-ui/primitives/issues/892) - Button primitive request (closed)
- [Community Discussion #1560](https://github.com/radix-ui/primitives/discussions/1560) - Why no Button?
- [Radix Styling Guide](https://radix-ui.com/primitives/docs/guides/styling) - "You are in control"
- [RustForWeb Radix](https://github.com/RustForWeb/radix) - Also has no Button primitive

## 🎯 Our Decision

**We will mirror Radix philosophy and NOT provide a Button primitive.**

This decision:
- ✅ Keeps our library focused on hard problems
- ✅ Maintains philosophical consistency with Radix UI
- ✅ Enables powerful composition patterns via `as_child`
- ✅ Respects the separation between behavior and styling

**Next up: Switch component** - a perfect example of a component that DOES need behavioral primitives (controlled/uncontrolled state, ARIA switch role, keyboard handling).
