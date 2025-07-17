use leptos::context::Provider;
use leptos::prelude::*;
use leptos::*;
use std::fmt::{Display, Formatter};

/// Represents the checked state of a checkbox
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum CheckedState {
    False,
    True,
    Indeterminate,
}

impl Display for CheckedState {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                CheckedState::False => "false",
                CheckedState::True => "true",
                CheckedState::Indeterminate => "indeterminate",
            }
        )
    }
}

// For now, we'll use the string representation directly in attributes

/// Context value shared between Checkbox and CheckboxIndicator
#[derive(Clone, Debug)]
struct CheckboxContextValue {
    state: Signal<CheckedState>,
    disabled: Signal<bool>,
}

/// Main Checkbox component - Phase II: Production Features
#[component]
pub fn Checkbox(
    /// Unique identifier for the checkbox
    #[prop(into, optional)]
    id: MaybeProp<String>,
    /// Controlled checked state
    #[prop(into, optional)]
    checked: MaybeProp<CheckedState>,
    /// Default checked state for uncontrolled usage
    #[prop(into, optional)]
    default_checked: MaybeProp<CheckedState>,
    /// Callback fired when checked state changes
    #[prop(into, optional)]
    on_checked_change: Option<Callback<CheckedState>>,
    /// Custom value for form submission (Phase III: Advanced value handling)
    #[prop(into, optional)]
    value: MaybeProp<String>,
    /// Callback fired when value changes (Phase III: Advanced composition)
    #[prop(into, optional)]
    on_value_change: Option<Callback<String>>,
    /// Whether the checkbox is disabled
    #[prop(into, optional)]
    disabled: MaybeProp<bool>,
    /// Whether the checkbox is required in forms
    #[prop(into, optional)]
    required: MaybeProp<bool>,
    /// Name attribute for form submission
    #[prop(into, optional)]
    name: MaybeProp<String>,
    /// Form attribute - associates checkbox with a form by ID
    #[prop(into, optional)]
    form: MaybeProp<String>,
    /// Additional CSS classes
    #[prop(into, optional)]
    class: MaybeProp<String>,
    /// Render as child element (composition pattern)
    #[prop(into, optional)]
    as_child: MaybeProp<bool>,
    /// Node reference for DOM access
    #[prop(into, optional)]
    node_ref: MaybeProp<NodeRef<html::Button>>,
    /// Child components (typically CheckboxIndicator)
    children: ChildrenFn,
) -> impl IntoView {
    // Convert props to signals
    let disabled = Signal::derive(move || disabled.get().unwrap_or(false));
    let required = Signal::derive(move || required.get().unwrap_or(false));
    let value = Signal::derive(move || value.get().unwrap_or("on".into()));
    let name = Signal::derive(move || name.get());
    let id = Signal::derive(move || id.get());
    let form = Signal::derive(move || form.get());
    let _class = Signal::derive(move || class.get().unwrap_or_default());
    let _as_child = Signal::derive(move || as_child.get().unwrap_or(false));

    // Handle node ref
    let internal_node_ref = NodeRef::<html::Button>::new();
    let final_node_ref = node_ref.get().unwrap_or(internal_node_ref);

    // Debug: Component mounting
    leptos::logging::log!("🚀 Checkbox component mounting with id: {:?}", id.get());

    // Controllable state management
    let (internal_checked, set_internal_checked) = signal(
        checked
            .get()
            .or(default_checked.get())
            .unwrap_or(CheckedState::False),
    );

    let current_checked =
        Signal::derive(move || checked.get().unwrap_or_else(|| internal_checked.get()));

    // Phase III: Advanced value handling - Transform state to external value
    let external_value = Memo::new(move |_| {
        let base_value = if value.get().is_empty() {
            "on".to_string()
        } else {
            value.get()
        };
        match current_checked.get() {
            CheckedState::True => base_value,
            CheckedState::False => "".to_string(),
            CheckedState::Indeterminate => "mixed".to_string(),
        }
    });

    // Phase III: Performance optimization - Memoize interactive state
    let _is_interactive = Memo::new(move |_| !disabled.get());

    leptos::logging::log!("📊 Initial state: {:?}", current_checked.get());

    // Enhanced change handler with error boundaries
    let handle_change = move |new_state: CheckedState| {
        // Update internal state if uncontrolled
        if checked.get().is_none() {
            set_internal_checked.set(new_state);
        }

        // Call state change callback with error handling
        if let Some(callback) = on_checked_change {
            // Wrap callback in error boundary
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                callback.run(new_state);
            }))
            .unwrap_or_else(|_| {
                leptos::logging::error!("Error in checkbox state callback");
            });
        }

        // Phase III: Call value change callback with transformed value
        if let Some(value_callback) = on_value_change {
            let new_value = match new_state {
                CheckedState::True => {
                    if value.get().is_empty() {
                        "on".to_string()
                    } else {
                        value.get()
                    }
                }
                CheckedState::False => "".to_string(),
                CheckedState::Indeterminate => "mixed".to_string(),
            };

            // Wrap value callback in error boundary
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                value_callback.run(new_value);
            }))
            .unwrap_or_else(|_| {
                leptos::logging::error!("Error in checkbox value callback");
            });
        }
    };

    // Handle click events
    let handle_click = move |ev: ev::MouseEvent| {
        if disabled.get() {
            return;
        }

        ev.prevent_default();
        ev.stop_propagation();

        let current = current_checked.get();
        let new_state = match current {
            CheckedState::False => CheckedState::True,
            CheckedState::True => CheckedState::False,
            CheckedState::Indeterminate => CheckedState::True,
        };

        handle_change(new_state);
    };

    // Enhanced keyboard event handling
    let handle_keydown = move |ev: ev::KeyboardEvent| {
        if disabled.get() {
            return;
        }

        match ev.key().as_str() {
            " " | "Enter" => {
                ev.prevent_default();
                let current = current_checked.get();
                let new_state = match current {
                    CheckedState::False => CheckedState::True,
                    CheckedState::True => CheckedState::False,
                    CheckedState::Indeterminate => CheckedState::True,
                };
                handle_change(new_state);
            }
            _ => {}
        }
    };

    // Form reset handling
    Effect::new(move |_| {
        if let Some(node) = final_node_ref.get() {
            let form_element = node.form();
            if let Some(_form) = form_element {
                // Listen for form reset events
                let _reset_handler = move |_: web_sys::Event| {
                    let default_state = default_checked.get().unwrap_or(CheckedState::False);
                    if checked.get().is_none() {
                        set_internal_checked.set(default_state);
                    }
                };

                // Note: In a real implementation, we'd properly manage event listeners
                // This is a simplified version for Phase II
            }
        }
    });

    // Create context for child components
    let context_value = CheckboxContextValue {
        state: current_checked,
        disabled,
    };

    // Debug: Log context state changes
    Effect::new(move |_| {
        leptos::logging::log!("🔄 Context state changed to: {:?}", current_checked.get());
    });

    // Get state string for data attributes
    let get_state = move || match current_checked.get() {
        CheckedState::True => "checked",
        CheckedState::False => "unchecked",
        CheckedState::Indeterminate => "indeterminate",
    };

    // Get aria-checked value
    let get_aria_checked = move || match current_checked.get() {
        CheckedState::True => "true",
        CheckedState::False => "false",
        CheckedState::Indeterminate => "mixed",
    };

    view! {
        <Provider value=context_value>
            <div class="relative inline-flex">
                // Hidden input for form integration (bubble input pattern)
                <input
                    type="checkbox"
                    name=name
                    value=move || external_value.get() // Phase III: Use advanced value handling
                    form=form
                    checked=move || matches!(current_checked.get(), CheckedState::True)
                    required=required
                    disabled=disabled
                    class="absolute opacity-0 pointer-events-none m-0 w-px h-px"
                    tabindex="-1"
                />

                <button
                    type="button"
                    role="checkbox"
                    id=id
                    node_ref=final_node_ref
                    class="leptonic-checkbox relative inline-flex items-center justify-center w-6 h-6 rounded bg-white hover:bg-gray-200 disabled:opacity-50 disabled:cursor-not-allowed appearance-none cursor-pointer focus:shadow-[0_0_0_2px_theme(colors.black)]"
                    aria-checked=get_aria_checked
                    aria-required=move || if required.get() { "true" } else { "false" }
                    data-state=get_state
                    data-disabled=move || disabled.get().then_some("")
                    disabled=disabled
                    on:click=handle_click
                    on:keydown=handle_keydown
                >
                    {children()}
                </button>
            </div>
        </Provider>
    }
}

