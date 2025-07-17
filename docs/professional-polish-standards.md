# Professional Polish Standards

## 🎯 **The Final 10%**

> "Everything should be made as simple as possible, but no simpler" - Einstein

When a senior developer says a component isn't "very good (visually)," they're often not talking about the basic structure. They're talking about the subtle, high-effort details that create a "premium" feel. The AI is fantastic at the first 90%—the structure and basic styling. Your job as the Orchestrator is to guide it through that final 10% of polish.

## 🎨 **The Professional Polish Checklist**

### **1. Micro-Transitions & Animations**

This is often the biggest giveaway of an "AI-generated" feel. A static component might look right, but it feels dead.

- **Original:** Does the Radix component have a subtle `transition` on its `colors`, `transform`, or `box-shadow`? When you hover or focus, does the change happen instantly, or does it fade in over 150 milliseconds?
- **Your Component:** Add a `transition-colors duration-150` class (or similar) to your component's root element. This one change can make a component feel dramatically more professional.

**Radix Standard:**
- **Duration**: 120ms (not 150ms or 200ms)
- **Properties**: `box-shadow`, `background-color`, `border-color`
- **Timing**: `ease-out` for natural feel
- **Rule**: No instant changes - all state changes must be smoothly animated

### **2. Stateful Styling (`:hover`, `:focus`, `:disabled`)**

A component needs to visually communicate its state to the user.

- **Original:** Inspect the Radix component in your browser's dev tools. What styles are applied when you force the `:hover` or `:focus` state? What about `:disabled`?
- **Your Component:** Ensure your Tailwind classes include styles for these states. For example: `hover:bg-accent`, `focus-visible:ring-2`, `disabled:opacity-50 disabled:cursor-not-allowed`.

**Radix Patterns:**
- **Hover**: Subtle enhancements, not dramatic changes
- **Focus-visible**: 2px outline with proper offset (not focus)
- **Disabled**: Visual degradation without opacity (use specific colors)
- **Active**: Micro-timing (30ms) for touch feedback

### **3. Data-State Styling (`data-state="open"`)**

This is Radix's secret sauce for animations. Radix components add `data-state` attributes to elements when their state changes (e.g., a Dialog content gets `data-state="open"`). The CSS then uses these attributes as hooks for animations.

- **Original:** Look for CSS rules like `[data-state="open"] { ... }` in the `shadcn` CSS. This is how the "slide-in-from-top" animation works.
- **Your Component:** In your Leptos `view!` macro, add this attribute reactively: `data-state=move || if is_open.get() { "open" } else { "closed" }`. Then, ensure your Tailwind config can style based on these data attributes.

**Radix Patterns:**
- **Data Attributes**: `data-state`, `data-disabled`, `data-orientation`
- **Animation Hooks**: CSS targets `[data-state="checked"]` etc.
- **Transition Coordination**: State changes trigger smooth transitions
- **Compatibility**: Must match Radix UI data attribute patterns exactly

### **4. Pixel-Perfect Spacing & Typography**

This is about obsessive attention to detail.

- **Original:** Use your browser's dev tools to measure the exact `padding`, `margin`, `font-size`, `font-weight`, and `letter-spacing` of the original component.
- **Your Component:** Compare those values to your own. Is your padding `p-4` when it should be `p-6`? Is your title font `font-semibold` when it should be `font-bold`?

**Radix Measurements:**
- **Size-1**: 14px (calc(16px * 0.875))
- **Size-2**: 16px (default)
- **Size-3**: 20px (calc(16px * 1.25))
- **Border Radius**: Consistent scaling patterns
- **Indicators**: Proper ratios for visual balance

## 🤖 **AI Orchestrator Prompts**

### **Micro-Transitions Prompt**
```
Apply professional micro-transitions to this component following Radix UI patterns:

1. Use 120ms duration (Radix standard) for premium feel
2. Target box-shadow, background-color, border-color properties specifically
3. Use ease-out timing for natural feel
4. Ensure all state changes are smoothly animated (no instant changes)
5. Add transition CSS custom properties for easy customization

Example CSS pattern:
```css
--transition-duration: 120ms;
--transition-timing: ease-out;
--transition-properties: box-shadow, background-color, border-color;
transition: var(--transition-properties) var(--transition-duration) var(--transition-timing);
```
```

### **Stateful Styling Prompt**
```
Apply professional stateful styling to this component following Radix UI patterns:

1. Hover states: Subtle enhancements that don't distract
2. Focus-visible: Professional 2px outline with proper offset (not focus)
3. Disabled states: Proper cursor and visual degradation (avoid opacity)
4. Active states: 30ms micro-timing for touch feedback
5. State consistency: All states work together harmoniously

Ensure all interactive states are covered:
- :hover (subtle changes only)
- :focus-visible (not :focus)
- :disabled (cursor + visual degradation)
- :active (micro-timing)
- @media (pointer: coarse) for touch devices
```

### **Data-State Styling Prompt**
```
Implement professional data-state styling following Radix UI patterns:

1. Use data-state attributes for all state-based styling
2. Ensure CSS can target [data-state="checked"], [data-state="open"] etc.
3. Coordinate state changes with smooth transitions
4. Match Radix UI data attribute patterns exactly
5. Provide animation hooks for future enhancements

Required data attributes:
- data-state (primary state)
- data-disabled (disabled state)
- data-orientation (if applicable)
- data-accent-color (if applicable)
```

### **Pixel-Perfect Sizing Prompt**
```
Update this component's sizing to match Radix UI pixel-perfect measurements:

1. Implement size-1, size-2, size-3 variants using Radix calculations
2. Use CSS calc() functions for precise measurements
3. Ensure proper ratios between container and indicators
4. Match border-radius scaling patterns exactly
5. Verify measurements against Radix UI source code

Radix size patterns:
- size-1: calc(base * 0.875)
- size-2: base (default)
- size-3: calc(base * 1.25)
- Border radius: Consistent scaling with size
```

## 🎯 **Quality Gates**

No component can be marked as "Complete" until:

- [ ] **All 4 Professional Polish areas** are 100% complete
- [ ] **Micro-transitions** feel smooth and natural (120ms, ease-out)
- [ ] **Stateful styling** covers all interactive states professionally
- [ ] **Data-state styling** matches Radix patterns exactly
- [ ] **Pixel-perfect sizing** matches Radix measurements precisely
- [ ] **Visual regression test** confirms indistinguishable from Radix UI
- [ ] **Accessibility audit** confirms no degradation from polish changes

## 🚀 **Impact**

This Professional Polish phase transforms components from functional to premium. It's the difference between:

**Before Professional Polish:**
- Functional but feels "AI-generated"
- Basic transitions and states
- Approximate measurements
- Good enough for development

**After Professional Polish:**
- Indistinguishable from Radix UI
- Premium micro-interactions
- Pixel-perfect precision
- Ready for production use

The Professional Polish Checklist ensures every component that comes off THE MACHINE has that final 10% of polish that separates good components from truly professional ones.
