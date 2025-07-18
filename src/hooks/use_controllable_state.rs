use leptos::prelude::*;

/// Universal controllable state hook inspired by leptos-use patterns
///
/// This hook provides a unified interface for components that can be either
/// controlled (external state) or uncontrolled (internal state).
///
/// # Example
/// ```rust
/// let state = use_controllable_state(
///     checked,           // Optional controlled value
///     default_checked,   // Optional default value
///     on_checked_change  // Optional change callback
/// );
///
/// // Use state.value to read current value
/// // Use state.set_value to update value
/// ```
pub fn use_controllable_state<T>(
    controlled_value: MaybeProp<T>,
    default_value: MaybeProp<T>,
    on_change: Option<Callback<T>>,
) -> UseControllableStateReturn<T>
where
    T: Clone + PartialEq + Default + Send + Sync + 'static,
{
    // Internal state for uncontrolled mode
    let (internal_value, set_internal_value) = signal(default_value.get().unwrap_or_default());

    // Current value: controlled takes precedence over internal
    let current_value = Signal::derive(move || {
        controlled_value
            .get()
            .unwrap_or_else(|| internal_value.get())
    });

    // Setter that handles both controlled and uncontrolled modes
    let set_value = move |new_value: T| {
        // Only update internal state if not controlled
        if controlled_value.get().is_none() {
            set_internal_value.set(new_value.clone());
        }

        // Always call the change callback if provided
        if let Some(callback) = on_change {
            callback.run(new_value);
        }
    };

    UseControllableStateReturn {
        value: current_value,
        set_value: Callback::new(set_value),
        is_controlled: Signal::derive(move || controlled_value.get().is_some()),
    }
}

/// Return type for use_controllable_state hook
pub struct UseControllableStateReturn<T: Send + Sync + 'static> {
    /// Current value (controlled or internal)
    pub value: Signal<T>,
    /// Function to update the value
    pub set_value: Callback<T>,
    /// Whether the component is in controlled mode
    pub is_controlled: Signal<bool>,
}

// TODO: Add tests once MaybeProp API is clarified