/// CheckboxIndicator component - Phase III: Advanced Features
#[component]
pub fn CheckboxIndicator(
    /// Force mount the indicator even when unchecked
    #[prop(into, optional)]
    force_mount: MaybeProp<bool>,
    /// Additional CSS classes
    #[prop(into, optional)]
    class: MaybeProp<String>,
    /// Render as child element (composition pattern)
    #[prop(into, optional)]
    _as_child: MaybeProp<bool>,
    /// Node reference for DOM access
    #[prop(into, optional)]
    _node_ref: MaybeProp<NodeRef<html::Div>>,
    /// Child content (typically an icon)
    children: ChildrenFn,
) -> impl IntoView {
    let force_mount = Signal::derive(move || force_mount.get().unwrap_or(false));
    let class = Signal::derive(move || class.get().unwrap_or_default());

    // Get context from parent Checkbox
    let context = expect_context::<CheckboxContextValue>();

    // Debug: Log indicator context state
    Effect::new(move |_| {
        leptos::logging::log!("📍 Indicator sees context state: {:?}", context.state.get());
    });

    // Phase III: Determine if indicator should be present (checked OR indeterminate)
    let is_present = Signal::derive(move || {
        let state = context.state.get();
        let should_show = force_mount.get()
            || state == CheckedState::True
            || state == CheckedState::Indeterminate;
        leptos::logging::log!(
            "👁️ Indicator is_present: {}, state: {:?}",
            should_show,
            state
        );
        should_show
    });

    // Get state for data attributes
    let get_state = move || match context.state.get() {
        CheckedState::True => "checked",
        CheckedState::False => "unchecked",
        CheckedState::Indeterminate => "indeterminate",
    };

    view! {
        <Show when=move || is_present.get()>
            <div
                class=move || format!("leptonic-checkbox-indicator absolute inset-0 flex items-center justify-center text-black font-bold text-sm leading-none pointer-events-none {}", class.get())
                data-state=get_state
                data-disabled=move || context.disabled.get().then_some("")
            >
                {children()}
            </div>
        </Show>
    }
}

/// Clean checkmark icon matching Radix UI design
#[component]
pub fn CheckIcon() -> impl IntoView {
    view! {
        "✓"
    }
}
