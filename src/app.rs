use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment,
};
use crate::components::dialog::*;
use crate::components::checkbox::*;

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <AutoReload options=options.clone() />
                <HydrationScripts options/>
                <MetaTags/>
            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/leptos-radix-ui.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <Router>
            <main>
                <Routes fallback=|| "Page not found.".into_view()>
                    <Route path=StaticSegment("") view=HomePage/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    // Creates a reactive value to update the button
    let count = RwSignal::new(0);
    let on_click = move |_| *count.write() += 1;

    view! {
        <div class="p-8 space-y-8">
            <h1 class="text-4xl font-bold text-blue-600">"Welcome to Leptos Radix UI!"</h1>

            <div class="space-y-4">
                <h2 class="text-2xl font-semibold">"Counter Example"</h2>
                <button
                    class="bg-red-500 hover:bg-red-700 text-white font-bold py-2 px-4 rounded"
                    on:click=on_click
                >
                    "Click Me: " {count}
                </button>
            </div>

            <div class="space-y-4">
                <h2 class="text-2xl font-semibold">"Checkbox Example"</h2>
                <div class="space-y-2">
                    <Checkbox>
                        <CheckboxIndicator>
                            "✓"
                        </CheckboxIndicator>
                        " Accept terms and conditions"
                    </Checkbox>

                    <Checkbox default_checked=CheckedState::True>
                        <CheckboxIndicator>
                            "✓"
                        </CheckboxIndicator>
                        " Subscribe to newsletter"
                    </Checkbox>

                    <Checkbox default_checked=CheckedState::Indeterminate>
                        <CheckboxIndicator>
                            "−"
                        </CheckboxIndicator>
                        " Indeterminate state"
                    </Checkbox>
                </div>
            </div>

            <div class="space-y-4">
                <h2 class="text-2xl font-semibold">"Dialog Example"</h2>
                <Dialog>
                    <DialogTrigger>
                        "Open Dialog"
                    </DialogTrigger>

                    <DialogPortal>
                        <DialogOverlay />
                        <DialogContent>
                            <DialogTitle>
                                "Dialog Title"
                            </DialogTitle>
                            <DialogDescription>
                                "This is a dialog description. It explains what the dialog is for."
                            </DialogDescription>
                            <div>
                                <DialogClose>
                                    "Cancel"
                                </DialogClose>
                                <DialogClose>
                                    "Confirm"
                                </DialogClose>
                            </div>
                        </DialogContent>
                    </DialogPortal>
                </Dialog>
            </div>
        </div>
    }
}
