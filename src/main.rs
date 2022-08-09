mod characters;
mod ui_rendering;

// Importing
use characters::*;
use crossterm::{
    event::{self, Event as CEvent, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{
    error::Error, 
    io,
    sync::mpsc,
    thread,
    time::{Duration, Instant},
    option::Option::*, 
};
use tui::{backend::{Backend, CrosstermBackend}, Terminal};
use ui_rendering::{ActionOptions, UiState, StatefulList};

pub struct Game<'a> {
    pub app_state: AppState<'a>, 
    pub battle_state: BattleState, 
    pub ui_state: UiState, 
}
impl<'a> Game<'a> {
    pub fn new(title: &'a str) -> Game<'a> {
        let enemy_party = vec![
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
        ];
        let player_party = vec![
            Character {
                name: "Personaje1".to_string(),
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
            }, 
            Character {
                name: "Personaje2".to_string(),
                stats: Stats {
                    attack: 3,
                    defense: 5,
                    hope: 4,
                },
                health: 83,
                max_health: 100,
                mana: 56,
                max_mana: 100,
                ..Default::default()
            }, 
            Character {
                name: "Personaje3".to_string(),
                stats: Stats {
                    attack: 3,
                    defense: 4,
                    hope: 5,
                },
                health: 27,
                max_health: 100,
                mana: 38,
                max_mana: 100,
                ..Default::default()
            }, 
        ];

        Game { 
            app_state: AppState { 
                tittle: title, 
                should_quit: false, 
            }, 
            battle_state: BattleState { 
                enemy_party: enemy_party.clone(), 
                player_party: player_party.clone(), 
            }, 
            ui_state: UiState { 
                enemy_party: None, 
                player_party: None, 
                from: StatefulList::with_items(vec!["".to_string()], "Quien?"), 
                what: StatefulList::with_items(vec!["".to_string()], "Qué?"), 
                which: StatefulList::with_items(vec!["".to_string()], "Cual?"), 
                to: StatefulList::with_items(vec!["".to_string()], "A quien?")
            }, 
        }
    }
}

pub struct AppState<'a> {
    pub tittle: &'a str, 
    pub should_quit: bool, 
}

pub struct BattleState {
    /// Max 4
    enemy_party: Vec<Character>,
    /// Max 4
    player_party: Vec<Character>,
}

enum Event<I> {
    Input(I),
    Tick,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let game = Game::new("Asies");
    let res = run_app(&mut terminal, game);

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())

    // enable_raw_mode().expect("can run in raw mode");

    // //* Event loop
    // let (tx, rx) = mpsc::channel();
    // let tick_rate = Duration::from_millis(200);
    // thread::spawn(move || {
    //     let mut last_tick = Instant::now();
    //     loop {
    //         let timeout = tick_rate
    //             .checked_sub(last_tick.elapsed())
    //             .unwrap_or_else(|| Duration::from_secs(0));

    //         if event::poll(timeout).expect("polling failed") {
    //             if let CEvent::Key(key) = event::read().expect("couldn\'t read events") {
    //                 tx.send(Event::Input(key)).expect("couldn\'t send events");
    //             }
    //         }

    //         if last_tick.elapsed() >= tick_rate {
    //             if let Ok(_) = tx.send(Event::Tick) {
    //                 last_tick = Instant::now();
    //             }
    //         }
    //     }
    // });

    // //* Setup terminal output
    // let mut stdout = io::stdout();
    // execute!(stdout, EnterAlternateScreen)?;
    // let backend = CrosstermBackend::new(stdout);
    // let mut terminal = Terminal::new(backend)?;
    // terminal.clear()?;

    // // Setup game
    // let mut state = State {
    //     enemy_party: vec![
    //         Character {
    //             name: "Enemigo".to_string(),
    //             stats: Stats {
    //                 attack: 5,
    //                 defense: 5,
    //                 hope: 2,
    //             },
    //             health: 23,
    //             max_health: 100,
    //             mana: 82,
    //             max_mana: 100,
    //             ..Default::default()
    //         },
    //         Character {
    //             name: "Enemigo2".to_string(),
    //             stats: Stats {
    //                 attack: 5,
    //                 defense: 5,
    //                 hope: 2,
    //             },
    //             health: 23,
    //             max_health: 100,
    //             mana: 82,
    //             max_mana: 100,
    //             ..Default::default()
    //         },
    //         Character {
    //             name: "Enemigo3".to_string(),
    //             stats: Stats {
    //                 attack: 5,
    //                 defense: 5,
    //                 hope: 2,
    //             },
    //             health: 23,
    //             max_health: 100,
    //             mana: 82,
    //             max_mana: 100,
    //             ..Default::default()
    //         },
    //     ],
    //     player_party: vec![Character {
    //         name: "Personaje".to_string(),
    //         stats: Stats {
    //             attack: 5,
    //             defense: 4,
    //             hope: 3,
    //         },
    //         health: 78,
    //         max_health: 100,
    //         mana: 45,
    //         max_mana: 100,
    //         ..Default::default()
    //     }],
    // };

    // //? For demostration proposes
    // state.player_party.append(
    //     vec![
    //         state.player_party[0].clone(),
    //         state.player_party[0].clone(),
    //         state.player_party[0].clone(),
    //     ]
    //     .as_mut(),
    // );

    // let mut delta = 0.0;
    // let cerrar = false;
    // //* Main loop
    // while !cerrar {
    //     let time = Instant::now();

    //     update_chars_time(&mut state., delta);

    //     //* Render job
    //     ui_rendering::draw(&mut terminal, &state)?;

    //     //* Event handler
    //     match rx.recv()? {
    //         Event::Input(event) => match event.code {
    //             //* if Input
    //             KeyCode::Char('q') => {
    //                 disable_raw_mode()?;
    //                 terminal.show_cursor()?;
    //                 execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    //                 break;
    //             }
    //             KeyCode::Char('e') => {
    //                 for player in state.player_party.iter_mut() {
    //                     if player.act_available.contains(&ActionOptions::Magic) {
    //                         player.add_action(&ActionOptions::Manif);
    //                     } else {
    //                         player.add_action(&ActionOptions::Magic);
    //                     }
    //                 }
    //             }
    //             KeyCode::Up | KeyCode::Char('w') => {
    //                 up_interaction(&mut state);
    //             }
    //             KeyCode::Down | KeyCode::Char('s') => {
    //                 down_interaction(&mut state);
    //             }
    //             KeyCode::Left | KeyCode::Char('a') => {
    //             }
    //             KeyCode::Right | KeyCode::Char('d') => {
    //             }
    //             KeyCode::Enter | KeyCode::Char(' ') => {
    //                 let ui_state = &mut state.ui;
    //                 ui_state.select();
    //             }
    //             _ => {}
    //         },
    //         Event::Tick => {}
    //     }

    //     delta = time.elapsed().as_secs_f32();
    // }
}

