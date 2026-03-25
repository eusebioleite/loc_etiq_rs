#[derive(Debug, Clone)]
pub enum Message {
    // Searchbar
    SearchContents(String),

    // Handles Selection
    ToggleLocation(String, bool),

    // Success
    ClearNotification,

    // Default
    WindowDrag,
    CloseRequested,
    MinimizeRequested,
}
