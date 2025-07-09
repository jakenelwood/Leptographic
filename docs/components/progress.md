# Progress Component

A progress indicator that displays the completion progress of a task, typically displayed as a progress bar.

## Features

- ✅ **Full WAI-ARIA compliance** - Screen reader accessible
- ✅ **Controllable state** - Reactive Signal-based props
- ✅ **Indeterminate state** - For unknown progress
- ✅ **Custom styling** - CSS custom properties
- ✅ **Form integration** - ID and class props
- ✅ **Error handling** - Input validation with logging
- ✅ **Responsive design** - Mobile-first approach

## Installation

```rust
use leptos_radix_ui::components::progress::{Progress, ProgressIndicator};
```

## Basic Usage

```rust
use leptos::prelude::*;
use leptos_radix_ui::components::progress::{Progress, ProgressIndicator};

#[component]
fn ProgressExample() -> impl IntoView {
    view! {
        <Progress value=75.0>
            <ProgressIndicator style="width: 75%; height: 100%; background-color: #10b981;">
                "75%"
            </ProgressIndicator>
        </Progress>
    }
}
```

## Reactive Progress

```rust
#[component]
fn ReactiveProgress() -> impl IntoView {
    let (progress, set_progress) = signal(25.0);
    let progress_signal = Signal::derive(move || progress.get());
    
    view! {
        <Progress value=progress_signal max=100.0>
            <ProgressIndicator>
                {move || format!("{}%", progress.get() as i32)}
            </ProgressIndicator>
        </Progress>
        
        <button on:click=move |_| set_progress.update(|p| *p = (*p + 10.0).min(100.0))>
            "Increase"
        </button>
    }
}
```

## Indeterminate Progress

```rust
#[component]
fn IndeterminateProgress() -> impl IntoView {
    view! {
        <Progress>
            <ProgressIndicator>
                "Loading..."
            </ProgressIndicator>
        </Progress>
    }
}
```

## API Reference

### Progress

The root progress component that provides context for all progress parts.

#### Props

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `value` | `Option<Signal<f64>>` | `None` | The progress value. `None` for indeterminate state |
| `max` | `Option<Signal<f64>>` | `100.0` | The maximum progress value |
| `get_value_label` | `Option<fn(f64, f64) -> String>` | percentage format | Function to get accessible label text |
| `id` | `Option<String>` | `None` | Optional ID for the progress element |
| `class` | `Option<String>` | `None` | Optional class name for styling |
| `children` | `ChildrenFn` | - | Child components |

#### Data Attributes

- `data-state`: `"indeterminate"` | `"loading"` | `"complete"`
- `data-value`: Current progress value
- `data-max`: Maximum progress value

#### ARIA Attributes

- `role="progressbar"`
- `aria-valuemin="0"`
- `aria-valuemax`: Dynamic maximum value
- `aria-valuenow`: Current value (undefined for indeterminate)
- `aria-valuetext`: Human-readable progress description

### ProgressIndicator

The visual indicator that shows the progress.

#### Props

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `class` | `Option<String>` | `None` | Optional class name for styling |
| `style` | `Option<Signal<String>>` | `None` | Optional reactive style attribute |
| `children` | `Option<ChildrenFn>` | `None` | Child components |

#### Data Attributes

- `data-state`: `"indeterminate"` | `"loading"` | `"complete"`
- `data-value`: Current progress value
- `data-max`: Maximum progress value

## Styling

### CSS Import

```css
@import "leptos-radix-ui/styles/progress.css";
```

### CSS Custom Properties

```css
:root {
  --progress-width: 300px;
  --progress-height: 20px;
  --progress-border-radius: 9999px;
  --progress-background: #e5e7eb;
  --progress-indicator-background-loading: #10b981;
  --progress-indicator-background-complete: #059669;
  --progress-transition-duration: 300ms;
}
```

### Size Variants

```rust
view! {
    <Progress class="progress-sm" value=50.0>
        <ProgressIndicator />
    </Progress>
    
    <Progress class="progress-lg" value=75.0>
        <ProgressIndicator />
    </Progress>
}
```

### Color Variants

```rust
view! {
    <Progress class="progress-green" value=60.0>
        <ProgressIndicator />
    </Progress>
    
    <Progress class="progress-purple" value=80.0>
        <ProgressIndicator />
    </Progress>
}
```

## Accessibility

The Progress component follows WAI-ARIA guidelines:

- Uses `role="progressbar"` for screen reader identification
- Provides `aria-valuenow`, `aria-valuemin`, `aria-valuemax` for current state
- Includes `aria-valuetext` for human-readable descriptions
- Supports indeterminate state for unknown progress
- Respects `prefers-reduced-motion` for animations

## Examples

See `examples/progress_example.rs` for a complete working example with:
- Static progress values
- Reactive progress with controls
- Indeterminate loading states
- Custom styling and animations

## Browser Support

- All modern browsers
- IE11+ (with polyfills)
- Mobile browsers
- Screen readers

## Related Components

- **Switch** - For binary on/off states
- **Slider** - For selecting values from a range
- **Checkbox** - For boolean selections
