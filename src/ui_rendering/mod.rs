use crate::{State};
use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::*,
    widgets::*,
    Frame, Terminal,
};

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

    // TODO: make enemies panel

    let blocko = Block::default().title("Enemigos").borders(Borders::all());
    rect.render_widget(blocko, chunks[0]);
    build_enemies_section(rect, state, &chunks[0]);

    // TODO: make actions panel

    let blocko = Block::default().title("Acciones").borders(Borders::all());
    rect.render_widget(blocko, chunks[1]);

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
                    (100 / state.enemy_party.len()) as u16
                )].as_mut(),
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
        
        let gauge = Gauge::default()
            .block(Block::default().borders(Borders::LEFT | Borders::RIGHT))
            .gauge_style(
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            )
            .percent((enemy.mana as f32 * 100.0 / enemy.max_mana as f32).round() as u16)
            .label("Mana");
        rect.render_widget(gauge, player_chunks[0]);

        let gauge = Gauge::default()
            .block(Block::default().borders(Borders::LEFT | Borders::RIGHT))
            .gauge_style(Style::default().fg(Color::Red).add_modifier(Modifier::BOLD))
            .percent((enemy.health as f32 * 100.0 / enemy.max_health as f32).round() as u16)
            .label("Health");
        rect.render_widget(gauge, player_chunks[1]);

        let p_name = Paragraph::new(enemy.name.clone())
            .style(Style::default())
            .block(Block::default().borders(Borders::LEFT | Borders::RIGHT))
            .alignment(Alignment::Center);
        rect.render_widget(p_name, player_chunks[2]);

        let gauge = Gauge::default()
            .block(Block::default().borders(Borders::LEFT | Borders::RIGHT))
            .gauge_style(
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::SLOW_BLINK),
            )
            .percent((enemy.time / 60.0 * 100.0).round() as u16)
            .label("Time")
            .use_unicode(true);
        rect.render_widget(gauge, player_chunks[3]);
    }
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

        let gauge = Gauge::default()
            .block(Block::default().borders(Borders::LEFT | Borders::RIGHT))
            .gauge_style(
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::SLOW_BLINK),
            )
            .percent((player.time / 60.0 * 100.0).round() as u16)
            .label("Time")
            .use_unicode(true);
        rect.render_widget(gauge, player_chunks[0]);

        let p_name = Paragraph::new(player.name.clone())
            .style(Style::default())
            .block(Block::default().borders(Borders::LEFT | Borders::RIGHT))
            .alignment(Alignment::Center);
        rect.render_widget(p_name, player_chunks[1]);

        let gauge = Gauge::default()
            .block(Block::default().borders(Borders::LEFT | Borders::RIGHT))
            .gauge_style(Style::default().fg(Color::Red).add_modifier(Modifier::BOLD))
            .percent((player.health as f32 * 100.0 / player.max_health as f32).round() as u16)
            .label(format!("Health: {}/{}", player.health, player.max_health));
        rect.render_widget(gauge, player_chunks[2]);

        let gauge = Gauge::default()
            .block(Block::default().borders(Borders::LEFT | Borders::RIGHT))
            .gauge_style(
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            )
            .percent((player.mana as f32 * 100.0 / player.max_mana as f32).round() as u16)
            .label(format!("Mana: {}/{}", player.mana, player.max_mana));
        rect.render_widget(gauge, player_chunks[3]);
    }
}
