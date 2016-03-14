extern crate piston_window;

use piston_window::*;

pub struct App {
    pub rotation: f64,
    pub square: [f64; 4],
    pub color: [f64; 4]
}

impl App {
    pub fn update(&mut self, args: &UpdateArgs) {
        self.rotation += 45.0*args.dt;
    }

}
