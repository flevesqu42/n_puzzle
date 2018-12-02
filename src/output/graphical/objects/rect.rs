use super::*;
use super::super::graphics::Transformed;

pub struct Rect {
    pub position  : Position, // position on screen, rated into 0.0 and 1.0
    pub rectangle : types::Rectangle,
    pub color     : [f32; 4],
}

impl Object for Rect {

    fn render(&mut self, args : &RenderArgs, c : & Context, gl : &mut GlGraphics, _ : ObjectArguments) {

        let (x, y) = (args.width as f64 * self.position.x, args.height as f64 * self.position.y);

        let transform = c.transform.trans(x, y)
            .trans(-self.rectangle[2] / 2.0, -self.rectangle[3] / 2.0);

        rectangle(self.color, self.rectangle, transform, gl);
    }

    fn update(& mut self, _args : &UpdateArgs) {
    }

    fn get_type(&self) -> ObjectType {
        ObjectType::Graphic
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

impl Rect {
    pub fn new(position: Position, rectangle: types::Rectangle, color : [f32; 4]) -> Box<Rect> {
        Box::new(Rect {position, rectangle, color})
    }
}
