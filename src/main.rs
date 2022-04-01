use cgmath::Vector2;
use crossterm::cursor::{Hide, Show};
use crossterm::event::{Event, KeyCode};
use crossterm::{event, terminal, ExecutableCommand};
use crossterm::terminal::{EnterAlternateScreen, LeaveAlternateScreen};
use rusty_audio::Audio;
use std::error::Error;
use std::time::Duration;
use std::io;

type EntityIndex = usize;
struct Physics {
    position: Vector2<u8>,
    velocity: Vector2<f32>,
}

struct Entity {
    physics: Option<Physics>,
    health: Option<f32>,
}

struct GameState {
    entities: Vec<Option<Entity>>,
    player: EntityIndex,
}

impl Physics {
    pub fn new() -> Self {
        Self {
            position: Vector2::new(0, 0),
            velocity: Vector2::new(0.0, 0.0),
        }
    }
}

impl Entity {
    pub fn new() -> Self {
        Self {
            physics: Some(Physics::new()),
            health: Some(1.0),
        }
    }
}

impl GameState {
    pub fn new() -> Self {
        let player = Entity::new();
        let invader = Entity::new();

        Self {
            entities: vec![Some(player), Some(invader)],
            player: 0,
        }
    }
}
fn main() -> Result<(), Box<dyn Error>> {
    let mut audio = Audio::new();
    audio.add("explode", "audio/explode.wav");
    audio.add("lose", "audio/lose.wav");
    audio.add("move", "audio/move.wav");
    audio.add("pew", "audio/pew.wav");
    audio.add("startup", "audio/startup.wav");
    audio.add("win", "audio/win.wav");
    audio.play("startup");

    // Terminal
    let mut stdout = io::stdout();
    terminal::enable_raw_mode()?;
    stdout.execute(EnterAlternateScreen)?;
    stdout.execute(Hide)?;

    let mut game_state = GameState::new();

    'gameloop: loop {

        // Capture input state
        while event::poll(Duration::default())? {
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    //KeyCode::Left => player.move_left(),
                    //KeyCode::Right => player.move_right(),
                    /*KeyCode::Char(' ') | KeyCode::Enter => {
                        if player.shoot() {
                            audio.play("pew")
                        }
                    }*/
                    KeyCode::Esc | KeyCode::Char('q') => {
                        audio.play("lose");
                        break 'gameloop;
                    }
                    _ => {}
                }
            }
        }

    }

    audio.wait();
    stdout.execute(Show)?;
    stdout.execute(LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;

    Ok(())
}

