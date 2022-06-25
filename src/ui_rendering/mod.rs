
use tui::{
    backend::Backend,
    layout::{Layout, Direction, Constraint, Rect, Alignment},
    style::*,
    widgets::*,
    Frame,
    Terminal
};
use crate::{Character, State};

pub fn draw<B: Backend>(
    term: &mut Terminal<B>,
    player: &Character,
    state: &State, 
) -> Result<(), Box<dyn std::error::Error>> {
    term.draw(|rect| {
        term_ui(rect, player, state);
    })?;
    Ok(())
}

fn term_ui<B: Backend>(
    rect: &mut Frame<B>,
    player: &Character,
    state: &State, 
) {
    let mut size = rect.size();
    if size.height%2 != 0 { size.height -= 1; }
    if size.width%2 != 0 { size.width -= 1; }

    let chunks = Layout::default()
        // * La direccion en la que se va a separar el espacio
        .direction(Direction::Vertical)
        //* Separacion entre separaciones
        .margin(2)
        //* Las restricciones de cada separacion
        .constraints([
                Constraint::Percentage(20),
                Constraint::Min(4),
                Constraint::Percentage(20)
            ].as_ref()
        )
        //* Separar segun el tamaño del terminal
        .split(size);

    // TODO: make enemies panel

    let blocko = Block::default()
        .title("Enemigos")
        .borders(Borders::all());
    rect.render_widget(blocko, chunks[0]);

    // TODO: make actions panel

    let blocko = Block::default()
        .title("Acciones")
        .borders(Borders::all());
    rect.render_widget(blocko, chunks[1]);
    
    //* Making player characters panel
    let blocko = Block::default()
        .title("Personajes")
        .borders(Borders::all());
    rect.render_widget(blocko, chunks[2]);
    build_characters_section(rect, player, state, &chunks[2]);
}

fn build_characters_section<B: Backend>(
    rect: &mut Frame<B>,
    player: &Character,
    state: &State,
    chunk: &Rect
) {

    let mut constraints: Vec<Constraint> = vec![];
    if state.player_party > 1 {
        for _ in 0..state.player_party {
            constraints.append(
                vec![Constraint::Percentage((100/state.player_party) as u16)]
                .as_mut()
            );
        }
    } else {
        constraints.append(
            vec![Constraint::Percentage(100u16)].as_mut()
        );
    }

    
    let mut party_chunks = Layout::default()
    .direction(Direction::Horizontal)
    .horizontal_margin(1)
    .constraints(constraints)
    .split(*chunk);
    
    let mut players_chunks = {
        for p_idx in 0..state.player_party {
            
        }
    };

    for c_chunk in party_chunks {
        let player_chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([
                    Constraint::Percentage(25),
                    Constraint::Percentage(25),
                    Constraint::Percentage(25),
                    Constraint::Percentage(25),
                ].as_ref()
            ).split(c_chunk);
        
        let gauge = Gauge::default()
            .block(Block::default().borders(Borders::LEFT | Borders::RIGHT))
            .gauge_style(
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::SLOW_BLINK)
            )
            .percent(player.time.round() as u16)
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
            .gauge_style(
                Style::default()
                    .fg(Color::Red)
                    .add_modifier(Modifier::BOLD)
            )
            .percent(((145.0f32/180.0f32)*100.0f32).round() as u16)
            .label("Health");
        rect.render_widget(gauge, player_chunks[2]);

        let gauge = Gauge::default()
            .block(Block::default().borders(Borders::LEFT | Borders::RIGHT))
            .gauge_style(
                Style::default()
                    .fg(Color::Blue)
                    .add_modifier(Modifier::BOLD)
            )
            .percent(((120.0f32/180.0f32)*100.0f32).round() as u16)
            .label("Mana");
        rect.render_widget(gauge, player_chunks[3]);
    }
}

