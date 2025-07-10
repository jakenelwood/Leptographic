use leptos::prelude::*;
use leptos::context::Provider;

/// Simple theme modes for component testing
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ThemeMode {
    Light,
    Dark,
}

/// Simple theme context for component testing
#[derive(Clone, Copy)]
pub struct ThemeContext {
    pub mode: RwSignal<ThemeMode>,
}

impl ThemeContext {
    pub fn new() -> Self {
        Self {
            mode: RwSignal::new(ThemeMode::Light),
        }
    }

    pub fn toggle(&self) {
        let current = self.mode.get();
        let new_mode = match current {
            ThemeMode::Light => ThemeMode::Dark,
            ThemeMode::Dark => ThemeMode::Light,
        };
        self.mode.set(new_mode);
    }
}

/// Simple theme provider for component testing
#[component]
pub fn ThemeProvider(children: ChildrenFn) -> impl IntoView {
    let theme_context = ThemeContext::new();

    view! {
        <Provider value=theme_context>
            {children()}
        </Provider>
    }
}

/// Hook to access theme context
pub fn use_theme() -> ThemeContext {
    use_context::<ThemeContext>().unwrap_or_else(|| ThemeContext::new())
}
