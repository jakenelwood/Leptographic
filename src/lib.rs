#![forbid(unsafe_code)]

pub mod components;
pub mod hooks;
pub mod utils;

pub use components::*;
pub use hooks::CheckedState;
use leptos::prelude::*;

/// Theme context for light/dark mode
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Theme {
    Light,
    Dark,
}

/// SVG CheckIcon component - Professional checkmark icon
#[component]
pub fn CheckIconSvg() -> impl IntoView {
    use leptos::svg::{path, svg};

    svg()
        .attr("width", "19")
        .attr("height", "19")
        .attr("viewBox", "0 0 15 15")
        .attr("fill", "none")
        .attr("xmlns", "http://www.w3.org/2000/svg")
        .attr("class", "text-black")
        .child(
            path()
                .attr("d", "M11.4669 3.72684C11.7558 3.91574 11.8369 4.30308 11.648 4.59198L7.39799 11.092C7.29783 11.2452 7.13556 11.3467 6.95402 11.3699C6.77247 11.3931 6.58989 11.3355 6.45446 11.2124L3.70446 8.71241C3.44905 8.48022 3.43023 8.08494 3.66242 7.82953C3.89461 7.57412 4.28989 7.55529 4.5453 7.78749L6.75292 9.79441L10.6018 3.90792C10.7907 3.61902 11.178 3.53795 11.4669 3.72684Z")
                .attr("fill", "currentColor")
                .attr("fill-rule", "evenodd")
                .attr("clip-rule", "evenodd")
        )
}

/// Theme toggle button
#[component]
pub fn ThemeToggle(theme: RwSignal<Theme>) -> impl IntoView {
    view! {
        <button
            class="text-lg cursor-pointer transition-opacity hover:opacity-70"
            on:click=move |_| {
                theme.update(|t| *t = match *t {
                    Theme::Light => Theme::Dark,
                    Theme::Dark => Theme::Light,
                });
            }
        >
            {move || match theme.get() {
                Theme::Light => "🌙",
                Theme::Dark => "☀️",
            }}
        </button>
    }
}

// Define the shared App component that will be used by both SSR and hydration
#[component]
pub fn App() -> impl IntoView {
    leptos::logging::log!("🎯 App component rendering...");

    // Theme state
    let theme = RwSignal::new(Theme::Dark);

    view! {
        <div class=move || format!("min-h-screen transition-colors duration-200 {}",
            match theme.get() {
                Theme::Light => "bg-white text-gray-900",
                Theme::Dark => "bg-dark-bg text-white",
            }
        )>
            // Header with title and theme toggle
            <header class=move || format!("px-4 py-4 sm:px-6 sm:py-6 {}",
                match theme.get() {
                    Theme::Light => "bg-white",
                    Theme::Dark => "bg-dark-bg",
                }
            )>
                <div class="w-full flex flex-col sm:flex-row justify-between items-start sm:items-center gap-4">
                    <div class="flex-1">
                        <h1 class="text-2xl sm:text-3xl font-bold mb-2">
                            "Leptographic"
                        </h1>
                        <p class=move || format!("text-xs sm:text-sm {}",
                            match theme.get() {
                                Theme::Light => "text-gray-600",
                                Theme::Dark => "text-gray-400",
                            }
                        )>
                            "A Leptos UI system with Switch, Progress, and Separator components - styled with Tailwind CSS 4."
                        </p>
                    </div>
                    <div class="flex-shrink-0">
                        <ThemeToggle theme=theme />
                    </div>
                </div>
            </header>

            // Main content with five swimlanes
            <main class="w-full px-4 py-2">
                <ComponentShowcase theme=theme />
            </main>
        </div>
    }
}

