extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;

struct Game {
	gl: GlGraphics,
}

impl Game {
	fn render(&mut self, args: &RenderArgs) {
		// use graphics;

		let screen_color: [f32; 4] = [0.26, 0.89, 0.96, 0.9];
        // let RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];

		self.gl.draw(args.viewport(), |_c, gl| {
            // Clear the screen.
            graphics::clear(screen_color, gl);
        });
	}
}

fn main() {
    let opengl = OpenGL::V3_2;

    // Create an Glutin window.
    let mut window: Window = WindowSettings::new("snake-game", [200, 200])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

	let mut game = Game {
		gl: GlGraphics::new(opengl),
	};

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
			game.render(&args);
		}

        // if let Some(args) = e.update_args() {
        // }
    }
}