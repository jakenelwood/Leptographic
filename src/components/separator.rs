use leptos::prelude::*;

/// Separator component - Leptos 0.8.2 implementation of Radix UI Separator
///
/// Source: https://github.com/radix-ui/primitives/tree/main/packages/react/separator
/// Reference: https://github.com/RustForWeb/radix/tree/main/packages/primitives/leptos/separator
///
/// Visually or semantically separates content.

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum Orientation {
    #[default]
    Horizontal,
    Vertical,
}

impl Orientation {
    pub fn as_str(&self) -> &'static str {
        match self {
            Orientation::Horizontal => "horizontal",
            Orientation::Vertical => "vertical",
        }
    }
}

/// Production-ready Separator component with Phase III patterns (Leptos 0.8.2)
#[component]
pub fn Separator(
    /// Either `horizontal` or `vertical`. Defaults to `horizontal`.
    #[prop(optional)]
    orientation: Option<Orientation>,

    /// Whether or not the component is purely decorative. When true, accessibility-related
    /// attributes are updated so that the rendered element is removed from the accessibility tree.
    #[prop(optional)]
    decorative: Option<bool>,

    /// CSS class for styling
    #[prop(optional, into)]
    class: Option<String>,

    /// Optional ID for the separator element (useful for form integration)
    #[prop(optional, into)]
    id: Option<String>,

    /// Optional style attribute for inline styling
    #[prop(optional, into)]
    style: Option<String>,

    /// Data attributes for custom styling hooks
    #[prop(optional, into)]
    data_testid: Option<String>,

    /// ARIA label
    #[prop(optional, into)]
    aria_label: Option<String>,

    /// ARIA labelledby
    #[prop(optional, into)]
    aria_labelledby: Option<String>,

    /// ARIA describedby
    #[prop(optional, into)]
    aria_describedby: Option<String>,

    /// Node reference - 0.8.2 compatible
    #[prop(optional)]
    node_ref: Option<NodeRef<leptos::html::Div>>,

    /// Child components (optional for separator)
    #[prop(optional)]
    children: Option<ChildrenFn>,
) -> impl IntoView {
    // Production-ready prop handling with validation
    let orientation = orientation.unwrap_or_default();
    let decorative = decorative.unwrap_or(false);

    // Enhanced class handling for production use
    let class_value = match class {
        Some(c) => format!("separator-root {}", c),
        None => "separator-root".to_string(),
    };

    // ARIA orientation defaults to horizontal, so we only need it if vertical
    let aria_orientation = match orientation {
        Orientation::Vertical => Some("vertical"),
        Orientation::Horizontal => None,
    };

    // Semantic props based on decorative flag
    let (role, aria_orientation_attr) = if decorative {
        ("none", None)
    } else {
        ("separator", aria_orientation)
    };

    // Data state for styling hooks
    let data_state = if decorative { "decorative" } else { "semantic" };

    view! {
        <div
            id=id
            class=class_value
            style=style
            role=role
            data-radix-separator=""
            data-orientation=orientation.as_str()
            data-state=data_state
            data-testid=data_testid
            aria-orientation=aria_orientation_attr
            aria-label=aria_label
            aria-labelledby=aria_labelledby
            aria-describedby=aria_describedby
            node_ref=node_ref.unwrap_or_default()
        >
            {children.as_ref().map(|children| children())}
        </div>
    }
}