/// Five-column component showcase inspired by Ant Design
#[component]
fn ComponentShowcase(theme: RwSignal<Theme>) -> impl IntoView {
    view! {
        <div class="flex min-h-screen">
            // Swimlane 1: Component Names (always protected)
            <div class=move || format!("w-48 flex-shrink-0 p-2 {}",
                match theme.get() {
                    Theme::Light => "bg-white",
                    Theme::Dark => "bg-dark-bg",
                }
            )>
                <h3 class=move || format!("font-normal mb-3 text-sm uppercase tracking-wider opacity-60 {}",
                    match theme.get() {
                        Theme::Light => "text-gray-700",
                        Theme::Dark => "text-gray-400",
                    }
                )>
                    "Components"
                </h3>
                <div class="space-y-1">
                    <ComponentNavItem name="Checkbox" active=true theme=theme />
                    <ComponentNavItem name="Switch" active=false theme=theme />
                    <ComponentNavItem name="Progress" active=false theme=theme />
                    <ComponentNavItem name="Separator" active=false theme=theme />
                </div>
            </div>

            // Component Cards Container
            <div class="flex-1 grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-3 p-2">
                // Checkbox Component
                <div>
                    <ComponentCard title="Checkbox" theme=theme>
                        <CheckboxShowcase _theme=theme />
                    </ComponentCard>
                </div>

                // Switch Placeholder
                <div>
                    <ComponentCard title="Switch" theme=theme>
                        <div class="text-center text-gray-300 text-base">
                            "Coming Soon"
                        </div>
                    </ComponentCard>
                </div>

                // Progress Placeholder
                <div>
                    <ComponentCard title="Progress" theme=theme>
                        <div class="text-center text-gray-300 text-base">
                            "Coming Soon"
                        </div>
                    </ComponentCard>
                </div>

                // Separator Placeholder
                <div>
                    <ComponentCard title="Separator" theme=theme>
                        <div class="text-center text-gray-300 text-base">
                            "Coming Soon"
                        </div>
                    </ComponentCard>
                </div>
            </div>
        </div>
    }
}

/// Navigation item for component list
#[component]
fn ComponentNavItem(name: &'static str, active: bool, theme: RwSignal<Theme>) -> impl IntoView {
    view! {
        <div class=move || format!("px-2 py-1 text-sm cursor-pointer transition-colors tracking-wide {}",
            if active {
                match theme.get() {
                    Theme::Light => "text-gray-900 font-normal",
                    Theme::Dark => "text-white font-normal",
                }
            } else {
                match theme.get() {
                    Theme::Light => "text-gray-500 hover:text-[#605ED6] font-light",
                    Theme::Dark => "text-gray-500 hover:text-[#605ED6] font-light",
                }
            }
        )>
            {name}
        </div>
    }
}

/// Component card wrapper
#[component]
fn ComponentCard(
    title: &'static str,
    theme: RwSignal<Theme>,
    children: ChildrenFn,
) -> impl IntoView {
    // GitHub code links for each component
    let github_url = match title {
        "Checkbox" => {
            "https://github.com/jakenelwood/Leptographic/blob/main/src/components/checkbox.rs"
        }
        "Switch" => {
            "https://github.com/jakenelwood/Leptographic/blob/main/src/components/switch.rs"
        }
        "Progress" => {
            "https://github.com/jakenelwood/Leptographic/blob/main/src/components/progress.rs"
        }
        "Separator" => {
            "https://github.com/jakenelwood/Leptographic/blob/main/src/components/separator.rs"
        }
        _ => "https://github.com/jakenelwood/Leptographic/tree/main/src/components",
    };

    view! {
        <div class=move || format!("rounded border bg-[#605ED6] w-5/6 h-40 sm:h-44 lg:h-48 mx-auto overflow-hidden {}",
            match theme.get() {
                Theme::Light => "border-[#dedede]",
                Theme::Dark => "border-white",
            }
        )>
            // Title section with Code link
            <div class=move || format!("px-3 py-2 border-b relative {}",
                match theme.get() {
                    Theme::Light => "border-[#dedede]",
                    Theme::Dark => "border-white",
                }
            )>
                <h3 class="font-normal text-sm sm:text-base text-white tracking-wide">{title}</h3>
                // Code link in upper right corner
                <a
                    href=github_url
                    target="_blank"
                    rel="noopener noreferrer"
                    class="absolute top-2 right-3 text-xs text-white hover:text-gray-200 transition-colors cursor-pointer"
                >
                    "Code"
                </a>
            </div>
            // Component showcase area
            <div class="flex-1 flex items-center justify-center p-4 sm:p-5 lg:p-6">
                {children()}
            </div>
        </div>
    }
}

/// Checkbox component showcase
#[component]
fn CheckboxShowcase(_theme: RwSignal<Theme>) -> impl IntoView {
    view! {
        <div class="flex items-center space-x-3">
            <Checkbox id="demo-checkbox-1">
                <CheckboxIndicator>
                    <CheckIconSvg />
                </CheckboxIndicator>
            </Checkbox>
            <label for="demo-checkbox-1" class="cursor-pointer text-white text-base">
                "Accept terms"
            </label>
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
