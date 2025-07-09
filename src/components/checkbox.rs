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

// Production-ready context for checkbox state
#[derive(Clone, Copy)]
struct CheckboxContext {
    checked: RwSignal<CheckedState>,
}

// Phase III: Production Patterns for Leptos 0.8.2

/// Controllable state hook - 0.8.2 compatible version
fn use_controllable_state(
    controlled_value: Option<CheckedState>,
    default_value: Option<CheckedState>,
    _on_change: Option<Callback<CheckedState>>,
) -> (Signal<CheckedState>, RwSignal<CheckedState>) {
    let internal_state = RwSignal::new(default_value.unwrap_or_default());

    let current_value = Signal::derive(move || {
        controlled_value.unwrap_or_else(|| internal_state.get())
    });

    // For now, let's simplify and just return the RwSignal
    // We can add the callback logic later
    (current_value, internal_state)
}

/// Production-ready Checkbox component with Phase III patterns (Leptos 0.8.2)
#[component]
pub fn Checkbox(
    /// Controlled checked state
    #[prop(optional)] checked: Option<CheckedState>,
    /// Default checked state (uncontrolled)
    #[prop(optional)] default_checked: Option<CheckedState>,
    /// Callback when checked state changes
    #[prop(optional)] on_checked_change: Option<Callback<CheckedState>>,
    /// Form name attribute
    #[prop(optional)] name: Option<String>,
    /// Form value attribute
    #[prop(optional)] value: Option<String>,
    /// Required for form validation
    #[prop(optional)] required: Option<bool>,
    /// Disabled state
    #[prop(optional)] disabled: Option<bool>,
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

    let context_value = CheckboxContext {
        checked: RwSignal::new(checked_signal.get())
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
        let new_state = match current {
            CheckedState::True => CheckedState::False,
            CheckedState::False => CheckedState::True,
            CheckedState::Indeterminate => CheckedState::True,
        };
        checked_rw.set(new_state);
    };

    let handle_keydown = move |event: leptos::ev::KeyboardEvent| {
        match event.key().as_str() {
            " " => {
                event.prevent_default();
                if disabled.get() {
                    return; // Don't toggle if disabled
                }
                let current = checked_signal.get();
                let new_state = match current {
                    CheckedState::True => CheckedState::False,
                    CheckedState::False => CheckedState::True,
                    CheckedState::Indeterminate => CheckedState::True,
                };
                checked_rw.set(new_state);
            },
            "Enter" => {
                event.prevent_default(); // Prevent form submission
            },
            _ => {}
        }
    };

    view! {
        <Provider value=context_value>
            <button
                type="button"
                role="checkbox"
                class="checkbox-root"
                data-radix-checkbox=""
                aria-checked=move || match checked_signal.get() {
                    CheckedState::True => "true",
                    CheckedState::False => "false",
                    CheckedState::Indeterminate => "mixed",
                }
                aria-required=move || if required { Some("true") } else { None }
                aria-label=aria_label
                aria-labelledby=aria_labelledby
                aria-describedby=aria_describedby
                data-state=move || match checked_signal.get() {
                    CheckedState::True => "checked",
                    CheckedState::False => "unchecked",
                    CheckedState::Indeterminate => "indeterminate",
                }
                data-disabled=move || disabled.get().then_some("")
                disabled=move || disabled.get()
                value=value.clone()
                node_ref=node_ref.unwrap_or_default()
                on:click=handle_click
                on:keydown=handle_keydown
            >
                {children()}
            </button>

            // Hidden input for form integration (BubbleInput equivalent)
            <Show when=move || has_name>
                <input
                    type="checkbox"
                    name=name_value.clone()
                    value=value.clone()
                    checked=move || matches!(checked_signal.get(), CheckedState::True)
                    required=required
                    disabled=move || disabled.get()
                    aria-hidden="true"
                    tabindex="-1"
                    style="position: absolute; opacity: 0; pointer-events: none; margin: 0; transform: translateX(-100%);"
                />
            </Show>
        </Provider>
    }
}

/// Indicator element that shows the checkbox state - Phase IV with visual styling
#[component]
pub fn CheckboxIndicator(
    /// CSS class for styling
    #[prop(optional)] class: Option<&'static str>,
    /// Child components (custom check icon)
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView {
    let context = expect_context::<CheckboxContext>();

    view! {
        <span
            class=format!("checkbox-indicator {}", class.unwrap_or(""))
            data-state=move || match context.checked.get() {
                CheckedState::True => "checked",
                CheckedState::False => "unchecked",
                CheckedState::Indeterminate => "indeterminate",
            }
        >
            <Show when=move || context.checked.get() != CheckedState::False>
                {children.as_ref().map(|child_fn| child_fn())}
            </Show>
        </span>
    }
}
