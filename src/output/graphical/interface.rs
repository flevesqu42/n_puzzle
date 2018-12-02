use super::opengl_graphics::GlGraphics;
use super::graphics::{Context, clear};
use super::piston::input::{RenderArgs, UpdateArgs, Button};
use super::opengl_graphics::{ GlyphCache, TextureSettings, Filter };
use super::piston::input::keyboard::Key;

use super::objects::{Object, ObjectType, ObjectArguments, Position};

/* structs */

pub struct Interface<'a> {

    pub background_color : [f32; 4],
    pub objects          : Vec<Box<Object>>,
    pub glyphs           : Vec<GlyphCache<'a>>,
    pub speed            : f64,
    pub paused           : bool,
}

/* implementation */

impl <'a>Interface<'a> {
    pub fn new(background_color : [f32; 4]) -> Interface<'a> {
        Interface{background_color, objects: vec![], glyphs: vec![], speed: 1.0, paused: false}
    }

    pub fn update(& mut self, args : &UpdateArgs) {
        for object in self.objects.iter_mut() {
            object.update(args);
        }

    }

    pub fn render(& mut self, args : &RenderArgs, c : & Context, gl : & mut GlGraphics) {
        clear(self.background_color, gl);

        for object in self.objects.iter_mut() {

            let object_arguments = match object.get_type() {
                ObjectType::Composed(glyph_index) => ObjectArguments::Composed(& mut self.glyphs[glyph_index]),
                ObjectType::Text(glyph_index)     => ObjectArguments::Text(& mut self.glyphs[glyph_index]),
                ObjectType::Graphic               => ObjectArguments::None
            };

            object.render(args, &c, gl, object_arguments);
        }
    }

    pub fn input(& mut self, button : &Button) {
            match button {
                &Button::Keyboard(key) if key == Key::Equals && self.speed < 20.0 => { self.speed *= 1.5},
                &Button::Keyboard(key) if key == Key::Minus && self.speed > 0.01  => { self.speed /= 1.5},
                &Button::Keyboard(key) if key == Key::Space                       => { self.paused ^= true },
                _ => {}
            }
    }

    pub fn clear_objects(&mut self, new_objects : Vec<Box<Object>>) {
        self.objects = new_objects;
    }

    /* return an external ID of font */

    pub fn push_font(& mut self, font_path : & str) -> usize {
        let glyph = GlyphCache::new(font_path, (), TextureSettings::new().filter(Filter::Nearest)).expect("Could not load font");

        self.glyphs.push(glyph);
        self.glyphs.len() - 1
    }
    /* return an external ID of object */

    pub fn get_external_id_nearest_from_pos(&self, position : &Position) -> usize {

        if self.objects.len() == 0 {
            panic!("No objects found");
        }

        let mut distance_min = self.objects[0].get_position().distance_from(position);
        let mut external_id= 0;

        for (index, object) in self.objects.iter().enumerate() {
            let current_distance = object.get_position().distance_from(position);

            if current_distance < distance_min {
                distance_min = current_distance;
                external_id = index;
            }
        }

        external_id
    }
}