use iced::alignment::Horizontal;
use iced::widget::table::{self};
use iced::widget::{button, column, container, scrollable, text, text_input};
use iced::{window, Element, Length, Task, Theme};

// MAIN LOOP

#[derive(Debug, Clone)]
struct Estoque {
    local: String,
}

struct State {
    locais_estoque: Vec<Estoque>,
}

impl Default for State {
    fn default() -> Self {
        let raw_data = "INSPEC,NC,ALMOX,DEV,EXP,MOT,PA,PROC,DIVERG,RETTER,TERCEIROS,TERCEIRO,EMP-PDV,00011,00006,INJ010,533,524,SOP020,516,INJ013,SOP026,SOP030,SOP031,INJ023,SOP024,SOP032,SOP033,INJ022,923510001,ISBM 01,ISBM 02,ISBM 03,ISBM 04,ISBM 05,ISBM 06,ISBM 07,SOP013,CD-FAB-SOP,CD-FAB-INJ,SOP001,SOP015,SOP029,SOP006,SOP021,SOP028,INJ014,INJ011,INJ009,ALMOXSOPRO,SOP515,SOP035,SOP034,SOP002,SOP536,SOP011,SOP004,SOP008,SOP025,SOP027,SOP037,SOP036,SOP012,SOP017,SOP022,IBM 08,04874,AMX-999,CD-999,PROC-SOP,PROC-INJ,SOP023,SOP005,INJ008,SOP000,SOP014,SOP007,SOP009,SOP010,BOSS,00271,E-00271,E-04874,E-01907,I-04362,I-00053,I-04230,I-04811,I-00014,I-01817,I-03014,I-01349,I-02686,I-03414,E-05355,AMX-MM-SOP,AMX-MM-INJ,AMX-MM-MP,AMX-JAC,AMX-MAR,CD-MM,CD-JAC,CD-MAR,CD-MM-FR,518,INJ016,SOP003,WMS,WMS-SAIDAS,INJ007,INJ017,INJ001,INJ002,INJ003,INJ004,INJ005,INJ006,INJ012,INJ015,INJ018,INJ019,INJ020,INJ021,INJ024,SOP018,SOP016,MOLDBRASIL,ISBM 08,I-002328,CD-MM-PREF";
        
        // Split e conversão para Vec<Estoque>
        let locais_estoque = raw_data
            .split(',')
            .map(|s| Estoque { local: s.to_string() })
            .collect();

        Self { locais_estoque }
    }
}

#[derive(Debug, Clone)]
enum Message {
}


pub fn iced_run() -> iced::Result {
    let window_settings = window::Settings {
        size: iced::Size::new(480.0, 480.0),
        resizable: true,
        ..Default::default()
    };

    iced::application(
        || State::default(),
        update,
        view,
    )
    .title(|_state: &State| "Etiquetas de Estoque".to_string())
    .window(window_settings)
    .theme(|_state: &State| Theme::GruvboxLight)
    .centered()
    .run()
}
fn update(state: &mut State, message: Message) -> iced::Task<Message> {
    Task::none()
}

fn view(state: &State) -> Element<'_, Message> {
let header = row![
        text("Local de Estoque")
            .width(Length::Fill)
            .horizontal_alignment(Horizontal::Center),
    ]
    .padding(8)
    .spacing(8)
    .width(Length::Fill);
let rows = state.locais_estoque.iter().map(|estoque| {
        row![
            text(&estoque.local)
                .width(Length::Fill),
        ]
        .padding(8)
        .spacing(8)
        .width(Length::Fill)
        .into()
    });
let body = scrollable(
        column(rows)
            .width(Length::Fill)
            .spacing(4),
    )
    .width(Length::Fill);
let tabela = column![
        // Cabeçalho fixo
        container(header)
            .width(Length::Fill)
            .style(|theme| container::Appearance {
                background: Some(iced::Color::from_rgb8(220, 220, 220).into()),
                text_color: Some(iced::Color::BLACK),
                ..Default::default()
            }),

        // Corpo rolável
        body,
    ]
    .spacing(0);

    let columns = vec![
        table::column("Local de Estoque", |row: &Estoque| {
            text(&row.local)
        }),
    ];
    
    let content = column![
        text_input("Pesquisar...", "")
            .padding(10),
        container(tabela)
            .width(Length::Fill)
            .height(Length::FillPortion(8)),
        button("Gerar")
            .padding(10),
    ]
    .spacing(10)
    .align_x(Horizontal::Center)
    .max_width(480);
    
    container(content)
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x(Length::Fill)
        .center_y(Length::Fill)
        .into()
}