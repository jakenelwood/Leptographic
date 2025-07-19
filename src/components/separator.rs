use leptos::prelude::*;

/// Helper function to generate separator CSS classes
fn get_separator_classes(orientation: &str, decorative: bool, user_class: String) -> String {
    let base = match orientation {
        "vertical" => "inline-block w-px bg-gray-200",
        _ => "block h-px w-full bg-gray-200", // horizontal is default
    };
    
    let accessibility = if decorative {
        ""
    } else {
        "focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2"
    };
    
    format!("{base} {accessibility} {user_class}").trim().to_string()
}

/// Separator component - A visual divider between content sections
///
/// Provides both horizontal and vertical separators with proper ARIA semantics.
/// Styled with Tailwind CSS 4 ONLY - no custom CSS allowed.
///
/// # Examples
/// ```rust
/// // Horizontal separator (default)
/// view! {
///     <Separator />
/// }
///
/// // Vertical separator
/// view! {
///     <Separator orientation="vertical" class="h-6" />
/// }
///
/// // Decorative separator (no ARIA semantics)
/// view! {
///     <Separator decorative=true />
/// }
/// ```
#[component]
pub fn Separator(
    /// Orientation of the separator: "horizontal" (default) or "vertical"
    #[prop(into, optional)] orientation: MaybeProp<String>,
    
    /// Whether the separator is purely decorative (no ARIA semantics)
    #[prop(into, optional)] decorative: MaybeProp<bool>,
    
    /// Additional CSS classes
    #[prop(into, optional)] class: MaybeProp<String>,
    
    /// Optional ID for the separator
    #[prop(into, optional)] id: MaybeProp<String>,
) -> impl IntoView {
    // Reactive values with defaults
    let current_orientation = Signal::derive(move || {
        orientation.get().unwrap_or_else(|| "horizontal".to_string())
    });
    let is_decorative = Signal::derive(move || decorative.get().unwrap_or(false));
    let final_id = Signal::derive(move || id.get());
    let user_class = Signal::derive(move || class.get().unwrap_or_default());

    // Determine ARIA attributes based on decorative flag
    let role = Signal::derive(move || {
        if is_decorative.get() {
            None
        } else {
            Some("separator")
        }
    });

    let aria_orientation = Signal::derive(move || {
        if is_decorative.get() {
            None
        } else {
            let orient = current_orientation.get();
            if orient == "vertical" {
                Some("vertical")
            } else {
                Some("horizontal")
            }
        }
    });

    view! {
        <div
            id=move || final_id.get()
            role=move || role.get()
            aria-orientation=move || aria_orientation.get()
            data-orientation=move || current_orientation.get()
            class=move || get_separator_classes(
                &current_orientation.get(),
                is_decorative.get(),
                user_class.get()
            )
        />
    }
}
