extern crate glutin_window;

use super::piston::window::WindowSettings;
use super::piston::event_loop::*;
use super::piston::input::*;
use super::opengl_graphics::{ GlGraphics, OpenGL};

use self::glutin_window::GlutinWindow as Window;

use super::interface::Interface;

/* structs */

pub struct Display<'a> {
    gl            : GlGraphics,
    window        : Window,
    events        : Events,
    pub interface : Interface<'a>,
}

use std::time::{SystemTime};

/* implementation */

impl <'a>Display<'a> {
    pub fn new(title : & str, width : u32, height : u32, interface : Interface<'a>) -> Display<'a> {

        let opengl = OpenGL::V3_2;

        let window: Window = WindowSettings::new(title, [width, height])
            .opengl(opengl)
            .vsync(true)
            .exit_on_esc(true)
            .build()
            .expect("Could not create window");

        let events : Events = Events::new(EventSettings::new());

        let gl = GlGraphics::new(opengl);

        Display{gl, window, events, interface}
    }

    pub fn main_loop(& mut self) {

        while let Some(e) = self.events.next(&mut self.window) {
            self.parse_event(e);
        }
    }

    pub fn loop_until<F, T>(& mut self, condition: F, param : & mut T) where F : Fn(&T, &mut Interface) -> bool {
        while let Some(e) = self.events.next(&mut self.window) {
            if !self.interface.paused && condition(&param, & mut self.interface) {
                break ;
            }
            self.parse_event(e);
        }
    }

    fn parse_event(&mut self, e : Event) {
        if let Some(r) = e.render_args() {
            self.render(&r);
        }

        if let Some(u) = e.update_args() {
            self.update(&u);
        }

        if let Some(p) = e.press_args() {
            self.input(&p);
        }
    }

    pub fn sleep(& mut self, n : u32) {
        self.loop_until(super::conditions::n_nano_seconds_from, & mut (n, &SystemTime::now()));
    }

    pub fn render(& mut self, args: &RenderArgs) {
        let interface = & mut self.interface;

        self.gl.draw(args.viewport(), |c, gl| {

            interface.render(args, &c, gl);
        });
    }

    fn update(&mut self, args: &UpdateArgs) {
        self.interface.update(args);
    }

    fn input(&mut self, button: &Button) {
        self.interface.input(button);
    }
}
