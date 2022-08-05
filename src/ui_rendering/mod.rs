use crate::{characters::Character, State};
use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::*,
    text::Spans,
    widgets::*,
    Frame, Terminal,
};
use core::option::{Option::Some, Option::None};

pub struct UiState {
    pub from: (usize, bool),
    pub what: (usize, bool),
    pub which: (usize, bool),
    pub to: usize,
}
impl UiState {
    pub fn prev(&mut self) {
        if !self.from.1 {
            if self.from.0 > 0 { self.from.0 -= 1; }
        }
        else if !self.what.1 {
            if self.what.0 > 0 { self.what.0 -= 1; }
        }
        else if !self.which.1 {
            return ;
            // if self.which.0 > 0 { self.which.0 -= 1; }
        } else {
            if self.to > 0 { self.to -= 1; }
        }
    }
    pub fn next(&mut self, p_len: usize, act_len: usize, e_len: usize) {
        if !self.from.1 {
            if self.from.0 < p_len-1 { self.from.0 += 1; }
        }
        else if !self.what.1 {
            if self.what.0 < act_len-1 { self.what.0 += 1; }
        }
        else if !self.which.1 {
            return ;
            // if self.which.0 < 1 { self.which.0 += 1; }
        } else {
            if self.to < e_len-1 { self.to += 1; }
        }
    }
    pub fn select(&mut self) {
        if !self.from.1 { self.from.1 = true; }
        else if !self.what.1 { self.what.1 = true; }
        else if !self.which.1 { self.which.1 = true; }
        else { 
            self.from.1 = false;
            self.what.1 = false;
            self.which.1 = false;
            // TODO: Push action to the queue
        }
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

pub fn draw<B: Backend>(
    term: &mut Terminal<B>,
    state: &State,
) -> Result<(), Box<dyn std::error::Error>> {
    term.draw(|rect| {
        term_ui(rect, state);
    })?;
    Ok(())
}

fn term_ui<B: Backend>(rect: &mut Frame<B>, state: &State) {
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
    build_enemies_section(rect, state, &chunks[0]);

    // TODO: make middle panel
    build_middle_panels(rect, state, &chunks[1]);

    //* Making player characters panel
    let blocko = Block::default().title("Personajes").borders(Borders::all());
    rect.render_widget(blocko, chunks[2]);
    build_characters_section(rect, state, &chunks[2]);
}

fn build_enemies_section<B: Backend>(rect: &mut Frame<B>, state: &State, chunk: &Rect) {
    let mut constraints: Vec<Constraint> = vec![];
    if state.enemy_party.len() > 1 {
        for _ in state.enemy_party.iter() {
            constraints.append(
                vec![Constraint::Percentage(
                    (100 / state.enemy_party.len()) as u16,
                )]
                .as_mut(),
            );
        }
    } else {
        constraints.append(vec![Constraint::Percentage(100u16)].as_mut());
    }

    let party_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .horizontal_margin(1)
        .constraints(constraints)
        .split(*chunk);

    for (i, e_chunk) in party_chunks.iter().enumerate() {
        let enemy = &state.enemy_party[i];
        let player_chunks = Layout::default()
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
        rect.render_widget(gauge, player_chunks[0]);

        let gauge = create_gauge(
            enemy.health as f32, 
            enemy.max_health as f32, 
            Color::Red, 
            "Health",
            Option::Some(Modifier::BOLD),
            true
        );
        rect.render_widget(gauge, player_chunks[1]);

        let p_name = Paragraph::new(enemy.name.clone())
            .style(Style::default())
            .block(Block::default().borders(Borders::LEFT | Borders::RIGHT))
            .alignment(Alignment::Center);
        rect.render_widget(p_name, player_chunks[2]);

        // let time = (enemy.time / 60.0 * 100.0).round() as u16;
        let gauge = create_gauge(
            enemy.time, 
            1.0f32, 
            Color::Green, 
            "Time", 
            Option::Some(Modifier::SLOW_BLINK),
            true
        );
        rect.render_widget(gauge, player_chunks[3]);
    }
}

fn build_middle_panels<B: Backend>(rect: &mut Frame<B>, state: &State, chunk: &Rect) {
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
    render_players_list(rect, state, &middle_chunks[0]);
    // Action list
    render_action_list(rect, state, &middle_chunks[1]);
    // Which action list
    render_which_list(rect, state, &middle_chunks[2]);
    // To who list
    render_to_list(rect, state, &middle_chunks[3]);
}

fn render_players_list<B: Backend>(rect: &mut Frame<B>, state: &State, chunk: &Rect) {
    let list: List;
    let mut menu_opt: Vec<ListItem> = vec![ListItem::new(""); state.player_party.len() as usize];
    for (i, chara) in state.player_party.iter().enumerate() {
        menu_opt[i] = ListItem::new(format!("{}", chara.name));
    }
    list = List::new(menu_opt)
        .block(Block::default().title("Quien?")
        .borders(Borders::all()))
        .highlight_style(
            Style::default().add_modifier(Modifier::BOLD | Modifier::ITALIC)
        )
        .highlight_symbol(">>");
    let mut l_state = ListState::default();
    l_state.select(
        Some(state.ui.from.0)
    );
    rect.render_stateful_widget(list, *chunk, &mut l_state);
}

fn render_action_list<B: Backend>(rect: &mut Frame<B>, state: &State, chunk: &Rect) {
    let act_available = &state.player_party[state.ui.from.0].act_available;
    let list: List;
    let mut menu_opt: Vec<ListItem> = vec![ListItem::new(""); act_available.len()];
    for (i, act) in act_available.iter().enumerate() {
        menu_opt[i] = ListItem::new(format!("{:?}", act));
    }
    list = List::new(menu_opt)
        .block(Block::default().title("Que?")
        .borders(Borders::all()))
        .highlight_style(
            Style::default().add_modifier(Modifier::BOLD | Modifier::ITALIC)
        )
        .highlight_symbol(">>");
    let mut l_state = ListState::default();
    l_state.select(
        Some(state.ui.what.0)
    );
    rect.render_stateful_widget(list, *chunk, &mut l_state);
}

fn render_which_list<B: Backend>(rect: &mut Frame<B>, state: &State, chunk: &Rect) {
    // let act_available = &state.player_party[state.ui.from.0].act_available;
    // let j = state.player_party[state.ui.from.0].act_available[state.ui.what.0] as usize;
    // state.player_party[state.ui.from.0].act_opt[j]
    let list: List;
    let menu_opt: Vec<ListItem> = vec![ListItem::new("Asies")];
    // for (i, act) in act_available.iter().enumerate() {
    //     menu_opt[i] = ListItem::new(format!("{:?}", act));
    // }
    list = List::new(menu_opt)
        .block(Block::default().title("Cual?")
        .borders(Borders::all()))
        .highlight_style(
            Style::default().add_modifier(Modifier::BOLD | Modifier::ITALIC)
        )
        .highlight_symbol(">>");
    let mut l_state = ListState::default();
    l_state.select(
        Some(state.ui.which.0)
    );
    rect.render_stateful_widget(list, *chunk, &mut l_state);
}

fn render_to_list<B: Backend>(rect: &mut Frame<B>, state: &State, chunk: &Rect) {
    let list: List;
    let mut menu_opt: Vec<ListItem> = vec![ListItem::new(""); state.enemy_party.len() as usize];
    for (i, chara) in state.enemy_party.iter().enumerate() {
        menu_opt[i] = ListItem::new(format!("{}", chara.name));
    }
    list = List::new(menu_opt)
        .block(Block::default().title("A quien?")
        .borders(Borders::all()))
        .highlight_style(
            Style::default().add_modifier(Modifier::BOLD | Modifier::ITALIC)
        )
        .highlight_symbol(">>");
    let mut l_state = ListState::default();
    l_state.select(
        Some(state.ui.to)
    );
    rect.render_stateful_widget(list, *chunk, &mut l_state);
}

fn build_characters_section<B: Backend>(rect: &mut Frame<B>, state: &State, chunk: &Rect) {
    let mut constraints: Vec<Constraint> = vec![];
    if state.player_party.len() > 1 {
        for _ in state.player_party.iter() {
            constraints.append(
                vec![Constraint::Percentage(
                    (100 / state.player_party.len()) as u16,
                )]
                .as_mut(),
            );
        }
    } else {
        constraints.append(vec![Constraint::Percentage(100u16)].as_mut());
    }

    let party_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .horizontal_margin(1)
        .constraints(constraints)
        .split(*chunk);

    for (i, c_chunk) in party_chunks.iter().enumerate() {
        let player = &state.player_party[i];
        let player_chunks = Layout::default()
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
        rect.render_widget(gauge, player_chunks[0]);

        let p_name = Paragraph::new(player.name.clone())
            .style(Style::default())
            .block(Block::default().borders(Borders::LEFT | Borders::RIGHT))
            .alignment(Alignment::Center);
        rect.render_widget(p_name, player_chunks[1]);

        let gauge = create_gauge(
            player.health as f32, 
            player.max_health as f32, 
            Color::Red, 
            "Health",
            Option::Some(Modifier::BOLD),
            false
        );
        rect.render_widget(gauge, player_chunks[2]);

        let gauge = create_gauge(
            player.mana as f32, 
            player.max_mana as f32, 
            Color::Cyan, 
            "Mana",
            Option::Some(Modifier::BOLD),
            false
        );
        rect.render_widget(gauge, player_chunks[3]);
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
