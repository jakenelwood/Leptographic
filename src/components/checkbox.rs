use leptos::prelude::*;
use leptos::context::Provider;
use leptos::html::AnyElement;

/// Represents the checked state of a checkbox
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

/// Simple controllable state hook - mimics RustForWeb's use_controllable_state
fn use_controllable_state(
    controlled_value: Option<CheckedState>,
    default_value: Option<CheckedState>,
    on_change: Option<Callback<CheckedState>>,
) -> (Signal<CheckedState>, WriteSignal<CheckedState>) {
    let internal_state = RwSignal::new(default_value.unwrap_or_default());

    let current_value = Signal::derive(move || {
        controlled_value.unwrap_or_else(|| internal_state.get())
    });

    let set_value = WriteSignal::derive(move |new_value: CheckedState| {
        if controlled_value.is_none() {
            internal_state.set(new_value);
        }
        if let Some(on_change) = on_change {
            on_change.call(new_value);
        }
    });

    (current_value, set_value)
}

/// Context for sharing checkbox state between components
#[derive(Clone, Copy)]
struct CheckboxContext {
    checked: RwSignal<CheckedState>,
    disabled: RwSignal<bool>,
}

/// Root Checkbox component that provides context for all checkbox parts
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
    /// Render as child element
    #[prop(optional)] as_child: Option<bool>,
    /// Node reference
    #[prop(optional)] node_ref: Option<NodeRef<AnyElement>>,
    /// Additional attributes
    #[prop(attrs)] attrs: Vec<(&'static str, Attribute)>,
    /// Child components
    children: ChildrenFn,
) -> impl IntoView {
    // Use controllable state pattern
    let (checked_signal, set_checked) = use_controllable_state(
        checked,
        default_checked,
        on_checked_change,
    );

    let disabled = RwSignal::new(disabled.unwrap_or(false));
    let required = required.unwrap_or(false);
    let value = value.unwrap_or_else(|| "on".to_string());
    let has_name = name.is_some();
    let name_value = name.unwrap_or_default();
    let as_child = as_child.unwrap_or(false);

    let context_value = CheckboxContext {
        checked: RwSignal::new(checked_signal.get_untracked()),
        disabled
    };

    // Keep context in sync with controllable state
    Effect::new(move |_| {
        context_value.checked.set(checked_signal.get());
    });

    let toggle_checked = move || {
        if disabled.get() {
            return; // Don't toggle if disabled
        }
        let current = checked_signal.get();
        let new_state = match current {
            CheckedState::True => CheckedState::False,
            CheckedState::False => CheckedState::True,
            CheckedState::Indeterminate => CheckedState::True,
        };
        set_checked.set(new_state);
    };

    let handle_click = move |_| {
        toggle_checked();
    };

    let handle_keydown = move |event: leptos::ev::KeyboardEvent| {
        match event.key().as_str() {
            " " => {
                event.prevent_default();
                toggle_checked();
            },
            "Enter" => {
                event.prevent_default(); // Prevent form submission
            },
            _ => {}
        }
    };

    // Build attributes dynamically
    let mut button_attrs = attrs.clone();
    button_attrs.extend([
        ("type", "button".into_attribute()),
        ("role", "checkbox".into_attribute()),
        ("aria-checked", (move || match checked_signal.get() {
            CheckedState::True => "true",
            CheckedState::False => "false",
            CheckedState::Indeterminate => "mixed",
        }).into_attribute()),
        ("aria-required", (move || if required { Some("true") } else { None }).into_attribute()),
        ("data-state", (move || match checked_signal.get() {
            CheckedState::True => "checked",
            CheckedState::False => "unchecked",
            CheckedState::Indeterminate => "indeterminate",
        }).into_attribute()),
        ("data-disabled", (move || disabled.get().then_some("")).into_attribute()),
        ("disabled", (move || disabled.get().then_some("")).into_attribute()),
        ("value", value.clone().into_attribute()),
    ]);

    // Add optional ARIA attributes
    if let Some(label) = aria_label {
        button_attrs.push(("aria-label", label.into_attribute()));
    }
    if let Some(labelledby) = aria_labelledby {
        button_attrs.push(("aria-labelledby", labelledby.into_attribute()));
    }
    if let Some(describedby) = aria_describedby {
        button_attrs.push(("aria-describedby", describedby.into_attribute()));
    }

    view! {
        <Provider value=context_value>
            {if as_child {
                // TODO: Implement as_child rendering (complex)
                view! { <div>"as_child not implemented yet"</div> }
            } else {
                view! {
                    <button
                        node_ref=node_ref.unwrap_or_default()
                        on:click=handle_click
                        on:keydown=handle_keydown
                        {..button_attrs}
                    >
                        {children()}
                    </button>
                }
            }}

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

/// Indicator element that shows the checkbox state
#[component]
pub fn CheckboxIndicator(
    /// Force mount even when unchecked
    #[prop(optional)] force_mount: Option<bool>,
    /// Render as child element
    #[prop(optional)] as_child: Option<bool>,
    /// Additional attributes
    #[prop(attrs)] attrs: Vec<(&'static str, Attribute)>,
    /// Child components (typically a checkmark icon)
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView {
    let context = expect_context::<CheckboxContext>();
    let force_mount = force_mount.unwrap_or(false);
    let as_child = as_child.unwrap_or(false);

    let should_show = move || {
        force_mount ||
        context.checked.get() == CheckedState::True ||
        context.checked.get() == CheckedState::Indeterminate
    };

    // Build attributes dynamically
    let mut span_attrs = attrs.clone();
    span_attrs.extend([
        ("data-state", (move || match context.checked.get() {
            CheckedState::True => "checked",
            CheckedState::False => "unchecked",
            CheckedState::Indeterminate => "indeterminate",
        }).into_attribute()),
        ("data-disabled", (move || context.disabled.get().then_some("")).into_attribute()),
        ("style", "pointer-events: none;".into_attribute()),
    ]);

    view! {
        // Presence component equivalent - simple Show for now
        <Show when=should_show>
            {if as_child {
                // TODO: Implement as_child rendering
                view! { <div>"as_child not implemented yet"</div> }
            } else {
                view! {
                    <span {..span_attrs}>
                        {children.as_ref().map(|children| children())}
                    </span>
                }
            }}
        </Show>
    }
}
