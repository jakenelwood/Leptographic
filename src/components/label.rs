use leptos::prelude::*;
use leptos::wasm_bindgen::JsCast;

/// Label component - Leptos 0.8.2 implementation of Radix UI Label
///
/// Source: https://github.com/radix-ui/primitives/tree/main/packages/react/label
/// 
/// Renders an accessible label associated with controls.

/// Production-ready Label component with enhanced UX patterns (Leptos 0.8.2)
#[component]
pub fn Label(
    /// The ID of the form control this label is associated with
    #[prop(optional, into)]
    for_: Option<String>,

    /// CSS class for styling
    #[prop(optional, into)]
    class: Option<String>,

    /// Optional ID for the label element
    #[prop(optional, into)]
    id: Option<String>,

    /// Optional style attribute for inline styling
    #[prop(optional, into)]
    style: Option<String>,

    /// Data attributes for custom styling hooks
    #[prop(optional, into)]
    data_testid: Option<String>,

    /// ARIA label for additional accessibility
    #[prop(optional, into)]
    aria_label: Option<String>,

    /// ARIA labelledby for complex label associations
    #[prop(optional, into)]
    aria_labelledby: Option<String>,

    /// ARIA describedby for additional descriptions
    #[prop(optional, into)]
    aria_describedby: Option<String>,

    /// Whether this label is for a required field
    #[prop(optional)]
    required: Option<bool>,

    /// Whether this label is for a disabled field
    #[prop(optional)]
    disabled: Option<bool>,



    /// Node reference - 0.8.2 compatible
    #[prop(optional)]
    node_ref: Option<NodeRef<leptos::html::Label>>,

    /// Child components
    children: ChildrenFn,
) -> impl IntoView {
    // Production-ready prop handling with validation
    let required = required.unwrap_or(false);
    let disabled = disabled.unwrap_or(false);

    // Enhanced class handling for production use with state classes
    let mut class_parts = vec!["label-root"];
    if required {
        class_parts.push("label-required");
    }
    if disabled {
        class_parts.push("label-disabled");
    }
    if let Some(c) = &class {
        class_parts.push(c);
    }
    let class_value = class_parts.join(" ");

    // Data state for styling hooks
    let data_state = if disabled {
        "disabled"
    } else if required {
        "required"
    } else {
        "default"
    };

    // Smart mouse down handler to prevent text selection on double-click
    // while allowing interaction with form controls
    let handle_mouse_down = move |event: web_sys::MouseEvent| {
        // Only prevent text selection if clicking inside the label itself
        if let Some(target) = event.target() {
            if let Ok(element) = target.dyn_into::<web_sys::HtmlElement>() {
                // Check if the target is a form control or inside one
                let tag_name = element.tag_name().to_lowercase();
                if matches!(tag_name.as_str(), "button" | "input" | "select" | "textarea") {
                    return; // Allow normal interaction with form controls
                }

                // Check if target is inside a form control
                if let Ok(closest) = element.closest("button, input, select, textarea") {
                    if closest.is_some() {
                        return; // Allow normal interaction
                    }
                }
            }
        }

        // Prevent text selection when double clicking label
        if event.detail() > 1 {
            event.prevent_default();
        }
    };


    
    view! {
        <label
            id=id
            class=class_value
            style=style
            for=for_
            data-radix-label=""
            data-state=data_state
            data-required=move || required.then_some("")
            data-disabled=move || disabled.then_some("")
            data-testid=data_testid
            aria-label=aria_label
            aria-labelledby=aria_labelledby
            aria-describedby=aria_describedby
            node_ref=node_ref.unwrap_or_default()
            on:mousedown=handle_mouse_down
        >
            {children()}
        </label>
    }
}
