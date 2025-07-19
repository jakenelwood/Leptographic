// Phase 0: Hook Infrastructure for Leptographic Components
// Inspired by leptos-use patterns for clean, composable state management

// Layer 1: Core Utility Hooks (Foundation)
// pub mod use_composed_refs;  // TODO: Fix NodeRef type issues
pub mod use_controllable_state;
pub mod use_escape_key;
// pub mod use_focus_trap;     // TODO: Fix NodeRef type issues
pub mod use_id_generator;
// pub mod use_outside_click;  // TODO: Fix NodeRef type issues
pub mod use_previous;

// Layer 2: Component-Specific Hooks (Building Blocks)
pub mod use_checkbox_state;
pub mod use_progress_state;
// pub mod use_radio_group_state;     // TODO: Implement
// pub mod use_slider_state;          // TODO: Implement
pub mod use_switch_state;

// Layer 3: Behavior Hooks (Complex Interactions)
// pub mod use_dialog_behavior;       // TODO: Implement
// pub mod use_dropdown_behavior;     // TODO: Implement
// pub mod use_tooltip_behavior;      // TODO: Implement

// Layer 4: Integration Hooks (External Systems)
// pub mod use_accessibility_announcer; // TODO: Implement
// pub mod use_form_integration;      // TODO: Implement

// Re-exports for easy access
pub use use_checkbox_state::*;
pub use use_controllable_state::*;
pub use use_progress_state::*;
pub use use_switch_state::*;

// Core utilities
// pub use use_composed_refs::*;  // TODO: Fix NodeRef type issues
pub use use_escape_key::*;
// pub use use_focus_trap::*;     // TODO: Fix NodeRef type issues
pub use use_id_generator::*;
// pub use use_outside_click::*;  // TODO: Fix NodeRef type issues
pub use use_previous::*;

// Component-specific
// pub use use_radio_group_state::*;     // TODO: Implement
// pub use use_slider_state::*;          // TODO: Implement

// Behavior hooks
// pub use use_dialog_behavior::*;       // TODO: Implement
// pub use use_dropdown_behavior::*;     // TODO: Implement
// pub use use_tooltip_behavior::*;      // TODO: Implement

// Integration hooks
// pub use use_accessibility_announcer::*; // TODO: Implement
// pub use use_form_integration::*;      // TODO: Implement

// Re-export commonly used leptos-use hooks for convenience
// Note: Add leptos-use as dependency when ready to integrate
// pub use leptos_use::{
//     use_element_hover, use_event_listener,
//     use_document_visibility, use_media_query
// };
