use crate::hooks::{use_progress_state, UseProgressStateReturn};
use leptos::context::Provider;
use leptos::prelude::*;

/// Helper function to generate progress CSS classes
fn get_progress_classes(user_class: String) -> String {
    let base = "relative h-2 w-full overflow-hidden rounded-full bg-gray-200";
    let states = "data-[state=indeterminate]:animate-pulse";
    
    format!("{base} {states} {user_class}")
}

/// Helper function to generate progress indicator CSS classes
fn get_progress_indicator_classes(user_class: String) -> String {
    let base = "h-full bg-black transition-all duration-300 ease-in-out";
    let states = "data-[state=indeterminate]:animate-pulse data-[state=indeterminate]:bg-gradient-to-r data-[state=indeterminate]:from-gray-400 data-[state=indeterminate]:to-black";
    
    format!("{base} {states} {user_class}")
}

/// Renders the progress view with all elements
#[component]
fn ProgressView(
    progress_state: UseProgressStateReturn,
    final_id: Signal<String>,
    class: MaybeProp<String>,
    children: ChildrenFn,
) -> impl IntoView {
    view! {
        <div
            id=move || final_id.get()
            role="progressbar"
            // ARIA attributes from our hook
            aria-valuenow=move || progress_state.get_aria_valuenow.get()
            aria-valuemax=move || progress_state.get_aria_valuemax.get()
            aria-valuemin=move || progress_state.get_aria_valuemin.get()
            // Data attributes for styling
            data-state=move || progress_state.get_state_attr.get()
            data-value=move || progress_state.value.get().map(|v| v.to_string())
            data-max=move || progress_state.max.get().to_string()
            // Professional progress bar styling
            class=move || get_progress_classes(class.get().unwrap_or_default())
        >
            {children()}
        </div>
    }
}

/// Context value shared between Progress and ProgressIndicator
#[derive(Clone, Debug)]
pub struct ProgressContextValue {
    pub value: Signal<Option<f64>>,
    pub max: Signal<f64>,
    pub indeterminate: Signal<bool>,
    pub percentage: Memo<Option<f64>>,
    pub get_state_attr: Memo<&'static str>,
    pub get_progress_style: Memo<String>,
}

/// Progress component - Hook-first implementation
///
/// Uses our proven hook library for state management and ARIA compliance.
/// Styled with Tailwind CSS 4 ONLY - no custom CSS allowed.
#[component]
pub fn Progress(
    // Core state management
    #[prop(into, optional)] value: MaybeProp<Option<f64>>,
    #[prop(into, optional)] max: MaybeProp<f64>,
    #[prop(into, optional)] indeterminate: MaybeProp<bool>,

    // Accessibility & DOM
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] class: MaybeProp<String>,

    children: ChildrenFn,
) -> impl IntoView {
    // Compose hooks - no manual state management!
    let progress_state = use_progress_state(value, max, indeterminate);

    // Pre-compute common values to reduce complexity
    let final_id = Signal::derive(move || id.get().unwrap_or_else(|| "progress".to_string()));

    // Context for child components
    let context_value = ProgressContextValue {
        value: progress_state.value,
        max: progress_state.max,
        indeterminate: progress_state.indeterminate,
        percentage: progress_state.percentage,
        get_state_attr: progress_state.get_state_attr,
        get_progress_style: progress_state.get_progress_style,
    };

    view! {
        <Provider value=context_value>
            <ProgressView
                progress_state=progress_state
                final_id=final_id
                class=class
                children=children
            />
        </Provider>
    }
}

/// ProgressIndicator - Shows the actual progress bar
#[component]
pub fn ProgressIndicator(
    #[prop(into, optional)] class: MaybeProp<String>,
) -> impl IntoView {
    let context = expect_context::<ProgressContextValue>();

    view! {
        <div
            data-state=move || context.get_state_attr.get()
            data-value=move || context.value.get().map(|v| v.to_string())
            style=move || context.get_progress_style.get()
            // Professional indicator styling with smooth animation
            class=move || get_progress_indicator_classes(class.get().unwrap_or_default())
        />
    }
}
