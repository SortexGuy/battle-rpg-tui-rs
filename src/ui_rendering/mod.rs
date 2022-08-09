use crate::{characters::Character, BattleState};
use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::*,
    text::Span,
    widgets::*,
    Frame, Terminal,
};
use core::option::{Option::Some, Option::None};
use std::fmt::{self, Display};

pub struct UiState {
    pub enemy_party: Option<Vec<Character>>, 
    pub player_party: Option<Vec<Character>>, 

    pub from: StatefulList,
    pub what: StatefulList,
    pub which: StatefulList,
    pub to: StatefulList,
}
impl UiState {
    pub fn populate(&mut self, b_state: &BattleState) {
        if !b_state.enemy_party.is_empty() {
            self.enemy_party = Some(b_state.enemy_party.clone());
        } else { self.enemy_party = None; }
        if !b_state.player_party.is_empty() {
            self.player_party = Some(b_state.player_party.clone());
        } else { self.player_party = None; }
        self.from.change_items(&b_state.player_party);
        if let Some(i) = self.from.state.selected() {
            self.what.change_items(&b_state.player_party[i].act_available);
        }
        self.which.change_items(&["Asies"]);
        self.to.change_items(&b_state.enemy_party);
    }
    fn unselect_all(&mut self) {
        self.from.unselect();
        self.what.unselect();
        self.which.unselect();
        self.to.unselect();
    }
    pub fn prev(&mut self) {
        if !self.from.blocked {
            self.from.prev();
        } else if !self.what.blocked {
            self.what.prev();
        } else if !self.which.blocked {
            self.which.prev();
        } else {
            self.to.prev();
        }
    }
    pub fn next(&mut self) {
        if !self.from.blocked {
            self.from.next();
        } else if !self.what.blocked {
            self.what.next();
        } else if !self.which.blocked {
            self.which.next();
        } else {
            self.to.next();
        }
    }
    pub fn select(&mut self) {
        if !self.from.blocked { self.from.select(); }
        else if !self.what.blocked { self.what.select(); }
        else if !self.which.blocked { self.which.select(); }
        else if !self.to.blocked { 
            self.unselect_all()
            // TODO: Push action to the queue
        }
    }
    pub fn unselect(&mut self) {
        if self.which.blocked { self.which.unselect(); }
        else if self.what.blocked { self.what.unselect(); }
        else if self.from.blocked { self.from.unselect(); }
    }
}

