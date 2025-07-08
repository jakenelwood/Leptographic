#### WAI-ARIA Pattern: Dialog (Modal)
---

**1. Keyboard Interactions**
*   **For the Dialog:**
    *   `Tab`: Moves focus to the next tabbable element inside the dialog. If focus is on the last tabbable element, moves focus to the first tabbable element inside the dialog.
    *   `Shift + Tab`: Moves focus to the previous tabbable element inside the dialog. If focus is on the first tabbable element, moves focus to the last tabbable element inside the dialog.
    *   `Escape`: Closes the dialog.

**2. WAI-ARIA Roles, States, & Properties**
*   **Dialog Container Element:**
    *   `role`: `dialog`
    *   `aria-modal`: `true` (for modal dialogs)
    *   `aria-labelledby`: Must point to the ID of the dialog's title element, OR
    *   `aria-label`: Must provide a label if no visible title exists
    *   `aria-describedby`: (Optional) Must point to the ID of the dialog's description element. Omit if dialog contains complex semantic structures.

**3. Focus Management**
*   When the dialog opens, focus must be set on an appropriate element inside the dialog:
    *   Generally the first focusable element
    *   For content with semantic structures (lists, tables, paragraphs): add `tabindex="-1"` to a static element at the start and focus it
    *   For large content: focus the dialog title or first paragraph with `tabindex="-1"`
    *   For destructive actions: focus the least destructive action (e.g., Cancel button)
    *   For simple workflows: focus the most frequently used element (e.g., OK button)
*   Focus must be trapped within the dialog while it is open (Tab and Shift+Tab cycle within dialog only)
*   When the dialog closes, focus must return to the element that triggered it, unless:
    *   The triggering element no longer exists (focus another logical element)
    *   The workflow design makes focusing a different element more logical

**4. DOM Structure & Behavior Notes**
*   The dialog content should be rendered in a top-level portal to avoid z-index and stacking context issues
*   All elements required to operate the dialog must be descendants of the dialog container
*   The dialog should close when the user presses Escape
*   For modal dialogs: application code must prevent all users from interacting with content outside the dialog
*   For modal dialogs: visual styling must obscure content outside the dialog
*   It is strongly recommended to include a visible close button with `role="button"`
*   Windows under a modal dialog are inert - users cannot interact with content outside the active dialog
*   The dialog container should not be a descendant of any element with `aria-hidden="true"`

**5. Implementation Requirements**
*   Support both controlled and uncontrolled modes
*   Support custom trigger elements
*   Support portal rendering for proper layering
*   Implement focus trapping and restoration
*   Handle Escape key globally when dialog is open
*   Support overlay click-to-close (optional)
*   Provide proper TypeScript types for all props
*   Support composition via `as_child` pattern
*   Include data attributes for styling hooks (`data-state`, etc.)

**6. Component Structure**
*   `Dialog` - Root component providing context
*   `DialogTrigger` - Element that opens the dialog
*   `DialogPortal` - Portal container for dialog content
*   `DialogOverlay` - Background overlay (optional)
*   `DialogContent` - Main dialog container
*   `DialogTitle` - Dialog title (required for accessibility)
*   `DialogDescription` - Dialog description (optional)
*   `DialogClose` - Element that closes the dialog
