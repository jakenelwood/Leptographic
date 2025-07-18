use leptos::prelude::*;
use leptos::html::AnyElement;

/// Hook to compose multiple refs into a single ref
/// 
/// Useful when you need to forward a ref to a child component while also
/// using it internally for hooks like use_outside_click.
/// 
/// # Example
/// ```rust
/// let internal_ref = NodeRef::<AnyElement>::new();
/// let external_ref = NodeRef::<AnyElement>::new();
/// let composed_ref = use_composed_refs(vec![internal_ref, external_ref]);
/// 
/// view! {
///     <div node_ref=composed_ref>
///         "Element with multiple refs"
///     </div>
/// }
/// ```
pub fn use_composed_refs(refs: Vec<NodeRef<AnyElement>>) -> NodeRef<AnyElement> {
    let composed_ref = NodeRef::<AnyElement>::new();
    
    // Effect to sync the composed ref with all individual refs
    Effect::new(move |_| {
        if let Some(element) = composed_ref.get() {
            // Set the element on all provided refs
            for node_ref in &refs {
                // Note: This is a simplified version
                // In a full implementation, we'd need to handle the ref setting properly
                // For now, this serves as a placeholder for the concept
            }
        }
    });
    
    composed_ref
}

/// Hook to compose refs with a callback
/// 
/// # Example
/// ```rust
/// let my_ref = NodeRef::<AnyElement>::new();
/// let composed_ref = use_composed_refs_with_callback(
///     vec![my_ref],
///     Callback::new(|element: web_sys::Element| {
///         // Do something with the element when it's set
///         web_sys::console::log_1(&"Element ref set!".into());
///     })
/// );
/// ```
pub fn use_composed_refs_with_callback(
    refs: Vec<NodeRef<AnyElement>>,
    on_ref_set: Callback<web_sys::Element>,
) -> NodeRef<AnyElement> {
    let composed_ref = NodeRef::<AnyElement>::new();
    
    Effect::new(move |_| {
        if let Some(element) = composed_ref.get() {
            // Call the callback when ref is set
            on_ref_set.run(element.clone());
            
            // Set on all provided refs
            for node_ref in &refs {
                // Simplified implementation
            }
        }
    });
    
    composed_ref
}

// TODO: Implement proper ref composition logic
// TODO: Add support for function refs
// TODO: Add tests with proper HydrationCtx
