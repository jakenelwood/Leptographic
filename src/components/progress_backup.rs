use leptos::context::Provider;
use leptos::prelude::*;

const DEFAULT_MAX: f64 = 100.0;

/// Context value - SIMPLIFIED like Leptix (following blueprint "Keep It Simple")
#[derive(Clone)]
struct ProgressContextValue {
    value: Signal<Option<f64>>,
    max: Signal<f64>,
}

/// Progress component - SIMPLIFIED following blueprint "Keep It Simple" principle
/// Direct signal derivation like Leptix - no complex hooks
#[component]
pub fn Progress(
    #[prop(into, optional)] value: MaybeProp<f64>,
    #[prop(into, optional)] max: MaybeProp<f64>,
    #[prop(into, optional)] class: MaybeProp<String>,
    children: ChildrenFn,
) -> impl IntoView {
    // ✅ SIMPLE: Direct signal derivation like Leptix (following blueprint)
    let max = Signal::derive(move || {
        let max_val = max.get().unwrap_or(DEFAULT_MAX);
        if max_val > 0.0 {
            max_val
        } else {
            DEFAULT_MAX
        }
    });

    let value = Signal::derive(move || {
        value.get().and_then(|v| {
            let max_val = max.get();
            if v >= 0.0 && v <= max_val {
                Some(v)
            } else {
                None
            }
        })
    });

    // Create context value like Leptix
    let context_value = ProgressContextValue { value, max };

    view! {
        <Provider value=context_value>
            <div
                role="progressbar"
                aria-valuemax=move || max.get()
                aria-valuemin="0"
                aria-valuenow=move || value.get()
                data-state=move || {
                    value.get().map(|v| {
                        if v >= max.get() { "complete" } else { "loading" }
                    }).unwrap_or("indeterminate")
                }
                data-value=move || value.get()
                data-max=move || max.get()
                class=move || format!(
                    "relative h-3 w-full overflow-hidden rounded-full bg-gray-200 dark:bg-gray-700 shadow-inner {}",
                    class.get().unwrap_or_default()
                )
            >
                {children()}
            </div>
        </Provider>
    }
}

/// ProgressIndicator - Production-ready with beautiful Tailwind styling
#[component]
pub fn ProgressIndicator(#[prop(into, optional)] class: MaybeProp<String>) -> impl IntoView {
    // Use hydration-safe context access like our blueprint
    let context = use_context::<ProgressContextValue>();

    // Extract values with fallbacks like Switch/Checkbox do
    let (state_str, value_str, max_str, width_style) = if let Some(ctx) = context {
        (
            Signal::derive(move || {
                ctx.value
                    .get()
                    .map(|v| {
                        if v >= ctx.max.get() {
                            "complete".to_string()
                        } else {
                            "loading".to_string()
                        }
                    })
                    .unwrap_or("indeterminate".to_string())
            }),
            Signal::derive(move || ctx.value.get()),
            Signal::derive(move || ctx.max.get()),
            Signal::derive(move || {
                if let Some(v) = ctx.value.get() {
                    let percentage = (v / ctx.max.get()) * 100.0;
                    format!("width: {}%", percentage)
                } else {
                    "width: 0%".to_string()
                }
            }),
        )
    } else {
        // Fallback values for hydration safety
        (
            Signal::derive(move || "indeterminate".to_string()),
            Signal::derive(move || None::<f64>),
            Signal::derive(move || 100.0),
            Signal::derive(move || "width: 0%".to_string()),
        )
    };

    view! {
        <div
            data-state=move || state_str.get()
            data-value=move || value_str.get()
            data-max=move || max_str.get()
            style=move || width_style.get()
            class=move || format!(
                "h-full bg-gradient-to-r from-blue-500 to-blue-600 transition-all duration-500 ease-out rounded-full shadow-sm {}",
                class.get().unwrap_or_default()
            )
        />
    }
}
