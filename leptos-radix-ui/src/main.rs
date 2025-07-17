#[cfg(feature = "ssr")]
use axum::Router;
#[cfg(feature = "ssr")]
use leptos::hydration::{AutoReload, HydrationScripts};
#[cfg(feature = "ssr")]
use leptos::prelude::{
    get_configuration, use_context, ElementChild, GlobalAttributes, LeptosOptions,
};
#[cfg(feature = "ssr")]
use leptos::{component, view, IntoView};
#[cfg(feature = "ssr")]
use leptos_axum::{generate_route_list, LeptosRoutes};
#[cfg(feature = "ssr")]
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
#[cfg(feature = "ssr")]
use leptos_radix_ui::App;
#[cfg(feature = "ssr")]
use tokio::net::TcpListener;

// 🔥 THIS IS THE FIX 🔥
// Create a new parameter-less shell component that gets its options from context.
#[cfg(feature = "ssr")]
#[component]
fn AppShell() -> impl IntoView {
    // This line gets the options that were provided by .with_state() in main()
    let options = use_context::<LeptosOptions>().expect("LeptosOptions to be provided");
    provide_meta_context();

    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <Title text="Leptonic UI Components"/>
                <Stylesheet id="leptos" href="/pkg/leptos-radix-ui.css?v=2"/>

                // These components now get the options they need from the context
                <AutoReload options=options.clone() />
                <HydrationScripts options=options.clone() />

                // 🚨 REMOVED MANUAL SCRIPT 🚨
                // <HydrationScripts/> is the correct, idiomatic way to do this.
                // The manual script can cause double-hydration issues.

                <MetaTags/>
            </head>
            <body>
                // Your <App/> component is rendered here, inside the full HTML document
                <App/>
            </body>
        </html>
    }
}

// Wrapper function for the fallback handler that still expects LeptosOptions parameter
#[cfg(feature = "ssr")]
pub fn shell_wrapper(options: LeptosOptions) -> impl IntoView {
    // We need to provide the options to context manually for the fallback
    leptos::prelude::provide_context(options);
    AppShell()
}

#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    let conf = get_configuration(None).unwrap();
    let addr = conf.leptos_options.site_addr;
    let routes = generate_route_list(App);

    // build our application with a route
    let app = Router::new()
        // 👇 Use the new AppShell component here instead of App
        .leptos_routes(&conf.leptos_options, routes, AppShell)
        // 👇 Use the wrapper function for the fallback handler
        .fallback(leptos_axum::file_and_error_handler(shell_wrapper))
        .with_state(conf.leptos_options);

    let listener = TcpListener::bind(&addr).await.unwrap();
    println!("listening on http://{}", &addr);
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

#[cfg(not(any(feature = "ssr", feature = "hydrate")))]
pub fn main() {
    // no client-side main function unless we want to run the server
}
