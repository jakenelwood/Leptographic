use leptos::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::Event;

/// Hook to detect clicks outside of a specific element
///
/// Essential for dropdowns, modals, tooltips, and any overlay that should
/// close when clicking outside.
///
/// # Example
/// ```rust
/// let dropdown_ref = NodeRef::<AnyElement>::new();
/// let (is_open, set_is_open) = signal(false);
///
/// use_outside_click(
///     dropdown_ref,
///     Callback::new(move |_| set_is_open.set(false))
/// );
///
/// view! {
///     <div node_ref=dropdown_ref>
///         "Dropdown content"
///     </div>
/// }
/// ```
pub fn use_outside_click(
    element_ref: NodeRef<web_sys::Element>,
    on_outside_click: Callback<Event>,
) {
    Effect::new(move |_| {
        let element = element_ref.get();

        if let Some(element) = element {
            let callback = on_outside_click.clone();

            let closure = wasm_bindgen::closure::Closure::wrap(Box::new(move |event: Event| {
                if let Some(target) = event.target() {
                    if let Ok(target_element) = target.dyn_into::<web_sys::Element>() {
                        // Check if the click target is outside our element
                        if !element.contains(Some(&target_element)) {
                            callback.run(event);
                        }
                    }
                }
            })
                as Box<dyn FnMut(Event)>);

            // Add event listener to document
            if let Some(document) = web_sys::window().and_then(|w| w.document()) {
                let _ = document
                    .add_event_listener_with_callback("click", closure.as_ref().unchecked_ref());

                // Store closure to prevent it from being dropped
                closure.forget();
            }
        }
    });
}

/// Hook to detect clicks outside multiple elements
///
/// Useful when you have multiple related elements (trigger + content)
/// that should all be considered "inside".
///
/// # Example
/// ```rust
/// let trigger_ref = NodeRef::<AnyElement>::new();
/// let content_ref = NodeRef::<AnyElement>::new();
///
/// use_outside_click_multiple(
///     vec![trigger_ref, content_ref],
///     Callback::new(move |_| close_dropdown())
/// );
/// ```
pub fn use_outside_click_multiple(
    element_refs: Vec<NodeRef<AnyElement>>,
    on_outside_click: Callback<Event>,
) {
    Effect::new(move |_| {
        let elements: Vec<_> = element_refs.iter().filter_map(|r| r.get()).collect();

        if !elements.is_empty() {
            let callback = on_outside_click.clone();

            let closure = wasm_bindgen::closure::Closure::wrap(Box::new(move |event: Event| {
                if let Some(target) = event.target() {
                    if let Ok(target_element) = target.dyn_into::<web_sys::Element>() {
                        // Check if the click target is outside ALL our elements
                        let is_outside = elements
                            .iter()
                            .all(|element| !element.contains(Some(&target_element)));

                        if is_outside {
                            callback.run(event);
                        }
                    }
                }
            })
                as Box<dyn FnMut(Event)>);

            // Add event listener to document
            if let Some(document) = web_sys::window().and_then(|w| w.document()) {
                let _ = document
                    .add_event_listener_with_callback("click", closure.as_ref().unchecked_ref());

                closure.forget();
            }
        }
    });
}

/// Configuration for outside click detection
pub struct UseOutsideClickConfig {
    /// Whether to capture events during the capture phase
    pub capture: bool,
    /// Event types to listen for (default: ["click"])
    pub events: Vec<&'static str>,
    /// Whether the hook is enabled
    pub enabled: bool,
}

impl Default for UseOutsideClickConfig {
    fn default() -> Self {
        Self {
            capture: false,
            events: vec!["click"],
            enabled: true,
        }
    }
}

/// Advanced outside click hook with configuration options
///
/// # Example
/// ```rust
/// use_outside_click_with_config(
///     element_ref,
///     callback,
///     UseOutsideClickConfig {
///         events: vec!["click", "touchstart"],
///         capture: true,
///         enabled: is_open.get(),
///     }
/// );
/// ```
pub fn use_outside_click_with_config(
    element_ref: NodeRef<AnyElement>,
    on_outside_click: Callback<Event>,
    config: UseOutsideClickConfig,
) {
    Effect::new(move |_| {
        if !config.enabled {
            return;
        }

        let element = element_ref.get();

        if let Some(element) = element {
            let callback = on_outside_click.clone();

            for event_type in &config.events {
                let callback = callback.clone();
                let closure = wasm_bindgen::closure::Closure::wrap(Box::new(move |event: Event| {
                    if let Some(target) = event.target() {
                        if let Ok(target_element) = target.dyn_into::<web_sys::Element>() {
                            if !element.contains(Some(&target_element)) {
                                callback.run(event);
                            }
                        }
                    }
                })
                    as Box<dyn FnMut(Event)>);

                if let Some(document) = web_sys::window().and_then(|w| w.document()) {
                    let _ = document.add_event_listener_with_callback_and_bool(
                        event_type,
                        closure.as_ref().unchecked_ref(),
                        config.capture,
                    );

                    closure.forget();
                }
            }
        }
    });
}

// TODO: Add proper cleanup for event listeners
// TODO: Add tests with proper HydrationCtx
