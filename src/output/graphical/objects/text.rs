use super::*;

use super::super::graphics::text;
use super::super::graphics::Transformed;

pub enum TextOption {
    Centered
}

pub struct Text {
    pub position    : Position,
    pub color       : [f32; 4],
    pub size        : u32,
    pub text        : String,
    pub opt         : TextOption,
    pub index_glyph : usize,
}

impl Text {
    pub fn new(position : Position, color : [f32; 4], size : u32, text : String, opt : TextOption, index_glyph : usize) -> Box<Text>
    {
        Box::new(Text {position, color, size, text, opt, index_glyph})
    }
}

impl Object for Text {
    fn render(& mut self, args : &RenderArgs, c : & Context, gl : & mut GlGraphics, object_arguments : ObjectArguments) {

        let glyph = object_arguments.unwrap_text();

        let (x, y) = (args.width as f64 * self.position.x, args.height as f64 * self.position.y);

        let transform = {
            let off_x = -(self.size as f64 * (self.text.len()) as f64 / 4.0);
            let off_y = self.size as f64 / 4.0;

            c.transform.trans(x, y).trans(off_x, off_y)
        };

        match text(self.color, self.size, &self.text, glyph, transform, gl) {
            _ => {},
        }
    }

    fn update(& mut self, _args : &UpdateArgs) {}

    fn get_type(&self) -> ObjectType {
        ObjectType::Text(self.index_glyph)
    }

    fn get_position(&self) -> &Position {
        &self.position
    }

    fn set_position(&mut self, new : Position) {
        self.position = new;
    }

    fn translate(& mut self, x : f64, y : f64) {
        self.position.x += x;
        self.position.y += y;
    }

    fn set_color(&mut self, color : [f32; 4]) {
        self.color = color;
    }
}