use leptos::prelude::*;

/// Progress-specific state hook that provides clean state management
/// for progress components with value, max, and indeterminate states.
///
/// # Example
/// ```rust
/// let progress_state = use_progress_state(
///     value,
///     max,
///     indeterminate
/// );
///
/// // Use in component
/// view! {
///     <div
///         role="progressbar"
///         aria-valuenow=move || progress_state.get_aria_valuenow()
///         aria-valuemax=move || progress_state.get_aria_valuemax()
///         data-state=move || progress_state.get_state_attr()
///     >
///         <ProgressIndicator />
///     </div>
/// }
/// ```
pub fn use_progress_state(
    value: MaybeProp<Option<f64>>,
    max: MaybeProp<f64>,
    indeterminate: MaybeProp<bool>,
) -> UseProgressStateReturn {
    // Reactive values
    let current_value = Signal::derive(move || value.get().unwrap_or(None));
    let current_max = Signal::derive(move || max.get().unwrap_or(100.0));
    let is_indeterminate = Signal::derive(move || indeterminate.get().unwrap_or(false));

    // Computed percentage (0-100)
    let percentage = Memo::new(move |_| {
        if is_indeterminate.get() {
            None
        } else if let Some(val) = current_value.get() {
            let max_val = current_max.get();
            if max_val <= 0.0 {
                Some(0.0)
            } else {
                Some((val.clamp(0.0, max_val) / max_val * 100.0).clamp(0.0, 100.0))
            }
        } else {
            None
        }
    });

    // ARIA value now (use Memo for computed values)
    let get_aria_valuenow = Memo::new(move |_| {
        if is_indeterminate.get() {
            None
        } else {
            current_value.get()
        }
    });

    // ARIA value max (use Memo for computed values)
    let get_aria_valuemax = Memo::new(move |_| current_max.get());

    // ARIA value min (always 0 for progress)
    let get_aria_valuemin = Memo::new(move |_| 0.0);

    // Data state attribute value (use Memo for computed values)
    let get_state_attr = Memo::new(move |_| {
        if is_indeterminate.get() {
            "indeterminate"
        } else if current_value.get().is_some() {
            "loading"
        } else {
            "complete"
        }
    });

    // Progress bar style for width percentage
    let get_progress_style = Memo::new(move |_| {
        if let Some(pct) = percentage.get() {
            format!("width: {}%", pct)
        } else {
            "width: 0%".to_string()
        }
    });

    UseProgressStateReturn {
        value: current_value,
        max: current_max,
        indeterminate: is_indeterminate,
        percentage,
        get_aria_valuenow,
        get_aria_valuemax,
        get_aria_valuemin,
        get_state_attr,
        get_progress_style,
    }
}

/// Return type for use_progress_state hook
pub struct UseProgressStateReturn {
    /// Current progress value
    pub value: Signal<Option<f64>>,
    /// Maximum progress value
    pub max: Signal<f64>,
    /// Whether progress is indeterminate
    pub indeterminate: Signal<bool>,
    /// Computed percentage (0-100)
    pub percentage: Memo<Option<f64>>,
    /// Get ARIA valuenow attribute value
    pub get_aria_valuenow: Memo<Option<f64>>,
    /// Get ARIA valuemax attribute value
    pub get_aria_valuemax: Memo<f64>,
    /// Get ARIA valuemin attribute value (always 0)
    pub get_aria_valuemin: Memo<f64>,
    /// Get data-state attribute value
    pub get_state_attr: Memo<&'static str>,
    /// Get progress bar style for width
    pub get_progress_style: Memo<String>,
}
