use rand::Rng;
use std::ops;

use crate::controller::Direction;
use crate::descriptions::DESCRIPTIONS;

#[derive(Clone)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl ops::Add<Point> for Point {
    type Output = Point;

    fn add(self, rhs: Point) -> Point {
        Point{x: self.x + rhs.x, y: self.y + rhs.y}
    }
}

#[derive(Clone)]
pub struct Object {
    pub symbol: char,
    pub description: String,
    pub coordinate: Point,
    pub fg_color: u8,
    pub is_kitten: bool
}

impl Object {
    fn new() -> Object {
        Object {
            symbol: '&',
            description: String::from(""),
            coordinate: Point{x: 10, y: 10},
            fg_color: 15,
            is_kitten: false,
        }
    }

    fn ansi_color_str(color: u8) -> String {
        format!("5;{}", color)
    }

    pub fn fg_ansi_color_str(&self) -> String {
        Self::ansi_color_str(self.fg_color)
    }
}

pub struct Player {
    pub object: Object,
}

pub struct Playground {
    pub square: Point,
    pub objects: Vec<Object>,
    pub player: Player,
    pub status: String,
}

impl Playground {
    pub fn new(square: Point) -> Playground {
        let mut player_object = Object::new();
        player_object.coordinate = Point {
            x: square.x / 2,
            y: square.y / 2,
        };
        player_object.symbol = '#';
        player_object.fg_color = 15;

        let mut pg = Playground {
            square,
            objects: Vec::new(),
            player: Player {
                object: player_object,
            },
            status: String::from(""),
        };

        let obj_count = (pg.square.x * pg.square.y) / 50;
        for _ in 0..obj_count {
            pg.objects.push(pg.generate_random_object());
        }

        let mut kitten = pg.generate_random_object();
        kitten.is_kitten = true;
        kitten.description = String::from("Kitten!");
        pg.objects.push(kitten);

        pg
    }

    fn generate_random_object(&self) -> Object {
        let mut result = Object::new();
        result.coordinate.x = rand::thread_rng().gen_range(1..self.square.x);
        result.coordinate.y = rand::thread_rng().gen_range(1..self.square.y);
        result.symbol = rand::thread_rng().gen_range('!'..='~'); // '#' will be used also
        result.fg_color = rand::thread_rng().gen_range(1..16);
        result.description = String::from(DESCRIPTIONS[rand::thread_rng().gen_range(1..DESCRIPTIONS.len())]);
        result
    }

    fn is_within_border(&self, pt: &Point) -> bool {
        pt.x >= 0 && pt.x <= self.square.x &&
            pt.y >=0 && pt.y <= self.square.y
    }

    fn check_object<'a, 'b>(&'a self, pt: &'b Point) -> Option<Object> {
        for obj in self.objects.iter() {
            if obj.coordinate == *pt {
                let new_obj = obj.clone();
                return Some(new_obj);
            }
        }
        None
    }

    pub fn move_player<'a>(&'a mut self, dir: Direction) -> Option<Object> {
        let move_vector = match dir {
            Direction::Left => Point{x: -1, y: 0},
            Direction::Up => Point{x: 0, y: -1},
            Direction::Right => Point{x: 1, y: 0},
            Direction::Down => Point{x: 0, y: 1},
        };
        let target_point = self.player.object.coordinate.clone() + move_vector;

        if !self.is_within_border(&target_point) { return None; }

        let obj_ref_opt = self.check_object(&target_point);
        if let Some(obj) = obj_ref_opt.as_ref() {
            self.status = obj.description.clone();
        }
        else {
            self.player.object.coordinate = target_point;
        }
        obj_ref_opt
    }
}
