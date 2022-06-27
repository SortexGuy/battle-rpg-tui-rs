mod ui_rendering;
mod characters;

use crossterm::{
    event::{self, Event as CEvent, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{
    io,
    sync::mpsc,
    thread,
    time::{Duration, Instant},
};
use tui::{backend::CrosstermBackend, Terminal};
use characters::*;

enum Event<I> {
    Input(I),
    Tick,
}

pub struct State {
    /// Max 4
    enemy_party: Vec<Character>,
    /// Max 4
    player_party: Vec<Character>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    enable_raw_mode().expect("can run in raw mode");

    //* Event loop
    let (tx, rx) = mpsc::channel();
    let tick_rate = Duration::from_millis(200);
    thread::spawn(move || {
        let mut last_tick = Instant::now();
        loop {
            let timeout = tick_rate
                .checked_sub(last_tick.elapsed())
                .unwrap_or_else(|| Duration::from_secs(0));

            if event::poll(timeout).expect("polling failed") {
                if let CEvent::Key(key) = event::read().expect("couldn\'t read events") {
                    tx.send(Event::Input(key)).expect("couldn\'t send events");
                }
            }

            if last_tick.elapsed() >= tick_rate {
                if let Ok(_) = tx.send(Event::Tick) {
                    last_tick = Instant::now();
                }
            }
        }
    });

    //* Setup terminal output
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;

    //* Setup game
    let mut state = State {
        enemy_party: vec![
            Character {
                name: "Enemigo".to_string(),
                stats: Stats { attack: 5, defense: 5,hope: 2, },
                health: 23, max_health: 100,
                mana: 82, max_mana: 100,
                time_mod: 2.0, ..Default::default()
            },
            Character {
                name: "Enemigo2".to_string(),
                stats: Stats { attack: 5, defense: 5,hope: 2, },
                health: 23, max_health: 100,
                mana: 82, max_mana: 100,
                time_mod: 2.0, ..Default::default()
            },
            Character {
                name: "Enemigo3".to_string(),
                stats: Stats { attack: 5, defense: 5,hope: 2, },
                health: 23, max_health: 100,
                mana: 82, max_mana: 100,
                time_mod: 2.0, ..Default::default()
            },
        ],
        player_party: vec![
            Character {
                name: "Personaje".to_string(),
                stats: Stats { attack: 5, defense: 4, hope: 3, },
                health: 78, max_health: 100,
                mana: 45, max_mana: 100,
                time_mod: 3.0, ..Default::default()
            }
        ],
    };

    let mut delta = 0.0;
    let cerrar = false;
    //* Main loop
    while !cerrar {
        let time = Instant::now();

        update_chars_time(&mut state, delta);

        //* Render job
        ui_rendering::draw(&mut terminal, &state)?;

        //* Event handler
        match rx.recv()? {
            Event::Input(event) => match event.code {
                //* if Input
                KeyCode::Char('q') => {
                    disable_raw_mode()?;
                    terminal.show_cursor()?;
                    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
                    break;
                }
                KeyCode::Up | KeyCode::Char('w') => {
                    if !(state.player_party.len() >= 4) {
                        state
                            .player_party
                            .append(vec![state.player_party[0].clone()].as_mut());
                    } else {
                        state.player_party.pop();
                        state.player_party.pop();
                        state.player_party.pop();
                    }
                }
                KeyCode::Down | KeyCode::Char('s') => {
                    if !(state.player_party.len() <= 1) {
                        state.player_party.pop();
                    } else {
                        state.player_party.append(
                            vec![
                                state.player_party[0].clone(),
                                state.player_party[0].clone(),
                                state.player_party[0].clone(),
                            ]
                            .as_mut(),
                        );
                    }
                }
                _ => {}
            },
            Event::Tick => {}
        }

        delta = time.elapsed().as_secs_f32();
    }
    Ok(())
}
