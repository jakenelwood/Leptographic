#![forbid(unsafe_code)]

pub mod components;
pub mod utils;

pub use components::*;
use leptos::prelude::*;

/// SVG CheckIcon component - Professional checkmark icon
#[component]
pub fn CheckIconSvg() -> impl IntoView {
    use leptos::svg::{path, svg};

    svg()
        .attr("width", "15")
        .attr("height", "15")
        .attr("viewBox", "0 0 15 15")
        .attr("fill", "none")
        .attr("xmlns", "http://www.w3.org/2000/svg")
        .child(
            path()
                .attr("d", "M11.4669 3.72684C11.7558 3.91574 11.8369 4.30308 11.648 4.59198L7.39799 11.092C7.29783 11.2452 7.13556 11.3467 6.95402 11.3699C6.77247 11.3931 6.58989 11.3355 6.45446 11.2124L3.70446 8.71241C3.44905 8.48022 3.43023 8.08494 3.66242 7.82953C3.89461 7.57412 4.28989 7.55529 4.5453 7.78749L6.75292 9.79441L10.6018 3.90792C10.7907 3.61902 11.178 3.53795 11.4669 3.72684Z")
                .attr("fill", "currentColor")
                .attr("fill-rule", "evenodd")
                .attr("clip-rule", "evenodd")
        )
}

// Define the shared App component that will be used by both SSR and hydration
#[component]
pub fn App() -> impl IntoView {
    leptos::logging::log!("🎯 App component rendering...");

    // Test button to verify Leptos event system is working
    let test_click = move |_| {
        leptos::logging::log!("🧪 TEST BUTTON CLICKED - Leptos events are working!");
    };

    view! {
        <div class="min-h-screen bg-gray-900 text-white p-8">
            <div class="max-w-4xl mx-auto">
                <h1 class="text-4xl font-bold mb-8 text-center">
                    "Leptonic UI Components"
                </h1>
                <p class="text-lg text-gray-300 mb-12 text-center">
                    "A Leptographic (defn.): A UI design system created to satisfy Leptos frontend needs"
                </p>

                <div class="text-6xl text-red-500 font-bold mb-8 text-center">
                    "🔥 HELLO WORLD - CHANGES ARE WORKING! 🔥"
                </div>

                // Test button to verify Leptos event system
                <div class="mb-6 text-center">
                    <button
                        class="bg-blue-600 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded"
                        on:click=test_click
                    >
                        "🧪 Test Leptos Events"
                    </button>
                </div>

                <div class="space-y-12">
                    <CheckboxDemo />
                </div>
            </div>
        </div>
    }
}

#[component]
fn CheckboxDemo() -> impl IntoView {
    view! {
        <div class="bg-gray-800 rounded-lg p-8">
            <h2 class="text-2xl font-bold mb-6">"Checkbox"</h2>
            <div class="bg-gray-700 rounded-lg p-8 space-y-6">
                <div class="flex items-center space-x-4">
                    <Checkbox id="demo-checkbox-1">
                        <CheckboxIndicator>
                            <CheckIconSvg />
                        </CheckboxIndicator>
                    </Checkbox>
                    <label for="demo-checkbox-1" class="text-white cursor-pointer text-xl font-medium">
                        "Accept terms and conditions"
                    </label>
                </div>

                <div class="flex items-center space-x-4">
                    <Checkbox id="demo-checkbox-2" checked=CheckedState::True>
                        <CheckboxIndicator>
                            <CheckIconSvg />
                        </CheckboxIndicator>
                    </Checkbox>
                    <label for="demo-checkbox-2" class="text-white cursor-pointer text-xl font-medium">
                        "Pre-checked example"
                    </label>
                </div>

                <div class="flex items-center space-x-4">
                    <Checkbox id="demo-checkbox-3" disabled=true>
                        <CheckboxIndicator>
                            <CheckIconSvg />
                        </CheckboxIndicator>
                    </Checkbox>
                    <label for="demo-checkbox-3" class="text-gray-400 cursor-not-allowed text-xl font-medium">
                        "Disabled checkbox"
                    </label>
                </div>


            </div>
        </div>
    }
}

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use leptos::mount::hydrate_body;

    console_error_panic_hook::set_once();
    leptos::logging::log!("🚀 WASM hydrate() function called!");
    leptos::logging::log!("🔧 About to hydrate body...");
    hydrate_body(App);
    leptos::logging::log!("✅ Body hydrated successfully!");
}
