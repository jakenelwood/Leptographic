use super::use_controllable_state;
use leptos::prelude::*;

/// Switch-specific state hook that provides clean state management
/// for switch components (simpler than checkbox - just on/off).
///
/// # Example
/// ```rust
/// let switch_state = use_switch_state(
///     checked,
///     default_checked,
///     on_checked_change
/// );
///
/// // Use in component
/// view! {
///     <button
///         role="switch"
///         aria-checked=move || switch_state.get_aria_checked()
///         data-state=move || switch_state.get_state_attr()
///         on:click=move |_| switch_state.toggle()
///     >
///         <SwitchThumb />
///     </button>
/// }
/// ```
pub fn use_switch_state(
    checked: MaybeProp<bool>,
    default_checked: MaybeProp<bool>,
    on_checked_change: Option<Callback<bool>>,
) -> UseSwitchStateReturn {
    let state = use_controllable_state(
        checked,
        default_checked.get().unwrap_or(false).into(),
        on_checked_change,
    );

    // Toggle function: true <-> false (takes 1 argument as required by Callback)
    let toggle = {
        let set_value = state.set_value;
        move |_: ()| {
            let new_value = !state.value.get();
            set_value.run(new_value);
        }
    };

    // ARIA checked attribute value (use Memo for computed values)
    let get_aria_checked = Memo::new(move |_| if state.value.get() { "true" } else { "false" });

    // Data state attribute value (use Memo for computed values)
    let get_state_attr = Memo::new(move |_| {
        if state.value.get() {
            "checked"
        } else {
            "unchecked"
        }
    });

    // Form value for hidden input (use Memo for computed values)
    let get_form_value = Memo::new(move |_| if state.value.get() { "on" } else { "" });

    UseSwitchStateReturn {
        checked: state.value,
        toggle: Callback::new(toggle),
        get_aria_checked,
        get_state_attr,
        get_form_value,
        is_controlled: state.is_controlled,
    }
}

/// Return type for use_switch_state hook
pub struct UseSwitchStateReturn {
    /// Current checked state
    pub checked: Signal<bool>,
    /// Toggle between checked/unchecked
    pub toggle: Callback<()>,
    /// Get ARIA checked attribute value
    pub get_aria_checked: Memo<&'static str>,
    /// Get data-state attribute value
    pub get_state_attr: Memo<&'static str>,
    /// Get form value for hidden input
    pub get_form_value: Memo<&'static str>,
    /// Whether the component is in controlled mode
    pub is_controlled: Signal<bool>,
}

// TODO: Add tests with proper HydrationCtx and correct API usage
