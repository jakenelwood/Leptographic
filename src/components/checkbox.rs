use crate::hooks::{use_checkbox_state, CheckedState, UseCheckboxStateReturn};
use leptos::context::Provider;
use leptos::ev;
use leptos::prelude::*;

/// Helper function to generate checkbox CSS classes
fn get_checkbox_classes(user_class: String) -> String {
    let base = "relative inline-flex h-6 w-6 shrink-0 items-center justify-center rounded border-0 bg-white transition-all duration-200 ease-in-out shadow-sm";
    let focus = "focus:outline-none focus:ring-2 focus:ring-black focus:ring-offset-0";
    let states = "data-[state=checked]:bg-white data-[state=unchecked]:bg-white data-[state=indeterminate]:bg-white";
    let hover = "hover:bg-hover-purple data-[state=checked]:hover:bg-white data-[state=indeterminate]:hover:bg-white";
    let disabled = "data-[disabled]:opacity-50 data-[disabled]:cursor-not-allowed data-[disabled]:hover:bg-white";

    format!("{base} {focus} {states} {hover} {disabled} {user_class}")
}

/// Helper function to handle checkbox click events
fn handle_checkbox_click(disabled: bool, toggle: Callback<()>) {
    if !disabled {
        toggle.run(());
    }
}

/// Helper function to handle checkbox keyboard events
fn handle_checkbox_keydown(disabled: bool, toggle: Callback<()>, ev: ev::KeyboardEvent) {
    if disabled {
        return;
    }

    match ev.key().as_str() {
        " " | "Enter" => {
            ev.prevent_default();
            toggle.run(());
        }
        _ => {}
    }
}

/// Renders the checkbox view with all elements
#[component]
fn CheckboxView(
    checkbox_state: UseCheckboxStateReturn,
    is_disabled: Signal<bool>,
    is_required: Signal<bool>,
    final_id: Signal<String>,
    input_value: Signal<String>,
    name: MaybeProp<String>,
    form: MaybeProp<String>,
    class: MaybeProp<String>,
    children: ChildrenFn,
) -> impl IntoView {
    view! {
        <div class="relative inline-flex">
            // Hidden input for form integration (bubble input pattern)
            <input
                type="checkbox"
                name=move || name.get()
                value=move || input_value.get()
                form=move || form.get()
                checked=move || matches!(checkbox_state.checked.get(), CheckedState::True)
                required=move || is_required.get()
                disabled=move || is_disabled.get()
                // 🚨 TAILWIND CSS 4 ONLY - Hidden input styling
                class="absolute opacity-0 pointer-events-none"
                style="position: absolute; opacity: 0; pointer-events: none; margin: 0; width: 1px; height: 1px;"
                tabindex="-1"
            />

            <button
                id=move || final_id.get()
                type="button"
                role="checkbox"
                // ARIA attributes from our hook
                aria-checked=move || checkbox_state.get_aria_checked.get()
                aria-disabled=move || if is_disabled.get() { Some("true") } else { None }
                aria-required=move || if is_required.get() { Some("true") } else { None }
                // Data attributes for Tailwind CSS 4 styling
                data-state=move || checkbox_state.get_state_attr.get()
                data-disabled=move || if is_disabled.get() { Some("") } else { None }
                disabled=move || is_disabled.get()
                // 🚨 TAILWIND CSS 4 ONLY - Professional data-driven styling
                class=move || get_checkbox_classes(class.get().unwrap_or_default())
                on:click=move |_| handle_checkbox_click(is_disabled.get(), checkbox_state.toggle)
                on:keydown=move |ev: ev::KeyboardEvent| handle_checkbox_keydown(is_disabled.get(), checkbox_state.toggle, ev)
            >
                {children()}
            </button>
        </div>
    }
}

/// Context value shared between Checkbox and CheckboxIndicator
#[derive(Clone, Debug)]
pub struct CheckboxContextValue {
    pub state: Signal<CheckedState>,
    pub disabled: Signal<bool>,
}

/// Checkbox component - Hook-first implementation
///
/// Uses our proven hook library for state management and ARIA compliance.
/// Styled with Tailwind CSS 4 ONLY - no custom CSS allowed.
#[component]
pub fn Checkbox(
    // Core state management (from our hook library)
    #[prop(into, optional)] checked: MaybeProp<CheckedState>,
    #[prop(into, optional)] default_checked: MaybeProp<CheckedState>,
    #[prop(into, optional)] on_checked_change: Option<Callback<CheckedState>>,

    // Form integration
    #[prop(into, optional)] disabled: MaybeProp<bool>,
    #[prop(into, optional)] required: MaybeProp<bool>,
    #[prop(into, optional)] name: MaybeProp<String>,
    #[prop(into, optional)] value: MaybeProp<String>,
    #[prop(into, optional)] form: MaybeProp<String>,

    // Accessibility & DOM
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] class: MaybeProp<String>,

    children: ChildrenFn,
) -> impl IntoView {
    // Phase 0: Compose hooks - no manual state management!
    let checkbox_state = use_checkbox_state(checked, default_checked, on_checked_change);

    // Pre-compute common values to reduce complexity
    let is_disabled = Signal::derive(move || disabled.get().unwrap_or(false));
    let is_required = Signal::derive(move || required.get().unwrap_or(false));
    let final_id = Signal::derive(move || id.get().unwrap_or_else(|| "checkbox".to_string()));
    let input_value = Signal::derive(move || value.get().unwrap_or_else(|| "on".to_string()));

    // Context for child components
    let context_value = CheckboxContextValue {
        state: checkbox_state.checked,
        disabled: is_disabled,
    };

    view! {
        <Provider value=context_value>
            <CheckboxView
                checkbox_state=checkbox_state
                is_disabled=is_disabled
                is_required=is_required
                final_id=final_id
                input_value=input_value
                name=name
                form=form
                class=class
                children=children
            />
        </Provider>
    }
}

/// CheckboxIndicator - Shows when checkbox is checked or indeterminate
#[component]
pub fn CheckboxIndicator(
    #[prop(into, optional)] force_mount: MaybeProp<bool>,
    #[prop(into, optional)] class: MaybeProp<String>,
    children: ChildrenFn,
) -> impl IntoView {
    let context = expect_context::<CheckboxContextValue>();

    let is_present = Signal::derive(move || {
        let state = context.state.get();
        force_mount.get().unwrap_or(false)
            || state == CheckedState::True
            || state == CheckedState::Indeterminate
    });

    view! {
        <Show when=move || is_present.get()>
            <div
                data-state=move || match context.state.get() {
                    CheckedState::True => "checked",
                    CheckedState::False => "unchecked",
                    CheckedState::Indeterminate => "indeterminate",
                }
                // 🚨 TAILWIND CSS 4 ONLY - Professional indicator styling
                class=move || {
                    let base = "absolute inset-0 flex items-center justify-center text-white pointer-events-none";
                    let animation = "data-[state=checked]:animate-in data-[state=checked]:fade-in-0 data-[state=checked]:zoom-in-95 data-[state=unchecked]:animate-out data-[state=unchecked]:fade-out-0 data-[state=unchecked]:zoom-out-95";
                    let user = class.get().unwrap_or_default();
                    format!("{base} {animation} {user}")
                }
            >
                {children()}
            </div>
        </Show>
    }
}
