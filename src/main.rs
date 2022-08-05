mod characters;
mod ui_rendering;

use characters::*;
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
use ui_rendering::{ActionOptions, UiState};

enum Event<I> {
    Input(I),
    Tick,
}

pub struct State {
    /// Max 4
    enemy_party: Vec<Character>,
    /// Max 4
    player_party: Vec<Character>,
    ui: UiState,
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
                stats: Stats {
                    attack: 5,
                    defense: 5,
                    hope: 2,
                },
                health: 23,
                max_health: 100,
                mana: 82,
                max_mana: 100,
                ..Default::default()
            },
            Character {
                name: "Enemigo2".to_string(),
                stats: Stats {
                    attack: 5,
                    defense: 5,
                    hope: 2,
                },
                health: 23,
                max_health: 100,
                mana: 82,
                max_mana: 100,
                ..Default::default()
            },
            Character {
                name: "Enemigo3".to_string(),
                stats: Stats {
                    attack: 5,
                    defense: 5,
                    hope: 2,
                },
                health: 23,
                max_health: 100,
                mana: 82,
                max_mana: 100,
                ..Default::default()
            },
        ],
        player_party: vec![Character {
            name: "Personaje".to_string(),
            stats: Stats {
                attack: 5,
                defense: 4,
                hope: 3,
            },
            health: 78,
            max_health: 100,
            mana: 45,
            max_mana: 100,
            ..Default::default()
        }],
        ui: UiState {
            from: (0, false),
            what: (0, false),
            which: (0, false),
            to: 0,
        },
    };

    //? For demostration proposes
    state.player_party.append(
        vec![
            state.player_party[0].clone(),
            state.player_party[0].clone(),
            state.player_party[0].clone(),
        ]
        .as_mut(),
    );

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
                KeyCode::Char('e') => {
                    for player in state.player_party.iter_mut() {
                        if player.act_available.contains(&ActionOptions::Magic) {
                            player.add_action(&ActionOptions::Manif);
                        } else {
                            player.add_action(&ActionOptions::Magic);
                        }
                    }
                }
                KeyCode::Up | KeyCode::Char('w') => {
                    up_interaction(&mut state);
                }
                KeyCode::Down | KeyCode::Char('s') => {
                    down_interaction(&mut state);
                }
                KeyCode::Left | KeyCode::Char('a') => {
                }
                KeyCode::Right | KeyCode::Char('d') => {
                }
                KeyCode::Enter | KeyCode::Char(' ') => {
                    let ui_state = &mut state.ui;
                    ui_state.select();
                }
                _ => {}
            },
            Event::Tick => {}
        }

        delta = time.elapsed().as_secs_f32();
    }
    Ok(())
}

fn up_interaction(state: &mut State) {
    let ui_state = &mut state.ui;
    ui_state.prev();
}

fn down_interaction(state: &mut State) {
    let ui_state = &mut state.ui;
    ui_state.next(
        state.player_party.len(), 
        state.player_party[ui_state.from.0].act_available.len(), 
        state.enemy_party.len(), 
    );
}
