[ROLE]
You are an expert Accessibility Specialist and Technical Analyst. Your task is to read the official WAI-ARIA Authoring Practices Guide (APG) and extract the complete technical requirements for a given UI pattern. You must be precise, thorough, and structure your output in clean Markdown.

[CONTEXT]
I am building a library of headless, accessible UI components in Leptos (a Rust web framework). This process begins by creating a `BLUEPRINT.md` file for each component. This blueprint serves as the definitive specification for all subsequent AI-driven code generation. The accuracy and completeness of this document are critical.

[SOURCE OF TRUTH]
Your ONLY source of information is the official WAI-ARIA Authoring Practices Guide (APG). Do not use any other tutorials, blogs, or knowledge.
- Main APG Page: <https://www.w3.org/WAI/ARIA/apg/patterns/>
- The specific pattern you must analyze is the "{{WAI-ARIA Pattern Name}}".

[TASK]
Read the APG page for the "{{WAI-ARIA Pattern Name}}" and generate a detailed blueprint by extracting the following information:
1.  **Keyboard Interactions:** List ALL required keyboard behaviors for ALL component parts. Be specific about keys (e.g., `Tab`, `Shift + Tab`, `Escape`, `Enter`, `Space`, `Arrow Keys`).
2.  **WAI-ARIA Roles, States, and Properties:** Detail EVERY `role`, `aria-*` attribute, state, and property required. Specify which element each attribute belongs to (e.g., "The trigger element," "The content panel element").
3.  **Focus Management:** Describe all requirements related to focus. This includes focus trapping, initial focus placement when an element opens, and where focus returns to when it closes.
4.  **DOM Structure & Behavior Notes:** Mention any specific recommendations about DOM structure (e.g., the need for a portal), element relationships (e.g., ownership via `aria-controls`), or behavioral logic (e.g., closing on an outside click).

[OUTPUT FORMAT]
Present the extracted information in the following Markdown format. Do not add any conversational text outside of this structure.

#### WAI-ARIA Pattern: {{WAI-ARIA Pattern Name}}
---
**1. Keyboard Interactions**
*   **For the [Component Part Name, e.g., Trigger]:**
    *   `Key`: [Description of behavior].
*   **For the [Component Part Name, e.g., Content Panel]:**
    *   `Key`: [Description of behavior].

**2. WAI-ARIA Roles, States, & Properties**
*   **[Component Part Name, e.g., Trigger Element]:**
    *   `role`: [e.g., `button`]
    *   `aria-haspopup`: [e.g., `dialog`]
    *   `aria-expanded`: [e.g., `true` when open, `false` when closed]
*   **[Component Part Name, e.g., Content Element]:**
    *   `role`: [e.g., `dialog`]
    *   `aria-modal`: [e.g., `true`]
    *   `aria-labelledby`: [e.g., "Must point to the ID of the dialog's title element."]
    *   `aria-describedby`: [e.g., "Must point to the ID of the dialog's description element."]

**3. Focus Management**
*   [e.g., "When the dialog opens, focus must be set on the first focusable element inside the dialog content."]
*   [e.g., "Focus must be trapped within the dialog while it is open."]
*   [e.g., "When the dialog closes, focus must return to the element that triggered it."]

**4. DOM Structure & Behavior Notes**
*   [e.g., "The dialog content should be rendered in a top-level portal to avoid z-index and stacking context issues."]
*   [e.g., "The dialog should close when a user clicks outside of the dialog content area."]



