use crossterm::event::{read, Event, KeyCode};

pub enum Direction {
    Left,
    Up,
    Right,
    Down,
}

pub enum GameEvent {
    Move(Direction),
    Quit,
}

pub struct Controller {}

impl Controller {
    pub fn wait_event() -> GameEvent {
        loop {
            let event = read().unwrap();

            let code = match event {
                Event::Key(key_event) => key_event.code,
                _ => continue,
            };

            let game_event = match code {
                KeyCode::Left => GameEvent::Move(Direction::Left),
                KeyCode::Up => GameEvent::Move(Direction::Up),
                KeyCode::Right => GameEvent::Move(Direction::Right),
                KeyCode::Down => GameEvent::Move(Direction::Down),
                KeyCode::Esc => GameEvent::Quit,
                KeyCode::Char(c) => match c {
                    'q' => GameEvent::Quit,
                    'h' => GameEvent::Move(Direction::Left),
                    'k' => GameEvent::Move(Direction::Up),
                    'l' => GameEvent::Move(Direction::Right),
                    'j' => GameEvent::Move(Direction::Down),
                    _ => {continue;}
                }
                _ => {continue;}
            };
            break game_event;
        }
    }
}
