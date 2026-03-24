#[derive(Debug, Clone)]
pub enum Message {
    WindowDrag,
    CloseRequested,
    MinimizeRequested,
}