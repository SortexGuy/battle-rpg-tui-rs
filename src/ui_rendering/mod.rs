mod battle_blocks;

use crate::{characters::Character, AppState, BattleState, Commands};
use battle_blocks::*;
use core::option::{Option::None, Option::Some};
use crossterm::event::{KeyCode, KeyEvent};
use std::fmt::Display;
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    // text::Span,
    widgets::*,
    Frame,
    Terminal,
};

pub struct UiState {
    pub enemy_party: Option<Vec<Character>>,
    pub player_party: Option<Vec<Character>>,

    pub from: StatefulList,
    pub what: StatefulList,
    pub which: StatefulList,
    pub to: StatefulList,
}
impl UiState {
    pub fn handle_events(
        &mut self,
        app_state: &mut AppState,
        battle_state: &mut BattleState,
        event: KeyEvent,
    ) {
        match event.code {
            //* if Input
            KeyCode::Char('q') => {
                app_state.should_quit = true;
                return;
            }
            KeyCode::Char('e') => {
                let mut done = false;
                for player in battle_state.player_party.iter_mut() {
                    if done {
                        return;
                    }
                    if !player.cmd_available.contains(&Commands::Magic) {
                        player.add_action(&Commands::Magic);
                        done = true;
                    } else if !player.cmd_available.contains(&Commands::Manif) {
                        player.add_action(&Commands::Manif);
                        done = true;
                    }
                }
            }
            KeyCode::Up | KeyCode::Char('w') => {
                self.prev();
            }
            KeyCode::Down | KeyCode::Char('s') => {
                self.next();
            }
            KeyCode::Left | KeyCode::Char('a') => {
                self.unselect();
            }
            KeyCode::Right | KeyCode::Char('d') | KeyCode::Enter | KeyCode::Char(' ') => {
                self.select();
            }
            _ => {}
        }
    }

    pub fn populate(&mut self, b_state: &BattleState) {
        let enemy_party = &b_state.enemy_party;
        let player_party = &b_state.player_party;

        if !enemy_party.is_empty() {
            self.enemy_party = Some(enemy_party.clone());
        } else {
            self.enemy_party = None;
        }
        if !player_party.is_empty() {
            self.player_party = Some(player_party.clone());
        } else {
            self.player_party = None;
        }

        self.from.change_items(player_party);
        if let Some(i) = self.from.state.selected() {
            let char = &player_party[i];
            self.what.change_items(&char.cmd_available);

            if let Some(i) = self.what.state.selected() {
                let cmd = char.cmd_available[i];
                self.which.change_items(&char.act_available[cmd as usize]);
            }
        }
        let targets = enemy_party
            .iter()
            .cloned()
            .chain(player_party.iter().cloned())
            .collect::<Vec<Character>>();
        self.to.change_items(&targets);
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
        if !self.from.blocked && self.from.state.selected().is_some() {
            self.from.select();
        } else if !self.what.blocked && self.what.state.selected().is_some() {
            self.what.select();
        } else if !self.which.blocked && self.which.state.selected().is_some() {
            self.which.select();
        } else if !self.to.blocked && self.to.state.selected().is_some() {
            self.unselect_all()
            // TODO: Push action to the queue
        }
    }

    pub fn unselect(&mut self) {
        if self.which.blocked {
            self.which.unselect();
        } else if self.what.blocked {
            self.what.unselect();
        } else if self.from.blocked {
            self.from.unselect();
        }
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
            blocked: false,
            title: title.to_string(),
            state: ListState::default(),
            items,
        }
    }

    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn prev(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
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
        self.items = items.iter().map(|c| c.to_string()).collect::<Vec<String>>();
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
        .constraints(
            [
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
