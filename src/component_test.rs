use leptos::prelude::*;
use crate::components::checkbox::*;
use crate::components::switch::*;
use crate::components::progress::*;
use crate::components::separator::*;
use crate::components::label::*;
use crate::themes::{use_theme, ThemeMode};

#[component]
pub fn ComponentTestPage() -> impl IntoView {
    // Progress value for interactive testing
    let (progress_value, set_progress_value) = signal(50.0);

    // Use simplified theme system
    let theme = use_theme();

    let theme_button_text = move || {
        match theme.mode.get() {
            ThemeMode::Dark => "☀️",
            ThemeMode::Light => "🌙",
        }
    };

    // Effect to update HTML class when theme changes
    Effect::new(move |_| {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let html = document.document_element().unwrap();
        let class_list = html.class_list();

        match theme.mode.get() {
            ThemeMode::Dark => {
                let _ = class_list.add_1("dark");
                let _ = class_list.remove_1("light");
            }
            ThemeMode::Light => {
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
                    title=move || match theme.mode.get() {
                        ThemeMode::Dark => "Switch to light mode",
                        ThemeMode::Light => "Switch to dark mode",
                    }
                    on:click=move |_| theme.toggle()
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
                            <CheckboxIndicator />
                        </Checkbox>
                        <label style="color: #374151;">"Basic Checkbox (Unchecked)"</label>
                    </div>

                    <div style="display: flex; align-items: center; gap: 0.5rem;">
                        <Checkbox default_checked=CheckedState::True>
                            <CheckboxIndicator />
                        </Checkbox>
                        <label style="color: #374151;">"Checked by Default"</label>
                    </div>

                    <div style="display: flex; align-items: center; gap: 0.5rem;">
                        <Checkbox default_checked=CheckedState::Indeterminate>
                            <CheckboxIndicator />
                        </Checkbox>
                        <label style="color: #374151;">"Indeterminate State"</label>
                    </div>

                    <div style="display: flex; align-items: center; gap: 0.5rem;">
                        <Checkbox disabled=true>
                            <CheckboxIndicator />
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

            // Separator Tests
            <section style="margin: 2rem 0; padding: 1.5rem; border: 2px solid #e5e7eb; border-radius: 8px; background: #f9fafb;">
                <h2 style="color: #374151; margin-bottom: 1rem;">"➖ Separator Component"</h2>

                <div style="display: flex; flex-direction: column; gap: 1.5rem;">
                    <div>
                        <label style="color: #374151; display: block; margin-bottom: 0.5rem;">"Horizontal Separator (Default)"</label>
                        <div style="padding: 1rem; border: 1px solid #e5e7eb; border-radius: 4px;">
                            <p style="margin: 0; color: #6b7280;">"Content above"</p>
                            <Separator style="margin: 1rem 0;" />
                            <p style="margin: 0; color: #6b7280;">"Content below"</p>
                        </div>
                    </div>

                    <div>
                        <label style="color: #374151; display: block; margin-bottom: 0.5rem;">"Vertical Separator"</label>
                        <div style="padding: 1rem; border: 1px solid #e5e7eb; border-radius: 4px; display: flex; align-items: center; gap: 1rem; height: 3rem;">
                            <p style="margin: 0; color: #6b7280;">"Left content"</p>
                            <Separator orientation=Orientation::Vertical />
                            <p style="margin: 0; color: #6b7280;">"Right content"</p>
                        </div>
                    </div>

                    <div>
                        <label style="color: #374151; display: block; margin-bottom: 0.5rem;">"Styled Separators"</label>
                        <div style="padding: 1rem; border: 1px solid #e5e7eb; border-radius: 4px; display: flex; flex-direction: column; gap: 1rem;">
                            <div>
                                <p style="margin: 0 0 0.5rem 0; color: #6b7280; font-size: 0.875rem;">"Blue Large Separator"</p>
                                <Separator class="separator-blue separator-large" style="margin: 0.5rem 0;" />
                            </div>
                            <div>
                                <p style="margin: 0 0 0.5rem 0; color: #6b7280; font-size: 0.875rem;">"Green Dashed Separator"</p>
                                <Separator class="separator-green separator-dashed" style="margin: 0.5rem 0;" />
                            </div>
                            <div>
                                <p style="margin: 0 0 0.5rem 0; color: #6b7280; font-size: 0.875rem;">"Purple Gradient Separator"</p>
                                <Separator class="separator-purple separator-gradient" style="margin: 0.5rem 0;" />
                            </div>
                        </div>
                    </div>

                    <div>
                        <label style="color: #374151; display: block; margin-bottom: 0.5rem;">"Decorative Separator (No ARIA)"</label>
                        <div style="padding: 1rem; border: 1px solid #e5e7eb; border-radius: 4px;">
                            <p style="margin: 0; color: #6b7280;">"Visual decoration only"</p>
                            <Separator
                                decorative=true
                                style="margin: 1rem 0; height: 2px; background: linear-gradient(to right, #3b82f6, #8b5cf6); border-radius: 1px;"
                                data_testid="decorative-separator"
                            />
                            <p style="margin: 0; color: #6b7280;">"Not announced to screen readers"</p>
                        </div>
                    </div>

                    <div>
                        <label style="color: #374151; display: block; margin-bottom: 0.5rem;">"Production Features Demo"</label>
                        <div style="padding: 1rem; border: 1px solid #e5e7eb; border-radius: 4px;">
                            <p style="margin: 0; color: #6b7280;">"Separator with ID and custom class"</p>
                            <Separator
                                id="production-separator"
                                class="custom-separator-class"
                                style="margin: 1rem 0; height: 1px; background-color: #10b981;"
                                aria_label="Content divider"
                                data_testid="production-separator"
                            />
                            <p style="margin: 0; color: #6b7280;">"Enhanced with production-ready props"</p>
                        </div>
                    </div>
                </div>
            </section>

            // Label Tests
            <section style="margin: 2rem 0; padding: 1.5rem; border: 2px solid #e5e7eb; border-radius: 8px; background: #f9fafb;">
                <h2 style="color: #374151; margin-bottom: 1rem;">"🏷️ Label Component"</h2>

                <div style="display: flex; flex-direction: column; gap: 1.5rem;">
                    <div>
                        <label style="color: #374151; display: block; margin-bottom: 0.5rem;">"Basic Form Association"</label>
                        <div style="padding: 1rem; border: 1px solid #e5e7eb; border-radius: 4px; display: flex; flex-direction: column; gap: 1rem;">
                            <div style="display: flex; flex-direction: column; gap: 0.5rem;">
                                <Label for_="email-input">"Email Address"</Label>
                                <input
                                    id="email-input"
                                    type="email"
                                    placeholder="Enter your email"
                                    style="padding: 0.5rem; border: 1px solid #d1d5db; border-radius: 4px;"
                                />
                            </div>

                            <div style="display: flex; flex-direction: column; gap: 0.5rem;">
                                <Label for_="password-input">"Password"</Label>
                                <input
                                    id="password-input"
                                    type="password"
                                    placeholder="Enter your password"
                                    style="padding: 0.5rem; border: 1px solid #d1d5db; border-radius: 4px;"
                                />
                            </div>
                        </div>
                    </div>

                    <div>
                        <label style="color: #374151; display: block; margin-bottom: 0.5rem;">"Label with Checkbox"</label>
                        <div style="padding: 1rem; border: 1px solid #e5e7eb; border-radius: 4px;">
                            <div style="display: flex; align-items: center; gap: 0.5rem;">
                                <Checkbox>
                                    <CheckboxIndicator />
                                </Checkbox>
                                <Label>"I agree to the terms and conditions"</Label>
                            </div>
                        </div>
                    </div>

                    <div>
                        <label style="color: #374151; display: block; margin-bottom: 0.5rem;">"Styled Labels"</label>
                        <div style="padding: 1rem; border: 1px solid #e5e7eb; border-radius: 4px; display: flex; flex-direction: column; gap: 1rem;">
                            <div style="display: flex; flex-direction: column; gap: 0.5rem;">
                                <Label
                                    for_="required-input"
                                    required=true
                                >"Required Field"</Label>
                                <input
                                    id="required-input"
                                    type="text"
                                    placeholder="This field is required"
                                    style="padding: 0.5rem; border: 2px solid #3b82f6; border-radius: 4px;"
                                />
                            </div>

                            <div style="display: flex; flex-direction: column; gap: 0.5rem;">
                                <Label
                                    for_="optional-input"
                                    class="label-subtle"
                                >"Optional Field"</Label>
                                <input
                                    id="optional-input"
                                    type="text"
                                    placeholder="This field is optional"
                                    style="padding: 0.5rem; border: 1px solid #d1d5db; border-radius: 4px;"
                                />
                            </div>

                            <div style="display: flex; flex-direction: column; gap: 0.5rem;">
                                <Label
                                    for_="disabled-input"
                                    disabled=true
                                >"Disabled Field"</Label>
                                <input
                                    id="disabled-input"
                                    type="text"
                                    placeholder="This field is disabled"
                                    disabled=true
                                    style="padding: 0.5rem; border: 1px solid #d1d5db; border-radius: 4px; background: #f9fafb; color: #9ca3af;"
                                />
                            </div>
                        </div>
                    </div>

                    <div>
                        <label style="color: #374151; display: block; margin-bottom: 0.5rem;">"Label Variants"</label>
                        <div style="padding: 1rem; border: 1px solid #e5e7eb; border-radius: 4px; display: flex; flex-direction: column; gap: 1rem;">
                            <div style="display: flex; flex-direction: column; gap: 0.5rem;">
                                <Label class="label-small label-blue">"Small Blue Label"</Label>
                                <input type="text" placeholder="Small input" style="padding: 0.375rem; border: 1px solid #d1d5db; border-radius: 4px; font-size: 0.875rem;" />
                            </div>

                            <div style="display: flex; flex-direction: column; gap: 0.5rem;">
                                <Label class="label-large label-green">"Large Green Label"</Label>
                                <input type="text" placeholder="Large input" style="padding: 0.75rem; border: 1px solid #d1d5db; border-radius: 4px; font-size: 1rem;" />
                            </div>

                            <div style="display: flex; flex-direction: column; gap: 0.5rem;">
                                <Label class="label-uppercase label-purple">"Uppercase Purple Label"</Label>
                                <input type="text" placeholder="Styled input" style="padding: 0.5rem; border: 1px solid #d1d5db; border-radius: 4px;" />
                            </div>

                            <div style="display: flex; flex-direction: column; gap: 0.5rem;">
                                <Label class="label-bold label-red">"Bold Red Label"</Label>
                                <input type="text" placeholder="Bold styled input" style="padding: 0.5rem; border: 1px solid #d1d5db; border-radius: 4px;" />
                            </div>
                        </div>
                    </div>
                </div>
            </section>

            // Theme Status
            <section style="margin: 2rem 0; padding: 1rem; border: 2px solid #e5e7eb; border-radius: 8px; background: #f9fafb;">
                <p style="color: #6b7280; text-align: center; margin: 0;">
                    "Theme toggle working • Click " {theme_button_text} " to toggle"
                </p>
            </section>
        </div>
    }
}
