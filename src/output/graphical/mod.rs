extern crate piston;
extern crate graphics;
extern crate opengl_graphics;

mod objects;
mod interface;
mod display;
mod conditions;

use self::objects::*;
use self::objects::text::*;
use self::objects::rect::*;
use self::objects::cell_object::*;
use self::interface::Interface;
use self::display::Display;
use self::conditions::*;

use models::{Map, Move};
use models;

use heuristics::manhattan_distance;

const TITLE : & str = "N-puzzulle !";
const WIDTH : u32 = 1000;
const HEIGHT : u32 = 1000;
const BACKGROUND_COLOR : [f32; 4] = [0.3, 0.3, 0.3, 1.0];

const FONTS_PATH : [& str; 1] = [
    "resources/ttf/Montserrat-Bold.ttf"
];
const FONT_COLOR : [f32; 4] = [1.0, 1.0, 1.0, 1.0];
const FONT_SIZE : u32 = 40;

const PIXEL_BETWEEN : f64 = 2.0;

const SOLVED : [f32; 4] = [0.16, 0.80, 0.38, 1.0];
const FAR : [f32; 4] = [0.95, 0.25, 0.25, 1.0];

macro_rules! width_cell {
    ($x:expr) => (WIDTH as f64 / $x - (PIXEL_BETWEEN * 2.0))
}

macro_rules! height_cell {
    ($x:expr) => (WIDTH as f64 / $x - (PIXEL_BETWEEN * 2.0))
}

pub fn init_display<'a>() -> Display<'a> {

    let mut interface = Interface::new(BACKGROUND_COLOR);

    interface.push_font(FONTS_PATH[0]);

    Display::new(TITLE, WIDTH, HEIGHT, interface)
}

pub fn display_map(display : & mut Display, map : &Map, n : u32) {
    let new_objects = generate_graphical_objects_from(map);

    display.interface.clear_objects(new_objects);
    display.sleep(n);
}

pub fn display_path(display : & mut Display, original_map : &Map, path : &Vec<Move>) {

    let mut map = original_map.clone();

    display_map(display, &map, 500_000_000);

    let size = map.size as f64;

    for last_move in path {
        slide(display, & mut map, last_move, size);
    }
}

fn slide(display : & mut Display, map : & mut Map, last_move : &Move, size : f64) {

    let from = cell_position(&last_move.from, size);
    let to = cell_position(&last_move.to, size);

    let id = display.interface.get_external_id_nearest_from_pos(&from);

    display.loop_until(object_slide_to, & mut (id, &to));

    map.move_in_place(&last_move);

    let color = get_cell_color(last_move.to.x as i32, last_move.to.y as i32, &map.cells[last_move.to.x + last_move.to.y * map.size].number.target, map.size);

    display.interface.objects[id].set_color(color);
}

fn generate_graphical_objects_from(map : &Map) -> Vec<Box<Object>> {

    let mut objects : Vec<Box<Object>> = Vec::new();
    let size = map.size as f64;

    for (index, cell) in map.cells.iter().enumerate() {
        match cell.number.value {
            0 => {},
            _ => {
                let x = index % map.size;
                let y = index / map.size;

                let current = models::Position::new(x, y);

                let font_color = FONT_COLOR;
                let rect_color = get_cell_color(x as i32, y as i32, &cell.number.target, map.size);

                let position = cell_position(&current, size);
                let rect  = Rect::new(position.clone(), [0.0, 0.0, width_cell!(size), height_cell!(size)], rect_color);
                let text = Text::new(position, font_color, FONT_SIZE, cell.number.value.to_string(), TextOption::Centered, 0);

                let cell_object = CellObject::new(rect, text);

                objects.push(cell_object);
            }
        };
    }

    objects
}

fn get_cell_color(x : i32 , y : i32, target : &models::Position, map_size : usize) -> [f32; 4] {

    let dist = (manhattan_distance(target, x, y) as f32 / (map_size - 1) as f32).min(1.0);

    [(SOLVED[0] - (SOLVED[0] * dist)) + (FAR[0] * dist),
        (SOLVED[1] - (SOLVED[1] * dist)) + (FAR[1] * dist),
        (SOLVED[2] - (SOLVED[2] * dist)) + (FAR[2] * dist),
        (SOLVED[3] - (SOLVED[3] * dist)) + (FAR[3] * dist)]
}


fn cell_position(initial_position : &models::Position, map_size : f64) -> Position {
    let x = initial_position.x as f64 + 0.5;
    let y = initial_position.y as f64 + 0.5;
    let size = map_size;

    Position::new(x / size, y / size)
}