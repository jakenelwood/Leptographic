use leptos::prelude::*;
use crate::components::checkbox::*;
use crate::components::switch::*;
use crate::components::progress::*;

#[derive(Debug, Clone, PartialEq)]
enum Theme {
    Light,
    Dark,
}

#[component]
pub fn ComponentTestPage() -> impl IntoView {
    // Progress value for interactive testing
    let (progress_value, set_progress_value) = signal(50.0);

    // Theme toggle using basic Leptos signals
    let (theme, set_theme) = signal(Theme::Light);

    let toggle_theme = move |_| {
        match theme.get() {
            Theme::Light => set_theme.set(Theme::Dark),
            Theme::Dark => set_theme.set(Theme::Light),
        }
    };

    let theme_button_text = move || {
        match theme.get() {
            Theme::Dark => "☀️",
            Theme::Light => "🌙",
        }
    };

    // Effect to update HTML class when theme changes
    Effect::new(move |_| {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let html = document.document_element().unwrap();
        let class_list = html.class_list();

        match theme.get() {
            Theme::Dark => {
                let _ = class_list.add_1("dark");
                let _ = class_list.remove_1("light");
            }
            Theme::Light => {
                let _ = class_list.add_1("light");
                let _ = class_list.remove_1("dark");
            }
        }
    });

    view! {
        <div style="padding: 2rem; font-family: sans-serif; max-width: 800px; margin: 0 auto;">
            // Add comprehensive dark mode styles
            <style>
                "
                body { transition: background-color 0.3s ease, color 0.3s ease; }
                .dark body { background-color: #1a1a1a; color: #ffffff; }
                .dark h1 { color: #ffffff !important; }
                .dark h2 { color: #374151 !important; }
                .dark p { color: #6b7280 !important; }
                .dark label { color: #374151 !important; }
                .dark section {
                    background: #bfbfbf !important;
                    border-color: #d1d5db !important;
                    color: #374151 !important;
                }
                .dark .theme-toggle { background: #374151 !important; color: #ffffff !important; border-color: #6b7280 !important; }
                "
            </style>

            // Header with theme toggle
            <div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 2rem;">
                <h1 style="color: #333; margin: 0;">"Leptos Radix UI - Component Test"</h1>
                <button
                    class="theme-toggle"
                    style="padding: 0.5rem; background: #f3f4f6; border: 1px solid #d1d5db; border-radius: 6px; cursor: pointer; font-size: 1.2rem; transition: all 0.2s ease;"
                    title=move || match theme.get() {
                        Theme::Dark => "Switch to light mode",
                        Theme::Light => "Switch to dark mode",
                    }
                    on:click=toggle_theme
                >
                    {theme_button_text}
                </button>
            </div>

            // Checkbox Tests
            <section style="margin: 2rem 0; padding: 1.5rem; border: 2px solid #e5e7eb; border-radius: 8px; background: #f9fafb;">
                <h2 style="color: #374151; margin-bottom: 1rem;">"✅ Checkbox Component"</h2>

                <div style="display: flex; flex-direction: column; gap: 1rem;">
                    <div style="display: flex; align-items: center; gap: 0.5rem;">
                        <Checkbox>
                            <CheckboxIndicator>
                                "✓"
                            </CheckboxIndicator>
                        </Checkbox>
                        <label style="color: #374151;">"Basic Checkbox (Unchecked)"</label>
                    </div>

                    <div style="display: flex; align-items: center; gap: 0.5rem;">
                        <Checkbox default_checked=CheckedState::True>
                            <CheckboxIndicator>
                                "✓"
                            </CheckboxIndicator>
                        </Checkbox>
                        <label style="color: #374151;">"Checked by Default"</label>
                    </div>

                    <div style="display: flex; align-items: center; gap: 0.5rem;">
                        <Checkbox default_checked=CheckedState::Indeterminate>
                            <CheckboxIndicator>
                                "−"
                            </CheckboxIndicator>
                        </Checkbox>
                        <label style="color: #374151;">"Indeterminate State"</label>
                    </div>

                    <div style="display: flex; align-items: center; gap: 0.5rem;">
                        <Checkbox disabled=true>
                            <CheckboxIndicator>
                                "✓"
                            </CheckboxIndicator>
                        </Checkbox>
                        <label style="color: #9ca3af;">"Disabled Checkbox"</label>
                    </div>
                </div>
            </section>

            // Switch Tests
            <section style="margin: 2rem 0; padding: 1.5rem; border: 2px solid #e5e7eb; border-radius: 8px; background: #f9fafb;">
                <h2 style="color: #374151; margin-bottom: 1rem;">"🔄 Switch Component"</h2>

                <div style="display: flex; flex-direction: column; gap: 1rem;">
                    <div style="display: flex; align-items: center; gap: 0.5rem;">
                        <Switch>
                            <SwitchThumb />
                        </Switch>
                        <label style="color: #374151;">"Basic Switch (Off)"</label>
                    </div>

                    <div style="display: flex; align-items: center; gap: 0.5rem;">
                        <Switch default_checked=true>
                            <SwitchThumb />
                        </Switch>
                        <label style="color: #374151;">"Default Checked"</label>
                    </div>

                    <div style="display: flex; align-items: center; gap: 0.5rem;">
                        <Switch disabled=true>
                            <SwitchThumb />
                        </Switch>
                        <label style="color: #9ca3af;">"Disabled Switch"</label>
                    </div>
                </div>
            </section>

            // Progress Tests
            <section style="margin: 2rem 0; padding: 1.5rem; border: 2px solid #e5e7eb; border-radius: 8px; background: #f9fafb;">
                <h2 style="color: #374151; margin-bottom: 1rem;">"📊 Progress Component"</h2>

                <div style="display: flex; flex-direction: column; gap: 1.5rem;">
                    <div>
                        <label style="color: #374151; display: block; margin-bottom: 0.5rem;">"Static Progress (75%)"</label>
                        <Progress value=Signal::from(75.0)>
                            <ProgressIndicator />
                        </Progress>
                    </div>

                    <div>
                        <label style="color: #374151; display: block; margin-bottom: 0.5rem;">
                            "Interactive Progress (" {move || format!("{:.0}%", progress_value.get())} ")"
                        </label>
                        <Progress value=progress_value>
                            <ProgressIndicator />
                        </Progress>
                        <div style="margin-top: 0.5rem; display: flex; gap: 0.5rem;">
                            <button
                                style="padding: 0.25rem 0.5rem; background: #3b82f6; color: white; border: none; border-radius: 4px; cursor: pointer;"
                                on:click=move |_| set_progress_value.update(|v| *v = (*v + 10.0).min(100.0))
                            >
                                "+10%"
                            </button>
                            <button
                                style="padding: 0.25rem 0.5rem; background: #ef4444; color: white; border: none; border-radius: 4px; cursor: pointer;"
                                on:click=move |_| set_progress_value.update(|v| *v = (*v - 10.0).max(0.0))
                            >
                                "-10%"
                            </button>
                            <button
                                style="padding: 0.25rem 0.5rem; background: #6b7280; color: white; border: none; border-radius: 4px; cursor: pointer;"
                                on:click=move |_| set_progress_value.set(0.0)
                            >
                                "Reset"
                            </button>
                        </div>
                    </div>

                    <div>
                        <label style="color: #374151; display: block; margin-bottom: 0.5rem;">"Indeterminate Progress"</label>
                        <Progress>
                            <ProgressIndicator />
                        </Progress>
                    </div>
                </div>
            </section>

            // Theme Status
            <section style="margin: 2rem 0; padding: 1rem; border: 2px solid #e5e7eb; border-radius: 8px; background: #f9fafb;">
                <p style="color: #6b7280; text-align: center; margin: 0;">
                    "Current theme: "
                    <strong style="color: #374151;">{move || format!("{:?}", theme.get())}</strong>
                    " • Click " {theme_button_text} " to toggle"
                </p>
            </section>
        </div>
    }
}
