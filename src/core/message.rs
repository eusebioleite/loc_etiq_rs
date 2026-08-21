#[derive(Debug, Clone)]
pub enum Message {
    // Event that sends a command to the printer
    Print,
    // Event that handles the selection of a printer in the dropdown
    PrinterSelected(String),

    // Event that handles changes in the slider that defines the amount of copies
    CopiesChanged(i32),

    // Event that handles changes in the Search Bar
    SearchContents(String),

    // Event that handles the selection of a new stock location
    ToggleLocation(String, bool),

    // Default events that handle taskbar actions
    ClearNotification,
    WindowDrag,
    CloseRequested,
    MinimizeRequested,
}

pub enum Toast {
    Success,
    Error,
}
