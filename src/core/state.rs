use crate::core::printer::get_printers;

#[derive(Default, Clone)]
pub struct TableRow {
    pub description: String,
    pub selected: bool,
}

pub struct State {
    pub show_success: bool,
    pub show_error: bool,
    pub msg_toast: String,
    pub search_query: String,
    pub locations: Vec<TableRow>,
    pub printers: Vec<String>,
    pub selected_printer: Option<String>,
    pub count_copies: i32,
}

impl Default for State {
    fn default() -> Self {
        Self {
            show_success: false,
            show_error: false,
            msg_toast: String::new(),
            search_query: String::new(),
            locations: crate::config::get().locais.clone()
                .into_iter()
                .map(|local| TableRow {
                    description: local,
                    selected: false,
                })
                .collect(),
            printers: get_printers().expect("Error getting printers."),
            selected_printer: None,
            count_copies: 1,
        }
    }
}
