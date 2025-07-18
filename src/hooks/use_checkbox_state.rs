use leptos::prelude::*;
use std::fmt::{Display, Formatter};

/// Represents the checked state of a checkbox
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Default)]
pub enum CheckedState {
    #[default]
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

/// Return type for use_checkbox_state hook
pub struct UseCheckboxStateReturn {
    pub checked: Signal<CheckedState>,
    pub toggle: Callback<()>,
    pub get_aria_checked: Memo<&'static str>,
    pub get_state_attr: Memo<&'static str>,
    pub get_form_value: Memo<&'static str>,
}

/// Hook for checkbox state management
///
/// Provides complete checkbox state logic including:
/// - Controlled/uncontrolled state management
/// - ARIA attribute generation
/// - Form value handling
/// - Toggle functionality
pub fn use_checkbox_state(
    checked: MaybeProp<CheckedState>,
    default_checked: MaybeProp<CheckedState>,
    on_checked_change: Option<Callback<CheckedState>>,
) -> UseCheckboxStateReturn {
    // Use our universal controllable state hook
    let (internal_checked, set_internal_checked) = signal(
        checked
            .get()
            .or(default_checked.get())
            .unwrap_or(CheckedState::False),
    );

    let current_checked =
        Signal::derive(move || checked.get().unwrap_or_else(|| internal_checked.get()));

    // Toggle function
    let toggle = Callback::new(move |_: ()| {
        let current = current_checked.get();
        let new_state = match current {
            CheckedState::False => CheckedState::True,
            CheckedState::True => CheckedState::False,
            CheckedState::Indeterminate => CheckedState::True, // Indeterminate -> True
        };

        // Update internal state if uncontrolled
        if checked.get().is_none() {
            set_internal_checked.set(new_state);
        }

        // Call external callback
        if let Some(callback) = on_checked_change {
            callback.run(new_state);
        }
    });

    // ARIA attributes (memoized for performance)
    let get_aria_checked = Memo::new(move |_| {
        match current_checked.get() {
            CheckedState::True => "true",
            CheckedState::False => "false",
            CheckedState::Indeterminate => "mixed", // ARIA uses "mixed" for indeterminate
        }
    });

    // Data state attribute for Tailwind CSS 4
    let get_state_attr = Memo::new(move |_| match current_checked.get() {
        CheckedState::True => "checked",
        CheckedState::False => "unchecked",
        CheckedState::Indeterminate => "indeterminate",
    });

    // Form value for hidden input
    let get_form_value = Memo::new(move |_| match current_checked.get() {
        CheckedState::True => "on",
        CheckedState::False => "",
        CheckedState::Indeterminate => "mixed",
    });

    UseCheckboxStateReturn {
        checked: current_checked,
        toggle,
        get_aria_checked,
        get_state_attr,
        get_form_value,
    }
}
