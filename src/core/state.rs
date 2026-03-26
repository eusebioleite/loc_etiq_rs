use crate::core::printer::get_printers;

const STRING_LOCAIS_ESTOQUE: &str =
    "00006,00011,00271,04874,516,518,524,533,923510001,ALMOX,ALMOXSOPRO,AMX-999,AMX-JAC,AMX-MAR,AMX-MM-INJ,AMX-MM-MP,AMX-MM-SOP,BOSS,CD-999,CD-FAB-INJ,CD-FAB-SOP,CD-JAC,CD-MAR,CD-MM ,CD-MM-FR,CD-MM-PREF,DEV,DIVERG,E-00271,E-00955,E-01907,E-02371,E-03247,E-04874,E-05355,E-05434,EMP-PDV,EXP,FABR,I-00014,I-00053,I-002328,I-01349,I-01817,I-02686,I-02939,I-02992,I-03014,I-03414,I-04136,I-04230,I-04362,I-04811,I-3247,IBM 08,INJ001,INJ002,INJ003,INJ004,INJ005,INJ006,INJ007,INJ008,INJ009,INJ010,INJ011,INJ012,INJ013,INJ014,INJ015,INJ016,INJ017,INJ018,INJ019,INJ020,INJ021,INJ022,INJ023,INJ024,INSPEC,ISBM 01,ISBM 02,ISBM 03,ISBM 04,ISBM 05,ISBM 06,ISBM 07,ISBM 08,MOLDBRASIL,MOT,NC,P032,PA,PROC,PROC-INJ,PROC-SOP,RETTER,SOP000,SOP001,SOP002,SOP003,SOP004,SOP005,SOP006,SOP007,SOP008,SOP009,SOP010,SOP011,SOP012,SOP013,SOP014,SOP015,SOP016,SOP017,SOP018,SOP020,SOP021,SOP022,SOP023,SOP024,SOP025,SOP026,SOP027,SOP028,SOP029,SOP030,SOP031,SOP032,SOP033,SOP034,SOP035,SOP036,SOP037,SOP515,SOP536,TERCEIRO,TERCEIROS,WMS,WMS-SAIDAS";

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
            locations: STRING_LOCAIS_ESTOQUE
                .split(',')
                .map(|l| TableRow {
                    description: l.to_string(),
                    selected: false,
                })
                .collect(),
            printers: get_printers().expect("Error getting printers."),
            selected_printer: None,
            count_copies: 1,
        }
    }
}
