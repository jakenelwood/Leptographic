use super::use_controllable_state;
use crate::components::checkbox::CheckedState;
use leptos::prelude::*;

/// Checkbox-specific state hook that provides clean state management
/// and utility functions for checkbox components.
///
/// # Example
/// ```rust
/// let checkbox_state = use_checkbox_state(
///     checked,
///     default_checked,
///     on_checked_change
/// );
///
/// // Use in component
/// view! {
///     <button
///         aria-checked=move || checkbox_state.get_aria_checked()
///         data-state=move || checkbox_state.get_state_attr()
///         on:click=move |_| checkbox_state.toggle()
///     >
///         "Checkbox"
///     </button>
/// }
/// ```
pub fn use_checkbox_state(
    checked: MaybeProp<CheckedState>,
    default_checked: MaybeProp<CheckedState>,
    on_checked_change: Option<Callback<CheckedState>>,
) -> UseCheckboxStateReturn {
    let state = use_controllable_state(checked, default_checked, on_checked_change);

    // Toggle function: False -> True, everything else -> False (takes 1 argument)
    let toggle = {
        let set_value = state.set_value;
        move |_: ()| {
            let new_state = match state.value.get() {
                CheckedState::False => CheckedState::True,
                _ => CheckedState::False,
            };
            set_value.run(new_state);
        }
    };

    // Set to indeterminate state (takes 1 argument)
    let set_indeterminate = {
        let set_value = state.set_value;
        move |_: ()| {
            set_value.run(CheckedState::Indeterminate);
        }
    };

    // ARIA checked attribute value (use Memo for computed values)
    let get_aria_checked = Memo::new(move |_| match state.value.get() {
        CheckedState::True => "true",
        CheckedState::False => "false",
        CheckedState::Indeterminate => "mixed",
    });

    // Data state attribute value (use Memo for computed values)
    let get_state_attr = Memo::new(move |_| match state.value.get() {
        CheckedState::True => "checked",
        CheckedState::False => "unchecked",
        CheckedState::Indeterminate => "indeterminate",
    });

    // Check if checkbox is checked (true or indeterminate)
    let is_checked = Signal::derive(move || {
        matches!(
            state.value.get(),
            CheckedState::True | CheckedState::Indeterminate
        )
    });

    UseCheckboxStateReturn {
        checked: state.value,
        toggle: Callback::new(toggle),
        set_indeterminate: Callback::new(set_indeterminate),
        get_aria_checked,
        get_state_attr,
        is_checked,
        is_controlled: state.is_controlled,
    }
}

/// Return type for use_checkbox_state hook
pub struct UseCheckboxStateReturn {
    /// Current checked state
    pub checked: Signal<CheckedState>,
    /// Toggle between checked/unchecked
    pub toggle: Callback<()>,
    /// Set to indeterminate state
    pub set_indeterminate: Callback<()>,
    /// Get ARIA checked attribute value
    pub get_aria_checked: Memo<&'static str>,
    /// Get data-state attribute value
    pub get_state_attr: Memo<&'static str>,
    /// Whether checkbox is checked (true or indeterminate)
    pub is_checked: Signal<bool>,
    /// Whether the component is in controlled mode
    pub is_controlled: Signal<bool>,
}

// TODO: Add tests with proper HydrationCtx and correct API usage
