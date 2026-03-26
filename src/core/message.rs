#[derive(Debug, Clone)]
pub enum Message {
    // Slider Qtd Copias
    CopiesChanged(i32),

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
