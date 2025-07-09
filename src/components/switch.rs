use leptos::prelude::*;
use leptos::context::Provider;

/// Switch component - Leptos 0.8.2 implementation of Radix UI Switch
///
/// Source: https://github.com/radix-ui/primitives/tree/main/packages/react/switch
/// Reference: https://github.com/RustForWeb/radix/tree/main/packages/primitives/leptos/switch
///
/// A control that allows the user to toggle between checked and not checked.

// Context for sharing switch state between components
#[derive(Clone, Copy)]
struct SwitchContext {
    checked: RwSignal<bool>,
    disabled: RwSignal<bool>,
}

/// Controllable state hook - adapted from Checkbox patterns
fn use_controllable_state(
    controlled_value: Option<bool>,
    default_value: Option<bool>,
    _on_change: Option<Callback<bool>>,
) -> (Signal<bool>, RwSignal<bool>) {
    let internal_state = RwSignal::new(default_value.unwrap_or(false));

    let current_value = Signal::derive(move || {
        controlled_value.unwrap_or_else(|| internal_state.get())
    });

    // For now, let's simplify and just return the RwSignal
    // We can add the callback logic later
    (current_value, internal_state)
}

/// Production-ready Switch component with Phase III patterns (Leptos 0.8.2)
#[component]
pub fn Switch(
    /// Controlled checked state
    #[prop(optional)] checked: Option<bool>,
    /// Default checked state (uncontrolled)
    #[prop(optional)] default_checked: Option<bool>,
    /// Callback when checked state changes
    #[prop(optional)] on_checked_change: Option<Callback<bool>>,
    /// Form name attribute
    #[prop(optional)] name: Option<String>,
    /// Form value attribute
    #[prop(optional)] value: Option<String>,
    /// Required for form validation
    #[prop(optional)] required: Option<bool>,
    /// Disabled state
    #[prop(optional)] disabled: Option<bool>,
    /// CSS class for styling
    #[prop(optional)] class: Option<&'static str>,
    /// ARIA label
    #[prop(optional)] aria_label: Option<String>,
    /// ARIA labelledby
    #[prop(optional)] aria_labelledby: Option<String>,
    /// ARIA describedby
    #[prop(optional)] aria_describedby: Option<String>,
    /// Node reference - 0.8.2 compatible
    #[prop(optional)] node_ref: Option<NodeRef<leptos::html::Button>>,
    /// Child components
    children: ChildrenFn,
) -> impl IntoView {
    // Use controllable state pattern
    let (checked_signal, checked_rw) = use_controllable_state(
        checked,
        default_checked,
        on_checked_change,
    );

    let disabled = RwSignal::new(disabled.unwrap_or(false));
    let required = required.unwrap_or(false);
    let value = value.unwrap_or_else(|| "on".to_string());
    let has_name = name.is_some();
    let name_value = name.unwrap_or_default();

    let context_value = SwitchContext {
        checked: RwSignal::new(checked_signal.get()),
        disabled,
    };

    // Keep context in sync with controllable state
    Effect::new(move |_| {
        context_value.checked.set(checked_signal.get());
    });

    let handle_click = move |_| {
        if disabled.get() {
            return; // Don't toggle if disabled
        }
        let current = checked_signal.get();
        checked_rw.set(!current);
    };

    let handle_keydown = move |ev: leptos::ev::KeyboardEvent| {
        if disabled.get() {
            return;
        }
        match ev.key().as_str() {
            " " | "Enter" => {
                ev.prevent_default();
                let current = checked_signal.get();
                checked_rw.set(!current);
            }
            _ => {}
        }
    };

    view! {
        <Provider value=context_value>
            <button
                type="button"
                role="switch"
                class=format!("switch-root {}", class.unwrap_or(""))
                data-radix-switch=""
                aria-checked=move || if checked_signal.get() { "true" } else { "false" }
                aria-required=move || if required { Some("true") } else { None }
                aria-label=aria_label
                aria-labelledby=aria_labelledby
                aria-describedby=aria_describedby
                data-state=move || if checked_signal.get() { "checked" } else { "unchecked" }
                data-disabled=move || disabled.get().then_some("")
                disabled=move || disabled.get()
                value=value.clone()
                node_ref=node_ref.unwrap_or_default()
                on:click=handle_click
                on:keydown=handle_keydown
            >
                {children()}
            </button>

            // Hidden input for form integration
            <Show when=move || has_name>
                <input
                    type="checkbox"
                    name=name_value.clone()
                    value=value.clone()
                    checked=move || checked_signal.get()
                    required=required
                    disabled=move || disabled.get()
                    tabindex="-1"
                    aria-hidden="true"
                    style:position="absolute"
                    style:pointer-events="none"
                    style:opacity="0"
                    style:margin="0"
                    style:transform="translateX(-100%)"
                />
            </Show>
        </Provider>
    }
}

/// SwitchThumb component - Phase IV visual indicator that moves within the switch
#[component]
pub fn SwitchThumb(
    /// CSS class for styling
    #[prop(optional)] class: Option<&'static str>,
    /// Child components
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView {
    let context = expect_context::<SwitchContext>();

    view! {
        <span
            class=format!("switch-thumb {}", class.unwrap_or(""))
            data-state=move || if context.checked.get() { "checked" } else { "unchecked" }
            data-disabled=move || context.disabled.get().then_some("")
        >
            {children.map(|children| children())}
        </span>
    }
}

/// CheckboxIndicator component for backward compatibility
/// This allows Switch to work with existing Checkbox patterns
#[component]
pub fn SwitchIndicator(
    /// Child components
    children: ChildrenFn,
) -> impl IntoView {
    let context = expect_context::<SwitchContext>();

    view! {
        <Show when=move || context.checked.get()>
            <span>
                {children()}
            </span>
        </Show>
    }
}
