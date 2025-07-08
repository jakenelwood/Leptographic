use leptos::prelude::*;
use leptos::context::Provider;

// Context for sharing dialog state between components
#[derive(Clone, Copy)]
struct DialogContext {
    open: RwSignal<bool>,
}

/// Root Dialog component that provides context for all dialog parts
#[component]
pub fn Dialog(
    /// Child components
    children: ChildrenFn,
) -> impl IntoView {
    let open = RwSignal::new(false);

    let context_value = DialogContext { open };

    view! {
        <Provider value=context_value>
            {children()}
        </Provider>
    }
}

/// Trigger element that opens the dialog
#[component]
pub fn DialogTrigger(
    /// Child components
    children: ChildrenFn,
) -> impl IntoView {
    let context = expect_context::<DialogContext>();

    let handle_click = move |_| {
        context.open.set(true);
    };

    view! {
        <button on:click=handle_click>
            {children()}
        </button>
    }
}

/// Portal container for dialog content
#[component]
pub fn DialogPortal(
    /// Child components
    children: ChildrenFn,
) -> impl IntoView {
    // For now, render in place (TODO: implement proper portal)
    children()
}

/// Background overlay for the dialog
#[component]
pub fn DialogOverlay(
    /// Child components
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView {
    let context = expect_context::<DialogContext>();

    let handle_click = move |_| {
        context.open.set(false);
    };

    view! {
        <Show when=move || context.open.get()>
            <div
                on:click=handle_click
                style="position: fixed; inset: 0; z-index: 50;"
            >
                {children.as_ref().map(|children| children())}
            </div>
        </Show>
    }
}

/// Main dialog content container
#[component]
pub fn DialogContent(
    /// Child components
    children: ChildrenFn,
) -> impl IntoView {
    let context = expect_context::<DialogContext>();

    view! {
        <Show when=move || context.open.get()>
            <div
                role="dialog"
                aria-modal="true"
            >
                {children()}
            </div>
        </Show>
    }
}

/// Dialog title component (required for accessibility)
#[component]
pub fn DialogTitle(
    /// Child components
    children: ChildrenFn,
) -> impl IntoView {
    view! {
        <h2>{children()}</h2>
    }
}

/// Dialog description component (optional)
#[component]
pub fn DialogDescription(
    /// Child components
    children: ChildrenFn,
) -> impl IntoView {
    view! {
        <p>{children()}</p>
    }
}

/// Close button component
#[component]
pub fn DialogClose(
    /// Child components
    children: ChildrenFn,
) -> impl IntoView {
    let context = expect_context::<DialogContext>();

    let handle_click = move |_| {
        context.open.set(false);
    };

    view! {
        <button on:click=handle_click>
            {children()}
        </button>
    }
}
