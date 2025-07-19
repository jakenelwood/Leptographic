use leptos::prelude::*;

const DEFAULT_MAX: f64 = 100.0;

/// Context value for sharing progress state between components
#[derive(Clone)]
struct ProgressContextValue {
    value: Signal<Option<f64>>,
    max: Signal<f64>,
}

#[component]
pub fn Progress(
    #[prop(into, optional)] value: MaybeProp<f64>,
    #[prop(into, optional)] max: MaybeProp<f64>,
    #[prop(into, optional)] class: MaybeProp<String>,
    children: ChildrenFn,
) -> impl IntoView {
    // Derive reactive signals for max and value with validation (following Leptix pattern)
    let max_signal = Signal::derive(move || {
        let max_val = max.get().unwrap_or(DEFAULT_MAX);
        if !max_val.is_nan() && max_val > 0.0 {
            max_val
        } else {
            DEFAULT_MAX
        }
    });

    let value_signal = Signal::derive(move || {
        let max_val = max_signal.get();
        value.get().and_then(|value| {
            (!value.is_nan() && value <= max_val && value >= 0.0).then_some(value)
        })
    });

    // Create context value for child components
    let context_value = ProgressContextValue {
        value: value_signal,
        max: max_signal,
    };

    provide_context(context_value);

    view! {
        <div
            class=move || {
                let mut class_str = String::from("relative overflow-hidden bg-black/25 rounded-full h-[25px] drop-shadow-md");
                if let Some(custom_class) = class.get() {
                    class_str.push(' ');
                    class_str.push_str(&custom_class);
                }
                class_str
            }
            style="transform: translateZ(0)"
            role="progressbar"
            aria-valuemax=move || max_signal.get()
            aria-valuemin="0"
            aria-valuenow=move || value_signal.get()
            data-state=move || {
                value_signal.get().map(|v| {
                    if v >= max_signal.get() { "complete" } else { "loading" }
                }).unwrap_or("indeterminate")
            }
            data-value=move || value_signal.get()
            data-max=move || max_signal.get()
        >
            {children()}
        </div>
    }
}

/// Progress indicator component that shows the visual progress bar
#[component]
pub fn ProgressIndicator(#[prop(into, optional)] class: MaybeProp<String>) -> impl IntoView {
    // Get context with fallback to prevent hydration panics (following Leptix pattern)
    let ProgressContextValue { max, value } =
        use_context().unwrap_or_else(|| ProgressContextValue {
            value: Signal::derive(|| None),
            max: Signal::derive(|| DEFAULT_MAX),
        });

    view! {
        <div
            class=move || {
                let mut class_str = String::from("bg-white w-full h-full transition-transform duration-[660ms] ease-[cubic-bezier(0.65,0,0.35,1)]");
                if let Some(custom_class) = class.get() {
                    class_str.push(' ');
                    class_str.push_str(&custom_class);
                }
                class_str
            }
            style=move || {
                let percentage = value.get()
                    .map(|v| (v / max.get()) * 100.0)
                    .unwrap_or(0.0);
                format!("transform: translateX(-{}%)", 100.0 - percentage)
            }
            data-state=move || {
                value.get().map(|v| {
                    if v >= max.get() { "complete" } else { "loading" }
                }).unwrap_or("indeterminate")
            }
            data-value=move || value.get()
        />
    }
}
