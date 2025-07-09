use leptos::prelude::*;
use leptos::context::Provider;


#[cfg(feature = "hydrate")]
use wasm_bindgen_futures;
#[cfg(feature = "hydrate")]
use web_sys;
#[cfg(feature = "hydrate")]
use gloo_timers;

/// Theme modes supported by the application
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ThemeMode {
    Light,
    Dark,
    System,
}

impl ThemeMode {
    pub fn as_str(&self) -> &'static str {
        match self {
            ThemeMode::Light => "light",
            ThemeMode::Dark => "dark",
            ThemeMode::System => "system",
        }
    }
    
    pub fn from_str(s: &str) -> Self {
        match s {
            "dark" => ThemeMode::Dark,
            "system" => ThemeMode::System,
            _ => ThemeMode::Light,
        }
    }
}

/// Theme context for sharing theme state throughout the app
#[derive(Clone, Copy)]
pub struct ThemeContext {
    pub mode: RwSignal<ThemeMode>,
    pub resolved_theme: Signal<ThemeMode>,
}

impl ThemeContext {
    pub fn new() -> Self {
        let mode = RwSignal::new(ThemeMode::System);
        
        // Resolve the actual theme based on system preference
        let resolved_theme = Signal::derive(move || {
            match mode.get() {
                ThemeMode::System => {
                    // Check system preference
                    if is_dark_mode_preferred() {
                        ThemeMode::Dark
                    } else {
                        ThemeMode::Light
                    }
                }
                other => other,
            }
        });
        
        Self {
            mode,
            resolved_theme,
        }
    }
    
    pub fn toggle(&self) {
        #[cfg(feature = "hydrate")]
        {
            web_sys::console::log_1(&"Theme toggle clicked!".into());
        }

        let current = self.mode.get();
        let new_mode = match current {
            ThemeMode::Light => ThemeMode::Dark,
            ThemeMode::Dark => ThemeMode::Light,
            ThemeMode::System => {
                // If system, toggle to opposite of current resolved theme
                match self.resolved_theme.get() {
                    ThemeMode::Dark => ThemeMode::Light,
                    _ => ThemeMode::Dark,
                }
            }
        };
        self.mode.set(new_mode);
        self.save_to_storage();
        self.apply_theme();
    }
    
    pub fn set_mode(&self, new_mode: ThemeMode) {
        self.mode.set(new_mode);
        self.save_to_storage();
        self.apply_theme();
    }
    
    fn save_to_storage(&self) {
        #[cfg(feature = "hydrate")]
        {
            if let Some(window) = web_sys::window() {
                if let Ok(Some(storage)) = window.local_storage() {
                    let _ = storage.set_item("theme", self.mode.get().as_str());
                }
            }
        }
    }
    
    fn load_from_storage(&self) {
        #[cfg(feature = "hydrate")]
        {
            if let Some(window) = web_sys::window() {
                if let Ok(Some(storage)) = window.local_storage() {
                    if let Ok(Some(stored_theme)) = storage.get_item("theme") {
                        self.mode.set(ThemeMode::from_str(&stored_theme));
                    }
                }
            }
        }
    }
    
    fn apply_theme(&self) {
        #[cfg(feature = "hydrate")]
        {
            if let Some(document) = web_sys::window().and_then(|w| w.document()) {
                if let Some(html) = document.document_element() {
                    let class_list = html.class_list();

                    // Add theme-switching class to prevent transitions
                    let _ = class_list.add_1("theme-switching");

                    // Remove existing theme classes
                    let _ = class_list.remove_2("light", "dark");

                    // Add new theme class
                    let theme_class = match self.resolved_theme.get() {
                        ThemeMode::Dark => "dark",
                        _ => "light",
                    };
                    let _ = class_list.add_1(theme_class);

                    // Remove theme-switching class after a brief delay
                    let class_list_clone = class_list.clone();
                    wasm_bindgen_futures::spawn_local(async move {
                        gloo_timers::future::TimeoutFuture::new(50).await;
                        let _ = class_list_clone.remove_1("theme-switching");
                    });
                }
            }
        }
    }
}

/// Check if the user prefers dark mode
fn is_dark_mode_preferred() -> bool {
    #[cfg(feature = "hydrate")]
    {
        if let Some(window) = web_sys::window() {
            if let Ok(Some(media_query)) = window.match_media("(prefers-color-scheme: dark)") {
                return media_query.matches();
            }
        }
        false
    }
    #[cfg(not(feature = "hydrate"))]
    {
        false
    }
}

/// Theme Provider component that provides theme context to children
#[component]
pub fn ThemeProvider(
    /// Initial theme mode (defaults to System)
    #[prop(optional)] initial_mode: Option<ThemeMode>,
    /// Child components
    children: ChildrenFn,
) -> impl IntoView {
    let theme_context = ThemeContext::new();
    
    // Set initial mode if provided
    if let Some(mode) = initial_mode {
        theme_context.set_mode(mode);
    } else {
        // Load from storage on mount
        theme_context.load_from_storage();
    }
    
    // Apply theme on mount and when resolved theme changes
    Effect::new(move |_| {
        theme_context.apply_theme();
    });
    
    view! {
        <Provider value=theme_context>
            {children()}
        </Provider>
    }
}

/// Hook to access theme context
pub fn use_theme() -> ThemeContext {
    expect_context::<ThemeContext>()
}

/// Theme toggle button component
#[component]
pub fn ThemeToggle(
    /// Optional class name for styling
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    let theme = use_theme();
    
    let button_text = Signal::derive(move || {
        match theme.resolved_theme.get() {
            ThemeMode::Dark => "☀️",
            _ => "🌙",
        }
    });
    
    let title_text = Signal::derive(move || {
        match theme.resolved_theme.get() {
            ThemeMode::Dark => "Switch to light mode",
            _ => "Switch to dark mode",
        }
    });
    
    view! {
        <button
            class=class.unwrap_or_default()
            title=title_text
            on:click=move |_| theme.toggle()
        >
            {button_text}
        </button>
    }
}
