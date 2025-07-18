use leptos::prelude::*;

/// Hook to track the previous value of a signal
///
/// Useful for animations, transitions, and comparing state changes.
///
/// # Example
/// ```rust
/// let (count, set_count) = signal(0);
/// let previous_count = use_previous(count);
///
/// // After set_count(5):
/// // count.get() == 5
/// // previous_count.get() == Some(0)
/// ```
pub fn use_previous<T>(value: Signal<T>) -> Signal<Option<T>>
where
    T: Clone + PartialEq + Send + Sync + 'static,
{
    let (previous, set_previous) = signal(None::<T>);

    // Track changes to the value
    Effect::new(move |_| {
        let current = value.get();
        set_previous.update(|prev| {
            *prev = Some(current);
        });
    });

    previous.into()
}

/// Hook to track previous value with custom comparison
///
/// Only updates previous value when the comparison function returns true.
///
/// # Example
/// ```rust
/// let (user, set_user) = signal(User { id: 1, name: "Alice" });
/// let previous_user = use_previous_with(user, |prev, curr| prev.id != curr.id);
///
/// // Only tracks when user ID changes, ignoring name changes
/// ```
pub fn use_previous_with<T, F>(value: Signal<T>, should_update: F) -> Signal<Option<T>>
where
    T: Clone + Send + Sync + 'static,
    F: Fn(&T, &T) -> bool + Send + Sync + 'static,
{
    let (previous, set_previous) = signal(None::<T>);
    let (last_value, set_last_value) = signal(None::<T>);

    Effect::new(move |_| {
        let current = value.get();

        if let Some(last) = last_value.get() {
            if should_update(&last, &current) {
                set_previous.set(Some(last));
            }
        }

        set_last_value.set(Some(current));
    });

    previous.into()
}

/// Return type for use_previous_detailed hook
pub struct UsePreviousDetailedReturn<T: Send + Sync + 'static> {
    /// Previous value
    pub previous: Signal<Option<T>>,
    /// Whether the value has changed
    pub has_changed: Signal<bool>,
    /// Whether this is the first render
    pub is_first_render: Signal<bool>,
}

/// Hook that provides detailed information about value changes
///
/// # Example
/// ```rust
/// let (count, set_count) = signal(0);
/// let previous_info = use_previous_detailed(count);
///
/// // Access detailed information
/// let has_changed = previous_info.has_changed.get();
/// let is_first = previous_info.is_first_render.get();
/// ```
pub fn use_previous_detailed<T>(value: Signal<T>) -> UsePreviousDetailedReturn<T>
where
    T: Clone + PartialEq + Send + Sync + 'static,
{
    let (previous, set_previous) = signal(None::<T>);
    let (has_changed, set_has_changed) = signal(false);
    let (is_first_render, set_is_first_render) = signal(true);

    Effect::new(move |_| {
        let current = value.get();

        if let Some(prev) = previous.get() {
            let changed = prev != current;
            set_has_changed.set(changed);
            set_is_first_render.set(false);
        } else {
            set_has_changed.set(false);
            set_is_first_render.set(true);
        }

        set_previous.set(Some(current));
    });

    UsePreviousDetailedReturn {
        previous: previous.into(),
        has_changed: has_changed.into(),
        is_first_render: is_first_render.into(),
    }
}

// TODO: Add tests with proper HydrationCtx
