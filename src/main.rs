pub mod playground;
pub mod gui;
pub mod controller;
pub mod descriptions;

use crate::playground::*;
use crate::gui::*;
use crate::controller::*;

fn main() -> Result<(), std::io::Error> {
    let mut pg = Playground::new(Point{x: 40, y: 20});
    let mut gui = GUI::new(&pg);

    gui.show(&pg)?;
    loop {
        let maybe_kitten = loop {
            let event = Controller::wait_event();
            let maybe_found_object = match event {
                GameEvent::Move(dir) => pg.move_player(dir),
                GameEvent::Quit =>  break None,
            };
            gui.show(&pg)?;
            let found_object = if let Some(obj) = maybe_found_object {
                obj
            }
            else {
                continue;
            };

            if found_object.is_kitten {
                break Some(found_object);
            }
        };

        if let Some(kitten) = maybe_kitten {
            gui.show_meeting_animation(&kitten)?;
        }
        break;
    }

    Ok(())
}
