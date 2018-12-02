use super::opengl_graphics::GlGraphics;
use super::piston::input::{RenderArgs, UpdateArgs};

use super::graphics::{Context, rectangle, types};

use super::opengl_graphics::GlyphCache;

pub mod text;
pub mod rect;
pub mod cell_object;

/* TRAIT */

pub trait Object {
    fn render(&mut self, args : &RenderArgs, c : & Context, gl : &mut GlGraphics, object_arguments : ObjectArguments);
    fn update(&mut self, args : &UpdateArgs);

    fn get_type(&self) -> ObjectType;
    fn get_position(&self) -> &Position;

    fn set_position(&mut self, new : Position);

    fn translate(&mut self, x : f64, y : f64);

    fn set_color(&mut self, color : [f32; 4]);
}

/* ENUM */

pub enum ObjectType {
    Graphic,
    Text(usize), // <<< index for GlyphCache
    Composed(usize), // <<< same as previous
}

pub enum ObjectArguments<'a, 'b : 'a> {
    None,
    Text(&'a mut GlyphCache<'b>),
    Composed(&'a mut GlyphCache<'b>),
}

/* IMPL */

impl <'a, 'b : 'a>ObjectArguments<'a, 'b> {
    pub fn unwrap_composed(self) -> (&'a mut GlyphCache<'b>) {
        match self {
            ObjectArguments::Composed(gc) => (gc),
            _                             => panic!("Unwrap ObjectArgument::Composed failed")
        }
    }

    pub fn unwrap_text(self) -> &'a mut GlyphCache<'b> {
        match self {
            ObjectArguments::Composed(gc) => gc,
            ObjectArguments::Text(gc)     => gc,
            _                             => panic!("Unwrap ObjectArgument::Text failed")
        }
    }
}

/* dependencies */

#[derive(Clone, Debug)]
pub struct Position {
    pub x: f64,
    pub y: f64,
}

impl Position {
    pub fn new(x: f64, y: f64) -> Position {
        Position{x, y}
    }

    pub fn distance_from(&self, other : &Position) -> f64 {
        let x = self.x - other.x;
        let y = self.y - other.y;

        (x * x + y * y).sqrt()
    }
}