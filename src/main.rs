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

enum Event<I> {
    Input(I),
    Tick,
}

pub struct State {
    /// Max 4
    enemy_party: u8,
    /// Max 4
    player_party: u8,
}

#[derive(Debug, Default)]
struct Stats {
    attack: u16,
    defense: u16,
    hope: u16,
}

#[derive(Debug, Default)]
pub struct Character {
    name: String,
    stats: Stats,
    time: f32,
}
impl Character {
    fn update(&mut self, delta: f32) {
        let mut time = self.time + delta;
        if time >= 100.0 { time = 100.0; }
        self.time = time;
    }
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
        enemy_party: 4,
        player_party: 1,
    };
    let mut player = Character {
        name: "Jugador".to_string(),
        stats: Stats { attack: 5, defense: 4, hope: 3 },
        ..Default::default()
    };

    
    let mut delta = 0.0;
    let cerrar = false;
    //* Main loop
    while !cerrar {
        let time = Instant::now();

        player.update(delta);

        //* Render job
        ui_rendering::draw(&mut terminal, &player, &state)?;
        
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
                    if !(state.player_party >= 4) { state.player_party+=1; }
                    else { state.player_party = 1; }
                }
                KeyCode::Down => {
                    if !(state.player_party <= 1) { state.player_party-=1; }
                    else { state.player_party = 4; }
                }
                _ => {}
            },
            Event::Tick => {}
        }

        delta = time.elapsed().as_secs_f32();
    }
    Ok(())
}