pub struct StatefulList {
    blocked: bool, 
    title: String, 
    state: ListState, 
    items: Vec<String>, 
}
impl StatefulList {
    pub fn with_items(items: Vec<String>, title: &str) -> StatefulList {
        StatefulList { 
            title: title.to_string(), 
            blocked: false, 
            state: ListState::default(), 
            items, 
        }
    }
    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 { 0 } 
                else { i + 1 }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }
    pub fn prev(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 { self.items.len() - 1 } 
                else { i - 1 }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }
    pub fn select(&mut self) {
        self.blocked = true;
    }
    pub fn unselect(&mut self) {
        self.blocked = false;
        self.state.select(None);
    }
    pub fn change_items<C: Display + Clone>(&mut self, items: &[C]) {
        self.items = items.iter()
            .map(|c| c.to_string())
            .collect::<Vec<String>>();
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ActionOptions {
    Attack,
    Defend,
    Magic,
    Ability,
    Manif,
}
impl Display for ActionOptions {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub fn draw<B: Backend>(
    term: &mut Terminal<B>,
    state: &mut UiState,
) -> Result<(), Box<dyn std::error::Error>> {
    term.draw(|rect| {
        term_ui(rect, state);
    })?;
    Ok(())
}

fn term_ui<B: Backend>(rect: &mut Frame<B>, state: &mut UiState) {
    let mut size = rect.size();
    if size.height % 2 != 0 {
        size.height -= 1;
    }
    if size.width % 2 != 0 {
        size.width -= 1;
    }

    let chunks = Layout::default()
        // * La direccion en la que se va a separar el espacio
        .direction(Direction::Vertical)
        //* Separacion entre separaciones
        .margin(1)
        //* Las restricciones de cada separacion
        .constraints( [
                Constraint::Length(6),
                Constraint::Min(6),
                Constraint::Length(6),
            ]
            .as_ref(),
        )
        //* Separar segun el tamaño del terminal
        .split(size);
    
    //* Making enemies panel
    let blocko = Block::default().title("Enemigos").borders(Borders::all());
    rect.render_widget(blocko, chunks[0]);
    if let Some(party) = &state.enemy_party {
        build_enemies_section(rect, party, &chunks[0]);
    }

    //* Making middle panels
    build_middle_panels(rect, state, &chunks[1]);
    
    //* Making player characters panel
    let blocko = Block::default().title("Personajes").borders(Borders::all());
    rect.render_widget(blocko, chunks[2]);
    if let Some(party) = &state.player_party {
        build_characters_section(rect, party, &chunks[2]);
    }
}

fn build_enemies_section<B: Backend>(
    rect: &mut Frame<B>, 
    party: &Vec<Character>, 
    chunk: &Rect, 
) {
    let constraints = {
        let p_len = party.len();
        vec![
            Constraint::Percentage((100 / p_len) as u16); p_len
        ]
    };

    let party_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .horizontal_margin(1)
        .constraints(constraints)
        .split(*chunk);

    for (i, e_chunk) in party_chunks.iter().enumerate() {
        let enemy = &party[i];
        let char_chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints( [
                    Constraint::Percentage(25),
                    Constraint::Percentage(25),
                    Constraint::Percentage(25),
                    Constraint::Percentage(25),
                ]
                .as_ref(),
            )
            .split(*e_chunk);

        let gauge = create_gauge(
            enemy.mana as f32, 
            enemy.max_mana as f32, 
            Color::Cyan, 
            "Mana",
            Option::Some(Modifier::BOLD),
            true
        );
        rect.render_widget(gauge, char_chunks[0]);

        let gauge = create_gauge(
            enemy.health as f32, 
            enemy.max_health as f32, 
            Color::Red, 
            "Health",
            Option::Some(Modifier::BOLD),
            true
        );
        rect.render_widget(gauge, char_chunks[1]);

        let p_name = Paragraph::new(enemy.name.clone())
            .style(Style::default())
            .block(Block::default().borders(Borders::LEFT | Borders::RIGHT))
            .alignment(Alignment::Center);
        rect.render_widget(p_name, char_chunks[2]);

        // let time = (enemy.time / 60.0 * 100.0).round() as u16;
        let gauge = create_gauge(
            enemy.time, 
            1.0f32, 
            Color::Green, 
            "Time", 
            Option::Some(Modifier::SLOW_BLINK),
            true
        );
        rect.render_widget(gauge, char_chunks[3]);
    }
}

fn build_middle_panels<B: Backend>(rect: &mut Frame<B>, state: &mut UiState, chunk: &Rect) {
    let middle_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .horizontal_margin(0)
        .constraints( [
            Constraint::Length(18), 
            Constraint::Length(18), 
            Constraint::Length(18), 
            Constraint::Length(18), 
            Constraint::Min(18), 
            ]
            .as_mut(), 
        )
        .split(*chunk);
    // Character
    render_statefull_list(rect, &mut state.from, &middle_chunks[0]);
    // Action list
    render_statefull_list(rect, &mut state.what, &middle_chunks[1]);
    // Which action list
    render_statefull_list(rect, &mut state.which, &middle_chunks[2]);
    // To who list
    render_statefull_list(rect, &mut state.to, &middle_chunks[3]);
}

fn render_statefull_list<B: Backend>(rect: &mut Frame<B>, s_list: &mut StatefulList, chunk: &Rect) {
    let list = List::new(
        s_list.items.iter().map(|s| {
            ListItem::new(s.clone())
        }).collect::<Vec<ListItem>>()
    )
        .block(Block::default().title(s_list.title.clone())
        .borders(Borders::all()))
        .highlight_style(
            Style::default().add_modifier(Modifier::BOLD | Modifier::ITALIC)
        )
        .highlight_symbol(">>");
    rect.render_stateful_widget(list, *chunk, &mut s_list.state);
}

// fn render_action_list<B: Backend>(rect: &mut Frame<B>, state: &State, chunk: &Rect) {
//     let act_available = &state.player_party[state.ui.from.0].act_available;
//     let list: List;
//     let mut menu_opt: Vec<ListItem> = vec![ListItem::new(""); act_available.len()];
//     for (i, act) in act_available.iter().enumerate() {
//         menu_opt[i] = ListItem::new(format!("{:?}", act));
//     }
//     list = List::new(menu_opt)
//         .block(Block::default().title("Que?")
//         .borders(Borders::all()))
//         .highlight_style(
//             Style::default().add_modifier(Modifier::BOLD | Modifier::ITALIC)
//         )
//         .highlight_symbol(">>");
//     let mut l_state = ListState::default();
//     l_state.select(
//         Some(state.ui.what.0)
//     );
//     rect.render_stateful_widget(list, *chunk, &mut l_state);
// }

// fn render_which_list<B: Backend>(rect: &mut Frame<B>, state: &State, chunk: &Rect) {
//     // let act_available = &state.player_party[state.ui.from.0].act_available;
//     // let j = state.player_party[state.ui.from.0].act_available[state.ui.what.0] as usize;
//     // state.player_party[state.ui.from.0].act_opt[j]
//     let list: List;
//     let menu_opt: Vec<ListItem> = vec![ListItem::new("Asies")];
//     // for (i, act) in act_available.iter().enumerate() {
//     //     menu_opt[i] = ListItem::new(format!("{:?}", act));
//     // }
//     list = List::new(menu_opt)
//         .block(Block::default().title("Cual?")
//         .borders(Borders::all()))
//         .highlight_style(
//             Style::default().add_modifier(Modifier::BOLD | Modifier::ITALIC)
//         )
//         .highlight_symbol(">>");
//     let mut l_state = ListState::default();
//     l_state.select(
//         Some(state.ui.which.0)
//     );
//     rect.render_stateful_widget(list, *chunk, &mut l_state);
// }

// fn render_to_list<B: Backend>(rect: &mut Frame<B>, state: &State, chunk: &Rect) {
//     let list: List;
//     let mut menu_opt: Vec<ListItem> = vec![ListItem::new(""); state.enemy_party.len() as usize];
//     for (i, chara) in state.enemy_party.iter().enumerate() {
//         menu_opt[i] = ListItem::new(format!("{}", chara.name));
//     }
//     list = List::new(menu_opt)
//         .block(Block::default().title("A quien?")
//         .borders(Borders::all()))
//         .highlight_style(
//             Style::default().add_modifier(Modifier::BOLD | Modifier::ITALIC)
//         )
//         .highlight_symbol(">>");
//     let mut l_state = ListState::default();
//     l_state.select(
//         Some(state.ui.to)
//     );
//     rect.render_stateful_widget(list, *chunk, &mut l_state);
// }

fn build_characters_section<B: Backend>(
    rect: &mut Frame<B>, 
    party: &Vec<Character>, 
    chunk: &Rect, 
) {
    let constraints = {
        let p_len = party.len();
        vec![
            Constraint::Percentage((100 / p_len) as u16); p_len
        ]
    };

    let party_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .horizontal_margin(1)
        .constraints(constraints)
        .split(*chunk);

    for (i, c_chunk) in party_chunks.iter().enumerate() {
        let player = &party[i];
        let char_chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints( [
                    Constraint::Percentage(25),
                    Constraint::Percentage(25),
                    Constraint::Percentage(25),
                    Constraint::Percentage(25),
                    ]
                    .as_ref(),
                )
                .split(*c_chunk);

        let gauge = create_gauge(
            player.time, 
            1.0f32, 
            Color::Green, 
            "Time", 
            Option::Some(Modifier::SLOW_BLINK),
            false
        );
        rect.render_widget(gauge, char_chunks[0]);

        let p_name = Paragraph::new(player.name.clone())
            .style(Style::default())
            .block(Block::default().borders(Borders::LEFT | Borders::RIGHT))
            .alignment(Alignment::Center);
        rect.render_widget(p_name, char_chunks[1]);

        let gauge = create_gauge(
            player.health as f32, 
            player.max_health as f32, 
            Color::Red, 
            "Health",
            Option::Some(Modifier::BOLD),
            false
        );
        rect.render_widget(gauge, char_chunks[2]);

        let gauge = create_gauge(
            player.mana as f32, 
            player.max_mana as f32, 
            Color::Cyan, 
            "Mana",
            Option::Some(Modifier::BOLD),
            false
        );
        rect.render_widget(gauge, char_chunks[3]);
    }
}

fn create_gauge(
    value: f32, max: f32,
    color: Color, name: &str,
    mods: Option<Modifier>,
    enemy: bool, 
) -> Gauge {
    let percent: u16 = if max == 1.0 { (value / 60.0 * 100.0).round() } // Percentage of time
        else { (value * 100.0 / max).round() } as u16;
    Gauge::default()
        .block(Block::default().borders(Borders::LEFT | Borders::RIGHT))
        .gauge_style(
            Style::default()
                .fg(color)
                .add_modifier(mods.unwrap_or(Modifier::empty())),
        )
        .percent(percent)
        .label(
            if max != 1.0 {
                if !enemy { format!("{}: {}/{}", name, value as u16, max as u16) }
                else { format!("{}: {}", name, value as u16) }
            } else {
                if !enemy { format!("{}: {}/{}", name, percent, 100u16) }
                else { format!("{}: {}", name, percent) }
            }
        )
}
