# Separator Component

Visually or semantically separates content.

## Features

- **Orientation Support**: Horizontal (default) and vertical orientations
- **Accessibility**: Full WAI-ARIA compliance with semantic/decorative modes
- **Styling System**: CSS custom properties with comprehensive theming
- **Production Ready**: Form integration, data attributes, and error handling
- **Responsive**: Mobile-first design with breakpoint adjustments

## Installation

The Separator component is included in the Leptos Radix UI library.

```rust
use leptos_radix_ui::components::separator::*;
```

## Basic Usage

### Horizontal Separator (Default)

```rust
use leptos::prelude::*;
use leptos_radix_ui::components::separator::*;

#[component]
fn HorizontalExample() -> impl IntoView {
    view! {
        <div>
            <p>"Content above"</p>
            <Separator />
            <p>"Content below"</p>
        </div>
    }
}
```

### Vertical Separator

```rust
#[component]
fn VerticalExample() -> impl IntoView {
    view! {
        <div style="display: flex; align-items: center; gap: 1rem;">
            <p>"Left content"</p>
            <Separator orientation=Orientation::Vertical />
            <p>"Right content"</p>
        </div>
    }
}
```

## Advanced Usage

### Decorative Separator

When a separator is purely decorative, use the `decorative` prop to remove it from the accessibility tree:

```rust
#[component]
fn DecorativeExample() -> impl IntoView {
    view! {
        <div>
            <p>"Visual decoration only"</p>
            <Separator 
                decorative=true 
                class="separator-gradient"
            />
            <p>"Not announced to screen readers"</p>
        </div>
    }
}
```

### Styled Separators

```rust
#[component]
fn StyledExample() -> impl IntoView {
    view! {
        <div>
            // Blue large separator
            <Separator 
                class="separator-blue separator-large"
                aria_label="Section divider"
            />
            
            // Dashed green separator
            <Separator 
                class="separator-green separator-dashed"
                style="margin: 2rem 0;"
            />
            
            // Purple gradient separator
            <Separator 
                class="separator-purple separator-gradient"
                id="main-divider"
            />
        </div>
    }
}
```

## API Reference

### Props

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `orientation` | `Option<Orientation>` | `Horizontal` | Either `Horizontal` or `Vertical` |
| `decorative` | `Option<bool>` | `false` | Whether the separator is purely decorative |
| `class` | `Option<String>` | `None` | CSS class for styling |
| `id` | `Option<String>` | `None` | Element ID for form integration |
| `style` | `Option<String>` | `None` | Inline styles |
| `data_testid` | `Option<String>` | `None` | Test ID for automation |
| `aria_label` | `Option<String>` | `None` | Accessible label |
| `aria_labelledby` | `Option<String>` | `None` | ID of labeling element |
| `aria_describedby` | `Option<String>` | `None` | ID of describing element |
| `node_ref` | `Option<NodeRef<leptos::html::Div>>` | `None` | Reference to DOM element |
| `children` | `Option<ChildrenFn>` | `None` | Child components (optional) |

### Orientation Enum

```rust
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum Orientation {
    #[default]
    Horizontal,
    Vertical,
}
```

## Styling

### CSS Classes

The component uses these CSS classes for styling:

- `.separator-root` - Base separator styles
- `.separator-small` - Small size variant
- `.separator-medium` - Medium size variant (default)
- `.separator-large` - Large size variant
- `.separator-blue` - Blue color variant
- `.separator-green` - Green color variant
- `.separator-red` - Red color variant
- `.separator-purple` - Purple color variant
- `.separator-solid` - Solid line style (default)
- `.separator-dashed` - Dashed line style
- `.separator-dotted` - Dotted line style
- `.separator-gradient` - Gradient fade style

### Data Attributes

- `data-orientation` - "horizontal" or "vertical"
- `data-state` - "semantic" or "decorative"
- `data-radix-separator` - Component identifier

### CSS Custom Properties

```css
:root {
  --separator-horizontal-height: 1px;
  --separator-vertical-width: 1px;
  --separator-color: #e5e7eb;
  --separator-color-decorative: #d1d5db;
  --separator-margin: 0;
  --separator-transition-duration: 150ms;
  --separator-transition-timing: ease;
}
```

## Accessibility

### Semantic Separator (Default)

- Uses `role="separator"`
- Includes `aria-orientation` for vertical separators
- Announced to screen readers
- Provides semantic meaning

### Decorative Separator

- Uses `role="none"`
- Hidden from accessibility tree
- Purely visual decoration
- Not announced to screen readers

### ARIA Support

The component supports comprehensive ARIA attributes:

- `aria-label` - Accessible name
- `aria-labelledby` - Reference to labeling element
- `aria-describedby` - Reference to describing element
- `aria-orientation` - Orientation for screen readers

## Examples

### Navigation Menu Separator

```rust
#[component]
fn NavigationMenu() -> impl IntoView {
    view! {
        <nav style="display: flex; align-items: center; gap: 1rem;">
            <a href="/home">"Home"</a>
            <Separator orientation=Orientation::Vertical />
            <a href="/about">"About"</a>
            <Separator orientation=Orientation::Vertical />
            <a href="/contact">"Contact"</a>
        </nav>
    }
}
```

### Content Section Divider

```rust
#[component]
fn ContentSections() -> impl IntoView {
    view! {
        <article>
            <section>
                <h2>"Introduction"</h2>
                <p>"Content introduction..."</p>
            </section>
            
            <Separator 
                aria_label="End of introduction section"
                style="margin: 2rem 0;"
            />
            
            <section>
                <h2>"Main Content"</h2>
                <p>"Main content..."</p>
            </section>
        </article>
    }
}
```

### Form Field Groups

```rust
#[component]
fn FormFieldGroups() -> impl IntoView {
    view! {
        <form>
            <fieldset>
                <legend>"Personal Information"</legend>
                // Form fields...
            </fieldset>
            
            <Separator 
                decorative=true
                class="separator-dashed"
                style="margin: 1.5rem 0;"
            />
            
            <fieldset>
                <legend>"Contact Information"</legend>
                // Form fields...
            </fieldset>
        </form>
    }
}
```

## Browser Support

- All modern browsers
- Responsive design with mobile-first approach
- Dark mode support via `prefers-color-scheme`
- High contrast mode support via `prefers-contrast`
- Reduced motion support via `prefers-reduced-motion`

## Related Components

- **Progress** - For progress indicators with visual separation
- **Tabs** - For tabbed interfaces with content separation
- **Dialog** - For modal content with visual sections
