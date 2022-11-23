use crate::{characters::Character,
    ui_rendering::*};
// use core::option::{Option::None, Option::Some};
use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::*,
    // text::Span,
    widgets::*,
    Frame,
    // Terminal,
};


pub fn build_enemies_section<B: Backend>(rect: &mut Frame<B>, party: &Vec<Character>, chunk: &Rect) {
    let constraints = {
        let p_len = party.len();
        vec![Constraint::Percentage((100 / p_len) as u16); p_len]
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
            .constraints(
                [
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
            // ! From file
            "Mana",
            Option::Some(Modifier::BOLD),
            true,
        );
        rect.render_widget(gauge, char_chunks[0]);

        let gauge = create_gauge(
            enemy.health as f32,
            enemy.max_health as f32,
            Color::Red,
            // ! From file
            "Health",
            Option::Some(Modifier::BOLD),
            true,
        );
        rect.render_widget(gauge, char_chunks[1]);

        let p_name = Paragraph::new(enemy.name.clone())
            .style(Style::default())
            .block(Block::default().borders(Borders::LEFT | Borders::RIGHT))
            .alignment(Alignment::Center);
        rect.render_widget(p_name, char_chunks[2]);

        let gauge = create_gauge(
            enemy.time,
            1.0f32,
            Color::Green,
            // ! From file
            "Time",
            Option::Some(Modifier::SLOW_BLINK),
            true,
        );
        rect.render_widget(gauge, char_chunks[3]);
    }
}

pub fn build_middle_panels<B: Backend>(rect: &mut Frame<B>, state: &mut UiState, chunk: &Rect) {
    let middle_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .horizontal_margin(0)
        .constraints(
            [
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

pub fn render_statefull_list<B: Backend>(rect: &mut Frame<B>, s_list: &mut StatefulList, chunk: &Rect) {
    let list = List::new(
        s_list
            .items
            .iter()
            .map(|s| ListItem::new(s.clone()))
            .collect::<Vec<ListItem>>(),
    )
    .block(
        Block::default()
            .title(s_list.title.clone())
            .borders(Borders::all()),
    )
    .style(Style::default().bg(if s_list.blocked {
        Color::Red
    } else {
        Color::Reset
    }))
    .highlight_style(Style::default().add_modifier(Modifier::BOLD | Modifier::ITALIC))
    .highlight_symbol(">>");
    rect.render_stateful_widget(list, *chunk, &mut s_list.state);
}

pub fn build_characters_section<B: Backend>(rect: &mut Frame<B>, party: &Vec<Character>, chunk: &Rect) {
    let constraints = {
        let p_len = party.len();
        vec![Constraint::Percentage((100 / p_len) as u16); p_len]
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
            .constraints(
                [
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
            // ! From file
            "Time",
            Option::Some(Modifier::SLOW_BLINK),
            false,
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
            // ! From file
            "Health",
            Option::Some(Modifier::BOLD),
            false,
        );
        rect.render_widget(gauge, char_chunks[2]);

        let gauge = create_gauge(
            player.mana as f32,
            player.max_mana as f32,
            Color::Cyan,
            // ! From file
            "Mana",
            Option::Some(Modifier::BOLD),
            false,
        );
        rect.render_widget(gauge, char_chunks[3]);
    }
}

fn create_gauge(
    value: f32,
    max: f32,
    color: Color,
    name: &str,
    mods: Option<Modifier>,
    enemy: bool,
) -> Gauge {
    let percent: u16 = if max == 1.0 {
        (value / 60.0 * 100.0).round()
    }
    // Percentage of time
    else {
        (value * 100.0 / max).round()
    } as u16;
    Gauge::default()
        .block(Block::default().borders(Borders::LEFT | Borders::RIGHT))
        .gauge_style(
            Style::default()
                .fg(color)
                .add_modifier(mods.unwrap_or(Modifier::empty())),
        )
        .percent(percent)
        .label(if max != 1.0 {
            if !enemy {
                format!("{}: {}/{}", name, value as u16, max as u16)
            } else {
                format!("{}: {}", name, value as u16)
            }
        } else if !enemy {
            format!("{}: {}/{}", name, percent, 100u16)
        } else {
            format!("{}: {}", name, percent)
        })
}
