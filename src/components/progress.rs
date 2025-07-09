use leptos::prelude::*;
use leptos::context::Provider;

/// Progress component - Leptos 0.8.2 implementation of Radix UI Progress
///
/// Source: https://github.com/radix-ui/primitives/tree/main/packages/react/progress
/// Reference: https://github.com/RustForWeb/radix/tree/main/packages/primitives/leptos/progress
///
/// Displays an indicator showing the completion progress of a task, typically displayed as a progress bar.

const DEFAULT_MAX: f64 = 100.0;

// Context for sharing progress state between components
#[derive(Clone, Copy)]
struct ProgressContext {
    value: RwSignal<Option<f64>>,
    max: RwSignal<f64>,
}

/// Progress state enum for data attributes and styling
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ProgressState {
    Indeterminate,
    Loading,
    Complete,
}

impl ProgressState {
    fn as_str(&self) -> &'static str {
        match self {
            ProgressState::Indeterminate => "indeterminate",
            ProgressState::Loading => "loading", 
            ProgressState::Complete => "complete",
        }
    }
}

/// Calculate progress state based on value and max
fn get_progress_state(value: Option<f64>, max: f64) -> ProgressState {
    match value {
        None => ProgressState::Indeterminate,
        Some(v) if v >= max => ProgressState::Complete,
        Some(_) => ProgressState::Loading,
    }
}

/// Default value label formatter (returns percentage)
fn default_get_value_label(value: f64, max: f64) -> String {
    format!("{}%", ((value / max) * 100.0).round() as i32)
}

/// Validate that max is a positive number
fn is_valid_max(max: f64) -> bool {
    max > 0.0 && !max.is_nan() && max.is_finite()
}

/// Validate that value is within valid range
fn is_valid_value(value: f64, max: f64) -> bool {
    value >= 0.0 && value <= max && !value.is_nan() && value.is_finite()
}

/// Root Progress component that provides context for all progress parts
#[component]
pub fn Progress(
    /// The progress value. Can be `None` for indeterminate state.
    /// When provided, creates a controlled progress component.
    #[prop(optional, into)]
    value: Option<Signal<f64>>,

    /// The maximum progress value. Defaults to 100.
    #[prop(optional, into)]
    max: Option<Signal<f64>>,

    /// Function to get the accessible label text representing the current value.
    /// Defaults to percentage format (e.g., "25%").
    #[prop(optional)]
    get_value_label: Option<fn(f64, f64) -> String>,

    /// Optional ID for the progress element (useful for form integration)
    #[prop(optional, into)]
    id: Option<String>,

    /// Optional class name for styling
    #[prop(optional, into)]
    class: Option<String>,

    /// Child components
    children: ChildrenFn,
) -> impl IntoView {
    // Create reactive signals for max value with validation
    let max_signal = Signal::derive(move || {
        let max_value = max.as_ref().map(|m| m.get()).unwrap_or(DEFAULT_MAX);
        if is_valid_max(max_value) {
            max_value
        } else {
            leptos::logging::warn!("Invalid max value {} for Progress. Using default {}", max_value, DEFAULT_MAX);
            DEFAULT_MAX
        }
    });

    // Create reactive signals for value with validation
    let value_signal = Signal::derive(move || {
        value.as_ref().and_then(|v| {
            let val = v.get();
            let max_val = max_signal.get();
            if is_valid_value(val, max_val) {
                Some(val)
            } else {
                leptos::logging::warn!("Invalid value {} for Progress. Value must be between 0 and {}", val, max_val);
                None
            }
        })
    });
    
    // Create context with reactive signals
    let context_value = ProgressContext {
        value: RwSignal::new(value_signal.get()),
        max: RwSignal::new(max_signal.get()),
    };

    // Update context when signals change
    Effect::new(move |_| {
        context_value.value.set(value_signal.get());
        context_value.max.set(max_signal.get());
    });

    // Calculate derived values for ARIA attributes
    let current_value = value_signal;
    let current_max = max_signal;
    let progress_state = Signal::derive(move || get_progress_state(current_value.get(), current_max.get()));

    // Calculate value label for accessibility
    let get_label_fn = get_value_label.unwrap_or(default_get_value_label);
    let value_label = Signal::derive(move || {
        current_value.get().map(|v| get_label_fn(v, current_max.get()))
    });
    
    view! {
        <Provider value=context_value>
            <div
                id=id
                class=class
                role="progressbar"
                aria-valuemin="0"
                aria-valuemax=move || current_max.get()
                aria-valuenow=move || current_value.get()
                aria-valuetext=move || value_label.get()
                data-state=move || progress_state.get().as_str()
                data-value=move || current_value.get().map(|v| v.to_string()).unwrap_or_default()
                data-max=move || current_max.get()
            >
                {children()}
            </div>
        </Provider>
    }
}

/// Progress indicator component that shows the visual progress
#[component]
pub fn ProgressIndicator(
    /// Optional class name for styling
    #[prop(optional, into)]
    class: Option<String>,

    /// Optional style attribute for inline styling
    #[prop(optional, into)]
    style: Option<Signal<String>>,

    /// Child components (optional)
    #[prop(optional)]
    children: Option<ChildrenFn>,
) -> impl IntoView {
    let context = expect_context::<ProgressContext>();

    let current_value = Signal::derive(move || context.value.get());
    let current_max = Signal::derive(move || context.max.get());
    let progress_state = Signal::derive(move || get_progress_state(current_value.get(), current_max.get()));

    // Calculate width percentage for the progress indicator
    let width_percentage = Signal::derive(move || {
        match current_value.get() {
            Some(value) => {
                let max = current_max.get();
                if max > 0.0 {
                    ((value / max) * 100.0).min(100.0).max(0.0)
                } else {
                    0.0
                }
            }
            None => 100.0, // Indeterminate state shows full width with animation
        }
    });

    // Combine custom style with width calculation
    let combined_style = Signal::derive(move || {
        let width_style = format!("width: {}%", width_percentage.get());
        match style.as_ref() {
            Some(custom_style) => format!("{}; {}", custom_style.get(), width_style),
            None => width_style,
        }
    });

    view! {
        <div
            class=class
            style=move || Some(combined_style.get())
            data-state=move || progress_state.get().as_str()
            data-value=move || current_value.get().map(|v| v.to_string()).unwrap_or_default()
            data-max=move || current_max.get()
        >
            {children.map(|children| children())}
        </div>
    }
}
