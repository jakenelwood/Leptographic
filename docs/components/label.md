# Label Component

Renders an accessible label associated with controls.

## Features

- **Form Association**: Proper `for` attribute linking to form controls
- **Smart Interaction**: Prevents text selection on double-click while preserving form control interaction
- **State Management**: Required, disabled, and default states with visual feedback
- **Accessibility**: Full ARIA support with semantic labeling
- **Styling System**: CSS custom properties with comprehensive theming
- **Production Ready**: Enhanced props, data attributes, and error handling

## Installation

The Label component is included in the Leptos Radix UI library.

```rust
use leptos_radix_ui::components::label::*;
```

## Basic Usage

### Simple Form Label

```rust
use leptos::prelude::*;
use leptos_radix_ui::components::label::*;

#[component]
fn BasicForm() -> impl IntoView {
    view! {
        <div>
            <Label for_="email">"Email Address"</Label>
            <input id="email" type="email" placeholder="Enter your email" />
        </div>
    }
}
```

### Required Field Label

```rust
#[component]
fn RequiredField() -> impl IntoView {
    view! {
        <div>
            <Label for_="password" required=true>"Password"</Label>
            <input id="password" type="password" required />
        </div>
    }
}
```

## Advanced Usage

### Form with Multiple States

```rust
#[component]
fn AdvancedForm() -> impl IntoView {
    view! {
        <form>
            // Required field
            <div>
                <Label for_="username" required=true>"Username"</Label>
                <input id="username" type="text" required />
            </div>
            
            // Optional field
            <div>
                <Label for_="nickname" class="label-subtle">"Nickname (Optional)"</Label>
                <input id="nickname" type="text" />
            </div>
            
            // Disabled field
            <div>
                <Label for_="readonly" disabled=true>"Read-only Field"</Label>
                <input id="readonly" type="text" value="Cannot edit" disabled />
            </div>
        </form>
    }
}
```

### Styled Labels

```rust
#[component]
fn StyledLabels() -> impl IntoView {
    view! {
        <div>
            // Size variants
            <Label class="label-small label-blue">"Small Blue Label"</Label>
            <Label class="label-large label-green">"Large Green Label"</Label>
            
            // Style variants
            <Label class="label-bold label-red">"Bold Red Label"</Label>
            <Label class="label-uppercase label-purple">"Uppercase Purple Label"</Label>
            
            // Subtle styling
            <Label class="label-subtle">"Subtle Label"</Label>
        </div>
    }
}
```

### Inline Labels (Checkboxes/Radios)

```rust
#[component]
fn InlineLabels() -> impl IntoView {
    view! {
        <div>
            <div style="display: flex; align-items: center; gap: 0.5rem;">
                <input type="checkbox" id="terms" />
                <Label for_="terms" class="label-inline">"I agree to the terms"</Label>
            </div>
            
            <div style="display: flex; align-items: center; gap: 0.5rem;">
                <input type="radio" id="option1" name="choice" />
                <Label for_="option1" class="label-inline">"Option 1"</Label>
            </div>
        </div>
    }
}
```

## API Reference

### Props

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `for_` | `Option<String>` | `None` | ID of the form control this label is associated with |
| `class` | `Option<String>` | `None` | CSS class for styling |
| `id` | `Option<String>` | `None` | Element ID |
| `style` | `Option<String>` | `None` | Inline styles |
| `data_testid` | `Option<String>` | `None` | Test ID for automation |
| `aria_label` | `Option<String>` | `None` | Accessible label |
| `aria_labelledby` | `Option<String>` | `None` | ID of labeling element |
| `aria_describedby` | `Option<String>` | `None` | ID of describing element |
| `required` | `Option<bool>` | `false` | Whether this label is for a required field |
| `disabled` | `Option<bool>` | `false` | Whether this label is for a disabled field |
| `node_ref` | `Option<NodeRef<leptos::html::Label>>` | `None` | Reference to DOM element |
| `children` | `ChildrenFn` | - | Child components |

## Styling

### CSS Classes

The component uses these CSS classes for styling:

- `.label-root` - Base label styles
- `.label-required` - Required field styling with asterisk
- `.label-disabled` - Disabled field styling
- `.label-small` - Small size variant
- `.label-medium` - Medium size variant (default)
- `.label-large` - Large size variant
- `.label-subtle` - Subtle text styling
- `.label-bold` - Bold text styling
- `.label-uppercase` - Uppercase text styling
- `.label-inline` - Inline styling for checkboxes/radios
- `.label-blue` - Blue color variant
- `.label-green` - Green color variant
- `.label-red` - Red color variant
- `.label-purple` - Purple color variant

### Data Attributes

- `data-state` - "default", "required", or "disabled"
- `data-required` - Present when required=true
- `data-disabled` - Present when disabled=true
- `data-radix-label` - Component identifier

### CSS Custom Properties

```css
:root {
  --label-font-family: inherit;
  --label-font-size: 0.875rem;
  --label-font-weight: 500;
  --label-line-height: 1.25rem;
  --label-color: #374151;
  --label-color-required: #dc2626;
  --label-color-disabled: #9ca3af;
  --label-color-hover: #1f2937;
  --label-transition-duration: 150ms;
  --label-required-content: " *";
}
```

## Accessibility

### Form Association

- Uses native `for` attribute to associate with form controls
- Clicking label focuses the associated input
- Screen readers announce the label when the input is focused

### ARIA Support

- `aria-label` - Provides accessible name when text content isn't sufficient
- `aria-labelledby` - References other elements that label this control
- `aria-describedby` - References elements that provide additional description

### Smart Interaction

- Prevents text selection on double-click for better UX
- Preserves normal interaction with nested form controls
- Maintains cursor pointer for clickable indication

## Examples

### Complete Form

```rust
#[component]
fn ContactForm() -> impl IntoView {
    view! {
        <form>
            <fieldset>
                <legend>"Contact Information"</legend>
                
                <div class="form-group">
                    <Label for_="name" required=true>"Full Name"</Label>
                    <input id="name" type="text" required />
                </div>
                
                <div class="form-group">
                    <Label for_="email" required=true>"Email"</Label>
                    <input id="email" type="email" required />
                </div>
                
                <div class="form-group">
                    <Label for_="phone" class="label-subtle">"Phone (Optional)"</Label>
                    <input id="phone" type="tel" />
                </div>
                
                <div class="form-group">
                    <Label for_="message" required=true>"Message"</Label>
                    <textarea id="message" rows="4" required></textarea>
                </div>
            </fieldset>
        </form>
    }
}
```

### Settings Panel

```rust
#[component]
fn SettingsPanel() -> impl IntoView {
    view! {
        <div>
            <h3>"Notification Settings"</h3>
            
            <div style="display: flex; align-items: center; gap: 0.5rem; margin: 1rem 0;">
                <input type="checkbox" id="email-notifications" />
                <Label for_="email-notifications" class="label-inline">"Email notifications"</Label>
            </div>
            
            <div style="display: flex; align-items: center; gap: 0.5rem; margin: 1rem 0;">
                <input type="checkbox" id="push-notifications" />
                <Label for_="push-notifications" class="label-inline">"Push notifications"</Label>
            </div>
            
            <div style="display: flex; align-items: center; gap: 0.5rem; margin: 1rem 0;">
                <input type="checkbox" id="sms-notifications" disabled />
                <Label for_="sms-notifications" class="label-inline" disabled=true>"SMS notifications (Coming soon)"</Label>
            </div>
        </div>
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

- **Checkbox** - For boolean input controls with labels
- **Switch** - For toggle controls with labels
- **Form** - For complete form management (future component)
- **Input** - For text input controls (future component)
