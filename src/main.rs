use std::{
    io, sync::mpsc, thread, 
    time::{Duration, Instant}
};
use crossterm::{
    event::{self, Event as CEvent, KeyCode},
    terminal::{
        disable_raw_mode, 
        enable_raw_mode, 
        LeaveAlternateScreen, 
        EnterAlternateScreen, 
    }, 
    execute, 
};
use tui::{
    backend::{CrosstermBackend}, 
    Terminal, 
};

mod ui_rendering;

mod character {
    #[derive(Debug, Default, Clone)]
    pub struct Stats {
        pub attack: u16,
        pub defense: u16,
        pub hope: u16,
    }

    #[derive(Debug, Default, Clone)]
    pub struct Character {
        pub name: String,
        pub stats: Stats,
        pub health: u16,
        pub max_health: u16,
        pub mana: u16,
        pub max_mana: u16,
        pub time: f32,
        pub time_mod: f32,
    }
    impl Character {
        pub fn update(&mut self, delta: f32) {
            let mut time = self.time + delta*self.time_mod;
            if time > 60.0 { time = 60.0; }
            self.time = time;
        }
    }
}
use character::*;

enum Event<I> {
    Input(I),
    Tick,
}

pub struct State {
    /// Max 4
    enemy_party: u8,
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
    let mut player = Character {
        name: "Jugador".to_string(),
        stats: Stats { attack: 5, defense: 4, hope: 3 },
        health: 78,
        max_health: 100,
        mana: 45,
        max_mana: 100,
        time_mod: 3.0,
        ..Default::default()
    };
    let mut state = State {
        enemy_party: 4,
        player_party: vec![player],
    };
    
    let mut delta = 0.0;
    let cerrar = false;
    //* Main loop
    while !cerrar {
        let time = Instant::now();

        for p in state.player_party.iter_mut() {
            p.update(delta);
        }

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
                KeyCode::Up => {
                    if !(state.player_party.len() >= 4) {
                        state.player_party.append(vec![
                            state.player_party[0].clone()
                        ].as_mut()
                        );
                    }
                    else {
                        state.player_party.pop();
                        state.player_party.pop();
                        state.player_party.pop();
                    }
                }
                KeyCode::Down => {
                    if !(state.player_party.len() <= 1) { state.player_party.pop(); }
                    else {
                        state.player_party.append(vec![
                            state.player_party[0].clone(),
                            state.player_party[0].clone(),
                            state.player_party[0].clone(),
                            ].as_mut()
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
