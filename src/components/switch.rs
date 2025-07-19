use crate::hooks::{use_switch_state, UseSwitchStateReturn};
use leptos::context::Provider;
use leptos::ev;
use leptos::prelude::*;

/// Helper function to generate switch CSS classes
fn get_switch_classes(user_class: String) -> String {
    let base = "relative inline-flex h-6 w-11 shrink-0 items-center rounded-full border-0 transition-all duration-200 ease-in-out shadow-sm";
    let focus = "focus:outline-none focus:ring-2 focus:ring-black focus:ring-offset-0";
    let states = "data-[state=checked]:bg-black data-[state=unchecked]:bg-gray-300";
    let hover = "hover:bg-gray-400 data-[state=checked]:hover:bg-gray-800";
    let disabled = "data-[disabled]:opacity-50 data-[disabled]:cursor-not-allowed";

    format!("{base} {focus} {states} {hover} {disabled} {user_class}")
}

/// Helper function to generate switch thumb CSS classes
fn get_switch_thumb_classes(user_class: String) -> String {
    let base = "pointer-events-none block h-5 w-5 rounded-full bg-white shadow-lg ring-0 transition-transform duration-200 ease-in-out";
    let states = "data-[state=checked]:translate-x-5 data-[state=unchecked]:translate-x-0";

    format!("{base} {states} {user_class}")
}

/// Helper function to handle switch click events
fn handle_switch_click(disabled: bool, toggle: Callback<()>) {
    if !disabled {
        toggle.run(());
    }
}

/// Helper function to handle switch keyboard events
fn handle_switch_keydown(disabled: bool, toggle: Callback<()>, ev: ev::KeyboardEvent) {
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

/// Renders the switch view with all elements
#[component]
fn SwitchView(
    switch_state: UseSwitchStateReturn,
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
                checked=move || switch_state.checked.get()
                required=move || is_required.get()
                disabled=move || is_disabled.get()
                // Hidden input styling
                class="absolute opacity-0 pointer-events-none"
                style="position: absolute; opacity: 0; pointer-events: none; margin: 0; width: 1px; height: 1px;"
                tabindex="-1"
            />

            <button
                id=move || final_id.get()
                type="button"
                role="switch"
                // ARIA attributes from our hook
                aria-checked=move || switch_state.get_aria_checked.get()
                aria-disabled=move || if is_disabled.get() { Some("true") } else { None }
                aria-required=move || if is_required.get() { Some("true") } else { None }
                // Data attributes for Tailwind CSS styling
                data-state=move || switch_state.get_state_attr.get()
                data-disabled=move || if is_disabled.get() { Some("") } else { None }
                disabled=move || is_disabled.get()
                // Professional data-driven styling
                class=move || get_switch_classes(class.get().unwrap_or_default())
                on:click=move |_| handle_switch_click(is_disabled.get(), switch_state.toggle)
                on:keydown=move |ev: ev::KeyboardEvent| handle_switch_keydown(is_disabled.get(), switch_state.toggle, ev)
            >
                {children()}
            </button>
        </div>
    }
}

/// Context value shared between Switch and SwitchThumb
#[derive(Clone, Debug)]
pub struct SwitchContextValue {
    pub checked: Signal<bool>,
    pub disabled: Signal<bool>,
}

/// Switch component - Hook-first implementation
///
/// Uses our proven hook library for state management and ARIA compliance.
/// Styled with Tailwind CSS 4 ONLY - no custom CSS allowed.
#[component]
pub fn Switch(
    // Core state management (from our hook library)
    #[prop(into, optional)] checked: MaybeProp<bool>,
    #[prop(into, optional)] default_checked: MaybeProp<bool>,
    #[prop(into, optional)] on_checked_change: Option<Callback<bool>>,

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
    // Compose hooks - no manual state management!
    let switch_state = use_switch_state(checked, default_checked, on_checked_change);

    // Pre-compute common values to reduce complexity
    let is_disabled = Signal::derive(move || disabled.get().unwrap_or(false));
    let is_required = Signal::derive(move || required.get().unwrap_or(false));
    let final_id = Signal::derive(move || id.get().unwrap_or_else(|| "switch".to_string()));
    let input_value = Signal::derive(move || value.get().unwrap_or_else(|| "on".to_string()));

    // Context for child components
    let context_value = SwitchContextValue {
        checked: switch_state.checked,
        disabled: is_disabled,
    };

    view! {
        <Provider value=context_value>
            <SwitchView
                switch_state=switch_state
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

/// SwitchThumb - The movable thumb inside the switch
#[component]
pub fn SwitchThumb(#[prop(into, optional)] class: MaybeProp<String>) -> impl IntoView {
    let context = expect_context::<SwitchContextValue>();

    view! {
        <div
            data-state=move || if context.checked.get() { "checked" } else { "unchecked" }
            data-disabled=move || if context.disabled.get() { Some("") } else { None }
            // Professional thumb styling with smooth animation
            class=move || get_switch_thumb_classes(class.get().unwrap_or_default())
        />
    }
}
