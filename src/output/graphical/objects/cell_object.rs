use super::*;

use super::text::Text;

pub struct CellObject {
    pub object : Box<Object>,
    pub text   : Box<Text>,
}


impl Object for CellObject {
    fn render(&mut self, args: &RenderArgs, c: &Context, gl: &mut GlGraphics, object_arguments : ObjectArguments) {
        let gc = object_arguments.unwrap_composed();

        self.object.render(args, c, gl, ObjectArguments::Composed(gc));
        self.text.render(args, c, gl, ObjectArguments::Text(gc));
    }

    fn update(& mut self, args : &UpdateArgs) {
        self.object.update(args);
    }

    fn get_type(&self) -> ObjectType {
        ObjectType::Composed(self.text.index_glyph)
    }

    fn get_position(&self) -> &Position {
        &self.object.get_position()
    }

    fn set_position(&mut self, new : Position) {
        self.object.set_position(new.clone());
        self.text.set_position(new);
    }

    fn translate(& mut self, x : f64, y : f64) {
        self.object.translate(x, y);
        self.text.translate(x, y);
    }

    fn set_color(&mut self, color : [f32; 4]) {
        self.object.set_color(color);
    }

}


impl CellObject {
    pub fn new(object : Box<Object>, text : Box<Text>) -> Box<CellObject> {
        Box::new(CellObject{object, text})
    }
}
