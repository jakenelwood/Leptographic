use leptos::prelude::*;
use std::sync::atomic::{AtomicUsize, Ordering};

/// Global counter for generating unique IDs
static ID_COUNTER: AtomicUsize = AtomicUsize::new(0);

/// Hook to generate a unique ID for accessibility and form integration
///
/// Essential for ARIA relationships, form labels, and any element that needs
/// a stable, unique identifier.
///
/// # Example
/// ```rust
/// let input_id = use_id();
/// let label_id = use_id();
///
/// view! {
///     <label id=label_id for=input_id>"Username"</label>
///     <input id=input_id aria-labelledby=label_id />
/// }
/// ```
pub fn use_id() -> Signal<String> {
    let id = ID_COUNTER.fetch_add(1, Ordering::Relaxed);
    Signal::derive(move || format!("leptos-radix-{id}"))
}

/// Hook to generate a unique ID with a custom prefix
///
/// # Example
/// ```rust
/// let dialog_id = use_id_with_prefix("dialog");
/// let button_id = use_id_with_prefix("button");
///
/// // Generates: "dialog-1", "button-2", etc.
/// ```
pub fn use_id_with_prefix(prefix: &'static str) -> Signal<String> {
    let id = ID_COUNTER.fetch_add(1, Ordering::Relaxed);
    Signal::derive(move || format!("{prefix}-{id}"))
}

/// Return type for use_related_ids hook
pub struct UseRelatedIdsReturn {
    /// Base ID for the main element
    pub base_id: Signal<String>,
    /// ID for the trigger element
    pub trigger_id: Signal<String>,
    /// ID for the content element
    pub content_id: Signal<String>,
    /// ID for the label element
    pub label_id: Signal<String>,
    /// ID for the description element
    pub description_id: Signal<String>,
}

/// Hook to generate a set of related IDs for complex components
///
/// Perfect for components like Dialog, Tooltip, Dropdown that need
/// multiple related elements with proper ARIA relationships.
///
/// # Example
/// ```rust
/// let ids = use_related_ids("tooltip");
///
/// view! {
///     <button
///         id=ids.trigger_id
///         aria-describedby=ids.content_id
///     >
///         "Hover me"
///     </button>
///     <div
///         id=ids.content_id
///         role="tooltip"
///         aria-labelledby=ids.label_id
///     >
///         <div id=ids.label_id>"Tooltip Title"</div>
///         <div id=ids.description_id>"Tooltip description"</div>
///     </div>
/// }
/// ```
pub fn use_related_ids(prefix: &'static str) -> UseRelatedIdsReturn {
    let base_id = ID_COUNTER.fetch_add(1, Ordering::Relaxed);

    UseRelatedIdsReturn {
        base_id: Signal::derive(move || format!("{prefix}-{base_id}")),
        trigger_id: Signal::derive(move || format!("{prefix}-{base_id}-trigger")),
        content_id: Signal::derive(move || format!("{prefix}-{base_id}-content")),
        label_id: Signal::derive(move || format!("{prefix}-{base_id}-label")),
        description_id: Signal::derive(move || format!("{prefix}-{base_id}-description")),
    }
}

/// Hook to generate IDs for form elements with proper relationships
///
/// # Example
/// ```rust
/// let form_ids = use_form_ids("username");
///
/// view! {
///     <label id=form_ids.label_id for=form_ids.input_id>
///         "Username"
///     </label>
///     <input
///         id=form_ids.input_id
///         aria-labelledby=form_ids.label_id
///         aria-describedby=form_ids.description_id
///         aria-errormessage=form_ids.error_id
///     />
///     <div id=form_ids.description_id>
///         "Enter your username"
///     </div>
///     <div id=form_ids.error_id>
///         "Username is required"
///     </div>
/// }
/// ```
pub struct UseFormIdsReturn {
    /// ID for the input element
    pub input_id: Signal<String>,
    /// ID for the label element
    pub label_id: Signal<String>,
    /// ID for the description element
    pub description_id: Signal<String>,
    /// ID for the error message element
    pub error_id: Signal<String>,
}

pub fn use_form_ids(field_name: &'static str) -> UseFormIdsReturn {
    let base_id = ID_COUNTER.fetch_add(1, Ordering::Relaxed);

    UseFormIdsReturn {
        input_id: Signal::derive(move || format!("{field_name}-{base_id}-input")),
        label_id: Signal::derive(move || format!("{field_name}-{base_id}-label")),
        description_id: Signal::derive(move || format!("{field_name}-{base_id}-description")),
        error_id: Signal::derive(move || format!("{field_name}-{base_id}-error")),
    }
}

/// Hook for generating IDs with custom patterns
///
/// # Example
/// ```rust
/// let custom_ids = use_custom_id_pattern("modal", vec!["header", "body", "footer"]);
/// // Generates: modal-1-header, modal-1-body, modal-1-footer
/// ```
pub fn use_custom_id_pattern(
    prefix: &'static str,
    suffixes: Vec<&'static str>,
) -> Vec<Signal<String>> {
    let base_id = ID_COUNTER.fetch_add(1, Ordering::Relaxed);

    suffixes
        .into_iter()
        .map(move |suffix| Signal::derive(move || format!("{prefix}-{base_id}-{suffix}")))
        .collect()
}

/// Hook to generate a stable ID that persists across re-renders
///
/// Uses a provided key to ensure the same ID is generated for the same key.
///
/// # Example
/// ```rust
/// let stable_id = use_stable_id("my-unique-key");
/// // Always generates the same ID for "my-unique-key"
/// ```
pub fn use_stable_id(key: &'static str) -> Signal<String> {
    // Simple hash function for the key
    let hash = key
        .chars()
        .fold(0u32, |acc, c| acc.wrapping_mul(31).wrapping_add(c as u32));

    Signal::derive(move || format!("leptos-radix-stable-{hash}"))
}

// TODO: Add SSR-safe ID generation
// TODO: Add tests with proper HydrationCtx
