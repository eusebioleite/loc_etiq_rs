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

    // Event that handles toggling all visible stock locations
    ToggleAllVisible(bool),

    // Event that handles changes in the new location text input
    NewLocationInputChanged(String),

    // Event that handles the addition of a new location
    AddLocation,

    // Event that handles deleting an existing location
    DeleteLocation(String),

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
