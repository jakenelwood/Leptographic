use leptos::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{Event, KeyboardEvent};

/// Hook to handle Escape key presses
///
/// Essential for modals, dropdowns, tooltips, and any overlay that should
/// close when pressing Escape.
///
/// # Example
/// ```rust
/// let (is_open, set_is_open) = signal(false);
///
/// use_escape_key(Callback::new(move |_| {
///     set_is_open.set(false);
/// }));
/// ```
pub fn use_escape_key(on_escape: Callback<KeyboardEvent>) {
    Effect::new(move |_| {
        let callback = on_escape;

        let closure = wasm_bindgen::closure::Closure::wrap(Box::new(move |event: Event| {
            if let Ok(keyboard_event) = event.dyn_into::<KeyboardEvent>() {
                if keyboard_event.key() == "Escape" {
                    callback.run(keyboard_event);
                }
            }
        }) as Box<dyn FnMut(Event)>);

        // Add event listener to document
        if let Some(document) = web_sys::window().and_then(|w| w.document()) {
            let _ = document
                .add_event_listener_with_callback("keydown", closure.as_ref().unchecked_ref());

            // Store closure to prevent it from being dropped
            closure.forget();
        }
    });
}

/// Hook to handle Escape key with conditional enabling
///
/// Only listens for Escape when enabled is true.
///
/// # Example
/// ```rust
/// let (is_modal_open, set_is_modal_open) = signal(false);
///
/// use_escape_key_when(
///     is_modal_open,
///     Callback::new(move |_| set_is_modal_open.set(false))
/// );
/// ```
pub fn use_escape_key_when(enabled: Signal<bool>, on_escape: Callback<KeyboardEvent>) {
    Effect::new(move |_| {
        if !enabled.get() {
            return;
        }

        let callback = on_escape;

        let closure = wasm_bindgen::closure::Closure::wrap(Box::new(move |event: Event| {
            if let Ok(keyboard_event) = event.dyn_into::<KeyboardEvent>() {
                if keyboard_event.key() == "Escape" {
                    callback.run(keyboard_event);
                }
            }
        }) as Box<dyn FnMut(Event)>);

        if let Some(document) = web_sys::window().and_then(|w| w.document()) {
            let _ = document
                .add_event_listener_with_callback("keydown", closure.as_ref().unchecked_ref());

            closure.forget();
        }
    });
}

/// Configuration for escape key handling
pub struct UseEscapeKeyConfig {
    /// Whether to prevent default behavior
    pub prevent_default: bool,
    /// Whether to stop propagation
    pub stop_propagation: bool,
    /// Whether the hook is enabled
    pub enabled: bool,
    /// Additional key codes to listen for
    pub additional_keys: Vec<String>,
}

impl Default for UseEscapeKeyConfig {
    fn default() -> Self {
        Self {
            prevent_default: false,
            stop_propagation: false,
            enabled: true,
            additional_keys: vec![],
        }
    }
}

/// Advanced escape key hook with configuration options
///
/// # Example
/// ```rust
/// use_escape_key_with_config(
///     callback,
///     UseEscapeKeyConfig {
///         prevent_default: true,
///         stop_propagation: true,
///         additional_keys: vec!["Enter".to_string()],
///         enabled: is_dialog_open.get(),
///     }
/// );
/// ```
pub fn use_escape_key_with_config(on_key: Callback<KeyboardEvent>, config: UseEscapeKeyConfig) {
    Effect::new(move |_| {
        if !config.enabled {
            return;
        }

        let callback = on_key;
        let prevent_default = config.prevent_default;
        let stop_propagation = config.stop_propagation;
        let additional_keys = config.additional_keys.clone();

        let closure = wasm_bindgen::closure::Closure::wrap(Box::new(move |event: Event| {
            if let Ok(keyboard_event) = event.dyn_into::<KeyboardEvent>() {
                let key = keyboard_event.key();
                let should_handle = key == "Escape" || additional_keys.contains(&key);

                if should_handle {
                    if prevent_default {
                        keyboard_event.prevent_default();
                    }
                    if stop_propagation {
                        keyboard_event.stop_propagation();
                    }

                    callback.run(keyboard_event);
                }
            }
        }) as Box<dyn FnMut(Event)>);

        if let Some(document) = web_sys::window().and_then(|w| w.document()) {
            let _ = document
                .add_event_listener_with_callback("keydown", closure.as_ref().unchecked_ref());

            closure.forget();
        }
    });
}

/// Hook for handling multiple key combinations
///
/// # Example
/// ```rust
/// use_key_combinations(vec![
///     ("Escape", Callback::new(move |_| close_modal())),
///     ("Enter", Callback::new(move |_| submit_form())),
///     ("ArrowDown", Callback::new(move |_| navigate_down())),
/// ]);
/// ```
pub fn use_key_combinations(key_handlers: Vec<(&'static str, Callback<KeyboardEvent>)>) {
    Effect::new(move |_| {
        let handlers = key_handlers.clone();

        let closure = wasm_bindgen::closure::Closure::wrap(Box::new(move |event: Event| {
            if let Ok(keyboard_event) = event.dyn_into::<KeyboardEvent>() {
                let key = keyboard_event.key();

                for (target_key, callback) in &handlers {
                    if key == *target_key {
                        callback.run(keyboard_event.clone());
                        break;
                    }
                }
            }
        }) as Box<dyn FnMut(Event)>);

        if let Some(document) = web_sys::window().and_then(|w| w.document()) {
            let _ = document
                .add_event_listener_with_callback("keydown", closure.as_ref().unchecked_ref());

            closure.forget();
        }
    });
}

// TODO: Add proper cleanup for event listeners
// TODO: Add tests with proper HydrationCtx
