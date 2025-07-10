use leptos::prelude::*;
use crate::components::checkbox::*;
use crate::components::switch::*;
use crate::components::progress::*;
use crate::components::separator::*;
use crate::components::label::*;
use crate::themes::{use_theme, ThemeMode};

#[component]
pub fn DemoPage() -> impl IntoView {
    let theme = use_theme();
    let (progress_value, set_progress_value) = signal(65.0);

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
        <div class="demo-container">
            <style>
                "
                .demo-container {
                    min-height: 100vh;
                    font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
                    line-height: 1.6;
                    transition: all 0.3s ease;
                }
                
                .dark .demo-container { background: #0a0a0a; color: #ffffff; }
                .light .demo-container { background: #ffffff; color: #1a1a1a; }
                
                .demo-header {
                    background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
                    color: white;
                    padding: 3rem 2rem;
                    text-align: center;
                    position: relative;
                }
                
                .demo-nav {
                    background: rgba(255, 255, 255, 0.95);
                    backdrop-filter: blur(10px);
                    padding: 1rem 2rem;
                    border-bottom: 1px solid #e5e7eb;
                    position: sticky;
                    top: 0;
                    z-index: 100;
                    display: flex;
                    justify-content: space-between;
                    align-items: center;
                }
                
                .dark .demo-nav {
                    background: rgba(26, 26, 26, 0.95);
                    border-bottom-color: #374151;
                }
                
                .demo-content {
                    max-width: 1200px;
                    margin: 0 auto;
                    padding: 2rem;
                    display: grid;
                    gap: 3rem;
                }
                
                .component-section {
                    background: #f8fafc;
                    border: 1px solid #e2e8f0;
                    border-radius: 12px;
                    padding: 2rem;
                    transition: all 0.3s ease;
                }
                
                .dark .component-section {
                    background: #1e293b;
                    border-color: #334155;
                }
                
                .component-section:hover {
                    transform: translateY(-2px);
                    box-shadow: 0 10px 25px rgba(0, 0, 0, 0.1);
                }
                
                .section-header {
                    display: flex;
                    align-items: center;
                    gap: 1rem;
                    margin-bottom: 2rem;
                    padding-bottom: 1rem;
                    border-bottom: 2px solid #e2e8f0;
                }

                .dark .section-header {
                    border-bottom-color: #334155;
                }
                
                .section-title {
                    font-size: 1.5rem;
                    font-weight: 600;
                    margin: 0;
                    color: #1e293b;
                }
                
                .dark .section-title {
                    color: #f1f5f9;
                }
                
                .demo-grid {
                    display: grid;
                    grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
                    gap: 1.5rem;
                }
                
                .demo-item {
                    background: white;
                    border: 1px solid #e2e8f0;
                    border-radius: 8px;
                    padding: 1.5rem;
                    transition: all 0.2s ease;
                }
                
                .dark .demo-item {
                    background: #334155;
                    border-color: #475569;
                }
                
                .demo-item:hover {
                    border-color: #667eea;
                    box-shadow: 0 4px 12px rgba(102, 126, 234, 0.15);
                }
                
                .theme-toggle {
                    padding: 0.75rem 1rem;
                    background: rgba(255, 255, 255, 0.2);
                    border: 1px solid rgba(255, 255, 255, 0.3);
                    border-radius: 8px;
                    color: white;
                    cursor: pointer;
                    font-size: 1.1rem;
                    transition: all 0.2s ease;
                    backdrop-filter: blur(10px);
                }
                
                .theme-toggle:hover {
                    background: rgba(255, 255, 255, 0.3);
                    transform: scale(1.05);
                }
                
                .stats-grid {
                    display: grid;
                    grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
                    gap: 1rem;
                    margin: 2rem 0;
                }

                .stat-card {
                    background: rgba(255, 255, 255, 0.1);
                    backdrop-filter: blur(10px);
                    border-radius: 8px;
                    padding: 1.5rem;
                    text-align: center;
                    border: 1px solid rgba(255, 255, 255, 0.2);
                }

                .stat-number {
                    font-size: 2rem;
                    font-weight: bold;
                    display: block;
                }

                .stat-label {
                    font-size: 0.875rem;
                    opacity: 0.9;
                    margin-top: 0.5rem;
                }

                /* Progress bar dark mode fixes */
                .dark [role='progressbar'] {
                    background-color: #374151 !important;
                    border: 1px solid #4b5563;
                }

                .dark [role='progressbar'] > div {
                    background-color: #3b82f6 !important;
                }

                .dark .progress-blue [role='progressbar'] > div {
                    background-color: #60a5fa !important;
                }

                .dark .progress-green [role='progressbar'] > div {
                    background-color: #34d399 !important;
                }

                .dark .progress-red [role='progressbar'] > div {
                    background-color: #f87171 !important;
                }

                .dark .progress-purple [role='progressbar'] > div {
                    background-color: #a78bfa !important;
                }
                "
            </style>

            // Header Section
            <header class="demo-header">
                <h1 style="font-size: 3rem; margin: 0 0 2rem 0; font-weight: 700;">
                    "Leptos Radix UI"
                </h1>
                
                <div class="stats-grid">
                    <div class="stat-card">
                        <span class="stat-number">"5"</span>
                        <div class="stat-label">"Components Ready"</div>
                    </div>
                    <div class="stat-card">
                        <span class="stat-number">"100%"</span>
                        <div class="stat-label">"Radix Compatible"</div>
                    </div>
                    <div class="stat-card">
                        <span class="stat-number">"0"</span>
                        <div class="stat-label">"Dependencies"</div>
                    </div>
                    <div class="stat-card">
                        <span class="stat-number">"A11Y"</span>
                        <div class="stat-label">"Accessible"</div>
                    </div>
                </div>
            </header>

            // Navigation
            <nav class="demo-nav">
                <div style="display: flex; align-items: center; gap: 2rem;">
                    <span style="font-weight: 600; color: #667eea;">"Component Demo"</span>
                </div>

                <button
                    class="theme-toggle"
                    title=move || match theme.mode.get() {
                        ThemeMode::Dark => "Switch to light mode",
                        ThemeMode::Light => "Switch to dark mode",
                    }
                    on:click=move |_| theme.toggle()
                >
                    {theme_button_text}
                </button>
            </nav>

            // Main Content
            <main class="demo-content">
                // Checkbox Section
                <section id="checkbox" class="component-section">
                    <div class="section-header">
                        <h2 class="section-title">"Checkbox"</h2>
                        <a href="https://github.com/jakenelwood/themachine/blob/main/src/components/checkbox.rs"
                           target="_blank"
                           style="color: #667eea; text-decoration: none; font-size: 0.875rem; margin-left: auto;">
                            "View Code →"
                        </a>
                    </div>
                    
                    <div class="demo-grid">
                        <div class="demo-item">
                            <h3 style="margin: 0 0 1rem 0; color: #475569;">"Basic States"</h3>
                            <div style="display: flex; flex-direction: column; gap: 1rem;">
                                <div style="display: flex; align-items: center; gap: 0.75rem;">
                                    <Checkbox>
                                        <CheckboxIndicator />
                                    </Checkbox>
                                    <Label>"Unchecked"</Label>
                                </div>
                                <div style="display: flex; align-items: center; gap: 0.75rem;">
                                    <Checkbox default_checked=true>
                                        <CheckboxIndicator />
                                    </Checkbox>
                                    <Label>"Checked"</Label>
                                </div>
                                <div style="display: flex; align-items: center; gap: 0.75rem;">
                                    <Checkbox disabled=true>
                                        <CheckboxIndicator />
                                    </Checkbox>
                                    <Label>"Disabled"</Label>
                                </div>
                            </div>
                        </div>
                        
                        <div class="demo-item">
                            <h3 style="margin: 0 0 1rem 0; color: #475569;">"Form Integration"</h3>
                            <form style="display: flex; flex-direction: column; gap: 1rem;">
                                <div style="display: flex; align-items: center; gap: 0.75rem;">
                                    <Checkbox name="newsletter".to_string()>
                                        <CheckboxIndicator />
                                    </Checkbox>
                                    <Label for_="newsletter">"Subscribe to newsletter"</Label>
                                </div>
                                <div style="display: flex; align-items: center; gap: 0.75rem;">
                                    <Checkbox name="terms".to_string() required=true>
                                        <CheckboxIndicator />
                                    </Checkbox>
                                    <Label for_="terms">"Accept terms and conditions *"</Label>
                                </div>
                            </form>
                        </div>
                    </div>
                </section>

                // Switch Section
                <section id="switch" class="component-section">
                    <div class="section-header">
                        <h2 class="section-title">"Switch"</h2>
                        <a href="https://github.com/jakenelwood/themachine/blob/main/src/components/switch.rs"
                           target="_blank"
                           style="color: #667eea; text-decoration: none; font-size: 0.875rem; margin-left: auto;">
                            "View Code →"
                        </a>
                    </div>

                    <div class="demo-grid">
                        <div class="demo-item">
                            <h3 style="margin: 0 0 1rem 0; color: #475569;">"Toggle States"</h3>
                            <div style="display: flex; flex-direction: column; gap: 1rem;">
                                <div style="display: flex; align-items: center; gap: 0.75rem;">
                                    <Switch>
                                        <SwitchThumb />
                                    </Switch>
                                    <Label>"Off"</Label>
                                </div>
                                <div style="display: flex; align-items: center; gap: 0.75rem;">
                                    <Switch default_checked=true>
                                        <SwitchThumb />
                                    </Switch>
                                    <Label>"On"</Label>
                                </div>
                                <div style="display: flex; align-items: center; gap: 0.75rem;">
                                    <Switch disabled=true>
                                        <SwitchThumb />
                                    </Switch>
                                    <Label>"Disabled"</Label>
                                </div>
                            </div>
                        </div>

                        <div class="demo-item">
                            <h3 style="margin: 0 0 1rem 0; color: #475569;">"Settings Panel"</h3>
                            <div style="display: flex; flex-direction: column; gap: 1rem;">
                                <div style="display: flex; justify-content: space-between; align-items: center;">
                                    <Label>"Dark Mode"</Label>
                                    <Switch>
                                        <SwitchThumb />
                                    </Switch>
                                </div>
                                <div style="display: flex; justify-content: space-between; align-items: center;">
                                    <Label>"Notifications"</Label>
                                    <Switch default_checked=true>
                                        <SwitchThumb />
                                    </Switch>
                                </div>
                                <div style="display: flex; justify-content: space-between; align-items: center;">
                                    <Label>"Auto-save"</Label>
                                    <Switch>
                                        <SwitchThumb />
                                    </Switch>
                                </div>
                            </div>
                        </div>
                    </div>
                </section>

                // Progress Section
                <section id="progress" class="component-section">
                    <div class="section-header">
                        <h2 class="section-title">"Progress"</h2>
                        <a href="https://github.com/jakenelwood/themachine/blob/main/src/components/progress.rs"
                           target="_blank"
                           style="color: #667eea; text-decoration: none; font-size: 0.875rem; margin-left: auto;">
                            "View Code →"
                        </a>
                    </div>

                    <div class="demo-grid">
                        <div class="demo-item">
                            <h3 style="margin: 0 0 1rem 0; color: #475569;">"Static Progress"</h3>
                            <div style="display: flex; flex-direction: column; gap: 1.5rem;">
                                <div>
                                    <Label style="display: block; margin-bottom: 0.5rem;">"25% Complete"</Label>
                                    <Progress value=Signal::from(25.0)>
                                        <ProgressIndicator />
                                    </Progress>
                                </div>
                                <div>
                                    <Label style="display: block; margin-bottom: 0.5rem;">"75% Complete"</Label>
                                    <Progress value=Signal::from(75.0)>
                                        <ProgressIndicator />
                                    </Progress>
                                </div>
                                <div>
                                    <Label style="display: block; margin-bottom: 0.5rem;">"100% Complete"</Label>
                                    <Progress value=Signal::from(100.0)>
                                        <ProgressIndicator />
                                    </Progress>
                                </div>
                            </div>
                        </div>

                        <div class="demo-item">
                            <h3 style="margin: 0 0 1rem 0; color: #475569;">"Interactive Progress"</h3>
                            <div style="display: flex; flex-direction: column; gap: 1rem;">
                                <div>
                                    <Label style="display: block; margin-bottom: 0.5rem;">
                                        "Progress: " {move || format!("{:.0}%", progress_value.get())}
                                    </Label>
                                    <Progress value=progress_value>
                                        <ProgressIndicator />
                                    </Progress>
                                </div>
                                <div style="display: flex; gap: 0.5rem; flex-wrap: wrap;">
                                    <button
                                        style="padding: 0.5rem 1rem; background: #3b82f6; color: white; border: none; border-radius: 6px; cursor: pointer; font-size: 0.875rem;"
                                        on:click=move |_| set_progress_value.update(|v| *v = (*v + 10.0).min(100.0))
                                    >
                                        "+10%"
                                    </button>
                                    <button
                                        style="padding: 0.5rem 1rem; background: #ef4444; color: white; border: none; border-radius: 6px; cursor: pointer; font-size: 0.875rem;"
                                        on:click=move |_| set_progress_value.update(|v| *v = (*v - 10.0).max(0.0))
                                    >
                                        "-10%"
                                    </button>
                                    <button
                                        style="padding: 0.5rem 1rem; background: #6b7280; color: white; border: none; border-radius: 6px; cursor: pointer; font-size: 0.875rem;"
                                        on:click=move |_| set_progress_value.set(50.0)
                                    >
                                        "Reset"
                                    </button>
                                </div>
                            </div>
                        </div>

                        <div class="demo-item">
                            <h3 style="margin: 0 0 1rem 0; color: #475569;">"Indeterminate"</h3>
                            <div>
                                <Label style="display: block; margin-bottom: 0.5rem;">"Loading..."</Label>
                                <Progress>
                                    <ProgressIndicator />
                                </Progress>
                            </div>
                        </div>
                    </div>
                </section>

                // Separator Section
                <section id="separator" class="component-section">
                    <div class="section-header">
                        <h2 class="section-title">"Separator"</h2>
                        <a href="https://github.com/jakenelwood/themachine/blob/main/src/components/separator.rs"
                           target="_blank"
                           style="color: #667eea; text-decoration: none; font-size: 0.875rem; margin-left: auto;">
                            "View Code →"
                        </a>
                    </div>

                    <div class="demo-grid">
                        <div class="demo-item">
                            <h3 style="margin: 0 0 1rem 0; color: #475569;">"Horizontal"</h3>
                            <div style="display: flex; flex-direction: column; gap: 1rem;">
                                <p style="margin: 0; color: #64748b;">"Content above"</p>
                                <Separator />
                                <p style="margin: 0; color: #64748b;">"Content below"</p>
                            </div>
                        </div>

                        <div class="demo-item">
                            <h3 style="margin: 0 0 1rem 0; color: #475569;">"Vertical"</h3>
                            <div style="display: flex; align-items: center; gap: 1rem; height: 3rem;">
                                <span style="color: #64748b;">"Left"</span>
                                <Separator orientation=Orientation::Vertical />
                                <span style="color: #64748b;">"Right"</span>
                            </div>
                        </div>

                        <div class="demo-item">
                            <h3 style="margin: 0 0 1rem 0; color: #475569;">"Styled"</h3>
                            <div style="display: flex; flex-direction: column; gap: 1rem;">
                                <div>
                                    <p style="margin: 0 0 0.5rem 0; color: #64748b; font-size: 0.875rem;">"Blue Separator"</p>
                                    <Separator class="separator-blue separator-large" />
                                </div>
                                <div>
                                    <p style="margin: 0 0 0.5rem 0; color: #64748b; font-size: 0.875rem;">"Dashed Separator"</p>
                                    <Separator class="separator-green separator-dashed" />
                                </div>
                            </div>
                        </div>
                    </div>
                </section>

                // Label Section
                <section id="label" class="component-section">
                    <div class="section-header">
                        <h2 class="section-title">"Label"</h2>
                        <a href="https://github.com/jakenelwood/themachine/blob/main/src/components/label.rs"
                           target="_blank"
                           style="color: #667eea; text-decoration: none; font-size: 0.875rem; margin-left: auto;">
                            "View Code →"
                        </a>
                    </div>

                    <div class="demo-grid">
                        <div class="demo-item">
                            <h3 style="margin: 0 0 1rem 0; color: #475569;">"Form Labels"</h3>
                            <div style="display: flex; flex-direction: column; gap: 1rem;">
                                <div style="display: flex; flex-direction: column; gap: 0.5rem;">
                                    <Label for_="demo-email">"Email Address"</Label>
                                    <input
                                        id="demo-email"
                                        type="email"
                                        placeholder="Enter your email"
                                        style="padding: 0.75rem; border: 1px solid #d1d5db; border-radius: 6px; font-size: 0.875rem;"
                                    />
                                </div>
                                <div style="display: flex; flex-direction: column; gap: 0.5rem;">
                                    <Label for_="demo-password" required=true>"Password"</Label>
                                    <input
                                        id="demo-password"
                                        type="password"
                                        placeholder="Enter your password"
                                        style="padding: 0.75rem; border: 1px solid #d1d5db; border-radius: 6px; font-size: 0.875rem;"
                                    />
                                </div>
                            </div>
                        </div>

                        <div class="demo-item">
                            <h3 style="margin: 0 0 1rem 0; color: #475569;">"Label Variants"</h3>
                            <div style="display: flex; flex-direction: column; gap: 1rem;">
                                <div>
                                    <Label class="label-small label-blue">"Small Blue Label"</Label>
                                </div>
                                <div>
                                    <Label class="label-large label-green">"Large Green Label"</Label>
                                </div>
                                <div>
                                    <Label class="label-uppercase label-purple">"Uppercase Purple"</Label>
                                </div>
                                <div>
                                    <Label disabled=true>"Disabled Label"</Label>
                                </div>
                            </div>
                        </div>

                        <div class="demo-item">
                            <h3 style="margin: 0 0 1rem 0; color: #475569;">"With Components"</h3>
                            <div style="display: flex; flex-direction: column; gap: 1rem;">
                                <div style="display: flex; align-items: center; gap: 0.75rem;">
                                    <Checkbox>
                                        <CheckboxIndicator />
                                    </Checkbox>
                                    <Label>"Accept terms"</Label>
                                </div>
                                <div style="display: flex; align-items: center; gap: 0.75rem;">
                                    <Switch>
                                        <SwitchThumb />
                                    </Switch>
                                    <Label>"Enable notifications"</Label>
                                </div>
                            </div>
                        </div>
                    </div>
                </section>


            </main>
        </div>
    }
}