pub fn run() -> Result<(), Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let game = Game::new("Asies");
    let res = run_app(&mut terminal, game);

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}

fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    mut game: Game,
) -> Result<(), Box<dyn Error>> {
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

    let mut delta = 0.0;
    //* Main loop
    while !game.app_state.should_quit {
        let time = Instant::now();

        update_chars_time(&mut game.battle_state, delta);
        game.ui_state.populate(&game.battle_state);

        //* Render job
        ui_rendering::draw(terminal, &mut game.ui_state)?;

        //* Event handler
        match rx.recv()? {
            Event::Input(event) => match event.code {
                //* if Input
                KeyCode::Char('q') => {
                    game.app_state.should_quit = true;
                    continue;
                }
                KeyCode::Char('e') => {
                    let mut done = false;
                    for player in game.battle_state.player_party.iter_mut() {
                        if done { continue; }
                        if player.act_available.contains(&ActionOptions::Magic) {
                            player.add_action(&ActionOptions::Manif);
                        } else {
                            player.add_action(&ActionOptions::Magic);
                        }
                        done = true;
                    }
                }
                KeyCode::Up | KeyCode::Char('w') => {
                    game.ui_state.prev();
                }
                KeyCode::Down | KeyCode::Char('s') => {
                    game.ui_state.next();
                }
                KeyCode::Left | KeyCode::Char('a') => {
                    game.ui_state.unselect();
                }
                KeyCode::Right | KeyCode::Char('d') |
                    KeyCode::Enter | KeyCode::Char(' ') => {
                    game.ui_state.select();
                }
                _ => {}
            },
            Event::Tick => {}
        }

        delta = time.elapsed().as_secs_f32();
    }
    Ok(())
}