What you are building is a form of **Model-Driven Development (MDD)**, where an abstract model (your `blueprint.md`) is automatically transformed into executable code [esystems.fi](https://www.esystems.fi/en/blog/automated-code-generation-what-it-is-and-its-impact-on-development). This is an advanced and incredibly powerful approach.

Let's break down the implications of this statement.

### The "Heavy Lifting": Designing the Code Generation Factory

Your work is focused on building a robust, predictable, and resilient "factory." The key design pillars of this factory are:

1. **The Blueprint Engine:** The system that ingests a high-level goal (e.g., "Dialog") and reliably produces a perfect, machine-readable specification (`blueprint.md`). This is your requirements gathering phase.
2. **The Code Generation Core (The Assembly Line):** The series of prompt templates and AI calls that translate the blueprint into its constituent parts—composable utilities, context providers, and consumer components. This is your core manufacturing process.
3. **The Verification & QA Loop (The Quality Control):** The automated system that lints the blueprint, runs `cargo check` on the output, and uses the "Auditor AI" to cross-reference the generated code against the specification. It's crucial to include a feedback loop where compiler errors from Rust (which are famously excellent) can be automatically fed back into the LLM to self-correct [ghuntley.com](https://ghuntley.com/specs/).
4. **The Orchestration Pipeline (The Factory Manager):** The master script that runs the entire process in the correct order for each primitive. It takes a single input (`Dialog`) and outputs a complete, verified, and ready-to-review Pull Request. A project on GitHub demonstrates this principle by using separate scripts to first generate the architecture and then the main program [github.com](https://github.com/BruceRayWilson/PBL_StockPredictions).

---

### Rust Radix Status
https://radix.rustforweb.org/introduction.html#frameworks

### List of primatives
Accessible Icon
Accordion
Alert Dialog
Arrow
Aspect Ratio
Avatar
Checkbox
Collapsible
Collection
Compose Refs
Context Menu
Context
Dialog
Direction
Dismissable Layer
Dropdown Menu
Focus Guards
Focus Scope
Form
Hover Card
ID
Label
Menu
Menubar
Navigation Menu
Popover
Popper
Portal
Presence
Primitive
Progress
Radio Group
Roving Focus
Scroll Area
Select
Separator
Slider
Slot
Switch
Tabs
Toast
Toggle Group
Toggle
Toolbar
Tooltip
Use Callback Ref
Use Controllable State
Use Escape Keydown
Use Layout Effect
Use Previous
Use Rect
Use Size
Visually Hidden

### List of existing primatives
https://github.com/RustForWeb/radix/tree/main/packages/primitives/leptos

Reviewing the work already completed in the `RustForWeb/radix` repository is like finding the Rosetta Stone for your project.

You are moving from a *theoretical* plan (based on the WAI-ARIA spec) to a *practical, proven* one (based on working Leptos code). By reverse-engineering the existing primitives, you can extract a definitive, repeatable architectural pattern that has already been validated. This is the most effective way to ensure consistency and quality across all the components you generate.


### Step-by-Step Analysis of an Existing Primitive

**Goal:** Deconstruct `packages/primitives/leptos/menu` to define a "Leptos Primitive Recipe".

### 1. Analyze the Directory and File Structure

The first pattern you'll notice is the standardized file organization.

- `src/lib.rs`: This is the public entry point for the crate (e.g., `leptos-radix-menu`). It uses `pub mod` to expose the component parts.
- `src/menu.rs` (or similar name): This is the core implementation file. This is where the magic happens.
- `examples/`: Contains a runnable example showing how to use the component, which is crucial for understanding its public API and intended use.

**Repeatable Pattern:** Every new primitive will have a similar crate structure.

### 2. Deconstruct the Core Implementation File (`menu.rs`)

Open this file and identify its key anatomical sections. You will find a highly consistent structure:

- **Imports:** Standard `use leptos::*;` and imports from other Radix primitives like `Popper`, `Collection`, and `RovingFocusGroup`.
- **Context Definition:** A `struct` defining the shared state. This is the heart of the "headless" pattern. The `menu.rs` file actually uses several nested contexts for different levels of the component tree.

    ```rust
    // Example from menu.rs
    #[derive(Clone)]
    struct MenuRootContextValue {
        is_using_keyboard: Signal<bool>,
        dir: Signal<Direction>,
        modal: Signal<bool>,
        on_close: Callback<()>,
    }
    ```
    **Pattern:** The `Context` struct is always `Clone`. It uses `Signal` or `RwSignal` for state and `Callback` for actions that children can invoke [docs.rs](https://docs.rs/leptos/latest/leptos/). This decouples child components from the parent's implementation.

- **The "Root" Component (`Menu`):**
    - This component is the state provider.
    - It uses signals like `create_rw_signal` to initialize state (`is_using_keyboard`).
    - It uses the `<Provider>` component (the JSX version of `provide_context`) to make `MenuRootContextValue` available to all children.
    - It renders its `children`.
    **Pattern:** The `Root` component is the stateful parent that sets up the shared context.

- **The "Part" Components (`MenuItem`, `MenuContent`, `MenuSeparator`):**
    - Each part is a `#[component]`.
    - They almost always start by calling `expect_context::<...>()` (a safe-unwrapping version of `use_context`).
    - They use the signals and callbacks from the context to read state and perform actions. For example, `MenuItem` calls the `on_close` callback from the root context to close the menu.
    - They render their own `children` or compose with other primitives.
    - Some parts, like `MenuContent`, provide their own **nested context** (`MenuContentContextValue`) for their children, enabling more complex, localized interactions like typeahead search.
    **Pattern:** Part components are consumers of the context. They are generally stateless themselves, deriving their behavior from the shared `Root` state.

### 3. Trace the State and Logic Flow

- **State:** Notice how the primary state (like whether the menu is `open`) lives in the `Menu` root's context. `MenuContentImpl` doesn't have its own `is_open` state; it derives its `data-state` attribute by reading the context: `data-state=(move || get_open_state(context.open.get()))`. This is the essence of fine-grained reactivity.
- **Logic:** Child components like `MenuItem` don't change state directly. They call a `Callback` (e.g., `root_context.on_close.call(())`) provided by the `Root`. This keeps the update logic centralized and predictable, preventing a tangled web of state updates.

### 4. Identify Composable Utilities

Look for standalone helper functions. The `menu.rs` file is full of them, demonstrating how to isolate complex logic into pure, testable functions that could be shared across a library.

- `get_next_match(...)`: A pure function that contains all the logic for the typeahead search feature.
- `when_mouse(...)`: A higher-order function that wraps an event handler, making it run only for mouse pointer events.
- `is_point_in_polygon(...)`: A geometric utility function used for complex pointer-tracking logic for submenus.
- `get_open_state(...)`: A simple formatter that converts a boolean to a "open" or "closed" string state for the `data-state` attribute.

---

### The Synthesized "Leptos Primitive Recipe"

After this analysis, you can define a concrete recipe. This recipe is the "style guide" you will enforce in your AI prompts.

1. **Crate Structure:** Each primitive gets its own crate (e.g., `leptos-radix-dialog`).
2. **Core File (`<name>.rs`):**
    - Define a `Clone`able `Context` struct holding `RwSignal`s and `Callback`s.
    - Create a `Root` component that initializes state and `provide_context`.
    - Create `Part` components (`Trigger`, `Content`, etc.) that `use_context` to function.
    - Pass properties and merge classes correctly using `#[prop(into)]` and `leptos_classes`.
3. **Reactivity Model:**
    - Centralize state in the `Root` component.
    - Use derived signals in child components (`move || { ... }`).
    - Use `Callback`s for child-to-parent communication.
4. **API Design:**
    - Expose component parts through a public module in `lib.rs`.
    - Provide a clear, working example in the `/examples` directory.

### How This Supercharges Your AI Automation

You now have a perfect "one-shot" or "few-shot" learning example for your AI. Your prompts can be dramatically improved:

**Old Prompt (Conceptual):**

> "Create a Leptos component for a Dialog using context..."
> 

**New Prompt (Example-Driven and Precise):**

> [ROLE]
You are an expert Leptos developer specializing in creating headless UI primitives.
> 
> 
> **[TASK]**
> Create the full implementation for a headless "Dialog" component in Leptos. You must follow the exact architectural pattern, coding style, and file structure demonstrated in the "Accordion" component provided below.
> 
> **[INPUTS]**
> 
> 1. **The Blueprint:** (Paste the `blueprint.md` for Dialog). This defines the *what*.
> 2. **The Reference Implementation:** (Paste the complete source code for `leptos-radix-accordion`). This defines the *how*.
> 
> **[OUTPUT]**
> Produce the Rust code for a `leptos-radix-dialog` crate, including the `lib.rs` and `dialog.rs` files, structured exactly like the accordion example.
> 

### Example in Action: Generating the Dialog Blueprint

1. Create an empty file: `src/dialog/blueprint.md`.
2. Open it and press `Cmd/Ctrl+I`.
3. Paste the template into the instruction box, replacing the placeholders:
    - `{{WAI-ARIA Pattern Name}}` becomes `Dialog (Modal)`
