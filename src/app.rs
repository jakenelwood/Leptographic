use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};

#[cfg(feature = "ssr")]
use leptos::hydration::{AutoReload, HydrationScripts};

use crate::themes::ThemeProvider;
use crate::demo_page::DemoPage;

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
                <ThemeProvider>
                    <App/>
                </ThemeProvider>
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

        // Phase IV Component Stylesheets
        <Stylesheet id="checkbox" href="/styles/checkbox.css"/>
        <Stylesheet id="label" href="/styles/label.css"/>
        <Stylesheet id="progress" href="/styles/progress.css"/>
        <Stylesheet id="separator" href="/styles/separator.css"/>
        <Stylesheet id="switch" href="/styles/switch.css"/>

        // sets the document title
        <Title text="Leptos Radix UI - Live Demo"/>

        // Professional demo page
        <DemoPage />
    }
}




