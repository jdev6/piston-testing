extern crate piston_window;

mod app;
use app::App;

use piston_window::*;

fn main() {
    //create opengl context
    let opengl = OpenGL::V3_2;

    //Create glutin window
    let mut window: PistonWindow = WindowSettings::new(
        "rustgame",
        (800, 600)
        )
        .opengl(opengl)
        .exit_on_esc(false)
        .build()
        .unwrap();

    let mut app = App {
        rotation: 0.0,
        color: [1.0,0.0,0.0,1.0],
        square: rectangle::square(0.0, 0.0, 50.0)
    };
    
    for e in window { //events
        e.draw_2d(|c,g|{
            clear([0.0; 4], g);
            rectangle([1.0,1.0,1.0,1.0],
                      app.square, c.transform, g) 
        });


    }
}
