use std::io::{stdout, Write, Stdout, Error};
use crossterm::{
    ExecutableCommand, QueueableCommand,
    terminal, cursor, style::{self, Stylize, Color}, Result
};
use std::time;
use crossterm::terminal::disable_raw_mode;
use crossterm::terminal::enable_raw_mode;

use crate::playground::*;

#[derive(Clone)]
struct DisplayPoint {
    x: u16,
    y: u16,
}

impl DisplayPoint {
    fn from(pt: &Point) -> Option<DisplayPoint> {
        if pt.x < 0 || pt.y < 0 {
            None
        }
        else {
            Some(DisplayPoint { x: pt.x as u16, y: pt.y as u16})
        }
    }
}

pub struct GUI {
    stdout: Stdout,
    header_height: u16,
    top_string: String,
    prev_player_pos: DisplayPoint,
    border: DisplayPoint,
}

impl GUI {
    pub fn new(pg: &Playground) -> GUI {
        let border_opt = pg.square.clone() + Point { x: 2, y: 2 };
        let border_opt = DisplayPoint::from(&border_opt);
        let border = match border_opt {
            Some(pt) => pt,
            None => { panic!("Negative border"); }
        };

        enable_raw_mode().expect("Unable to put terminal in the raw mode");
        GUI {
            stdout: stdout(),
            header_height: 3,
            top_string: String::from("Use 'hjkl' to move; 'q' to quit"),
            border,
            prev_player_pos: DisplayPoint { x: 1, y: 1 },
        }
    }

    fn draw_object_symbol(&mut self, object: &Object) -> Result<()> {
        self.stdout
            .queue(style::PrintStyledContent(
                object.symbol
                    .with(Color::parse_ansi(&object.fg_ansi_color_str()).unwrap())))?;
        Ok(())
    }

    fn draw_object_in_its_place(&mut self, object: &Object) -> Result<()> {
        let dp = if let Some(pt) = DisplayPoint::from(&object.coordinate) {
            pt
        }
        else {
            return Err(Error::new(std::io::ErrorKind::Other, "Negative point to display"));
        };

        self.stdout
            .queue(cursor::MoveTo(
                dp.x + 1,
                dp.y + 1 + self.header_height))?;
        self.draw_object_symbol(object)?;

        return Ok(());
    }

    fn print_status_string(&mut self, pt: &DisplayPoint, s: &str) -> Result<()> {
        self.stdout.queue(cursor::MoveTo(0, pt.y))?;
        self.stdout.execute(terminal::Clear(terminal::ClearType::CurrentLine))?;
        self.stdout.queue(style::PrintStyledContent("*".green()))?;
        self.stdout.queue(cursor::MoveTo(2, pt.y))?;
        self.stdout.write(s.as_bytes())?;

        if s.len() < self.border.x as usize {
            self.stdout.queue(cursor::MoveTo(self.border.x, pt.y))?;
            self.stdout.queue(style::PrintStyledContent("*".green()))?;
        }
        Ok(())
    }

    fn print_top_string<'a>(&'a mut self, s: &'a String) -> Result<()> {
        self.print_status_string(&DisplayPoint { x: 2, y: 1 }, s)
    }

    fn print_bottom_string<'a>(&'a mut self, s: &'a String) -> Result<()> {
        self.print_status_string(&DisplayPoint { x: 2, y: 2 }, s)
    }

    fn compose_meeting_paddings(i: u16, out_of: u16) -> (String, String) {
        let mut outer_string = String::from("");
        for _ in 0..i {
            outer_string.push(' ');
        }
        let mut inner_string = String::from("");
        for _ in 0..(out_of - i) {
            inner_string.push(' ');
        }
        (outer_string, inner_string)
    }

    pub fn show_meeting_animation(&mut self, obj: &Object) -> Result<()> {
        let iterations = 5;
        for i in 0..iterations {
            std::thread::sleep(time::Duration::from_secs(1));
            let (outer_string, inner_string) = Self::compose_meeting_paddings(i + 1, iterations);
            self.stdout.queue(cursor::MoveTo(2, 2))?;
            self.stdout.write(outer_string.as_bytes())?;
            self.stdout.write(String::from("#").as_bytes())?;
            self.stdout.write(inner_string.as_bytes())?;
            self.stdout.write(inner_string.as_bytes())?;
            self.draw_object_symbol(obj)?;
            self.stdout.write(outer_string.as_bytes())?;
            self.finalize_cursor_position()?;
            self.stdout.flush()?;
        }
        Ok(())
    }

    fn draw_border(&mut self) -> Result<()> {
        for y in 0..=(self.border.y + self.header_height) {
            for x in 0..=self.border.x {
                if (y == 0 ||
                    y == self.header_height ||
                    y == self.header_height + self.border.y) ||
                   (x == 0 || x == self.border.x) {
                    self.stdout
                        .queue(cursor::MoveTo(x, y))?
                        .queue(style::PrintStyledContent("*".green()))?;
                }
            }
        }
        Ok(())
    }

    pub fn remember_player_position(&mut self, pos: &Point) {
        let player_pos = pos.clone() + Point { x: 1, y: self.header_height as i32 + 1 };
        self.prev_player_pos = match DisplayPoint::from(&player_pos) {
            Some(pt) => pt,
            None => { panic!("Negative display point"); }
        };
    }

    fn clear_cell(&mut self, pt: &DisplayPoint) -> Result<()> {
        self.stdout
            .queue(cursor::MoveTo(pt.x, pt.y))?;
        self.stdout.write(" ".as_bytes())?;
        Ok(())
    }

    pub fn draw_updates(&mut self, playground: & Playground) -> Result<()> {
        self.clear_cell(&self.prev_player_pos.clone())?;
        self.draw_object_in_its_place(&playground.player.object)?;

        self.print_top_string(&self.top_string.clone())?;
        self.print_bottom_string(&playground.status)?;
        self.finalize_cursor_position()?;

        self.stdout.flush()?;
        Ok(())
    }

    // (0, 0) is top-left
    pub fn show<'a>(&mut self, playground: &'a Playground) -> Result<()> {
        self.stdout.execute(terminal::Clear(terminal::ClearType::All))?;

        self.draw_border()?;

        for obj in playground.objects.iter() {
            self.draw_object_in_its_place(&obj)?;
        }
        self.draw_object_in_its_place(&playground.player.object)?;

        self.print_top_string(&self.top_string.clone())?;
        self.print_bottom_string(&playground.status)?;
        self.finalize_cursor_position()?;

        self.stdout.flush()?;
        Ok(())
    }

    fn finalize_cursor_position(&mut self) -> Result<()> {
        self.stdout.queue(cursor::MoveTo(0, self.header_height + self.border.y + 1))?;
        Ok(())
    }
}


impl Drop for GUI {
    fn drop(&mut self) {
        disable_raw_mode().expect("Unable to get terminal out of the raw mode");
    }
}

