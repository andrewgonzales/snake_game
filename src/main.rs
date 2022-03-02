extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventLoop, EventSettings, Events};
use piston::input::{
    Button, ButtonArgs, ButtonEvent, ButtonState, Key, RenderArgs, RenderEvent, UpdateArgs,
    UpdateEvent,
};
use piston::window::WindowSettings;

struct Game {
    gl: GlGraphics,
    snake: Snake,
}

impl Game {
    fn render(&mut self, args: &RenderArgs) {
        // use graphics;

        let screen_color: [f32; 4] = [0.26, 0.89, 0.96, 0.9];

        self.gl.draw(args.viewport(), |_c, gl| {
            // Clear the screen.
            graphics::clear(screen_color, gl);
        });

        self.snake.render(&mut self.gl, &args);
    }

    fn update(&mut self) {
        self.snake.update();
    }

    fn on_press(&mut self, args: &ButtonArgs) {
        self.snake.on_press(&args);
    }
}

#[derive(PartialEq)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

struct Snake {
    pos_x: i32,
    pos_y: i32,
    dir: Direction,
}

impl Snake {
    fn render(&mut self, gl: &mut GlGraphics, args: &RenderArgs) {
        let snake_color: [f32; 4] = [0.0, 1.0, 0.0, 0.9];
        let cell_width = 20;
        let x = self.pos_x * cell_width;
        let y = self.pos_y * cell_width;

        let square = graphics::rectangle::square(x as f64, y as f64, 20_f64);
        gl.draw(args.viewport(), |c, gl| {
            let transform = c.transform;
            // Draw the snake.
            graphics::rectangle(snake_color, square, transform, gl);
        });
    }

    fn update(&mut self) {
        match self.dir {
            Direction::Left => {
                if self.pos_x == 0 {
                    self.pos_x = 9;
                } else {
                    self.pos_x -= 1
                };
            }
            Direction::Right => {
                if self.pos_x == 9 {
                    self.pos_x = 0;
                } else {
                    self.pos_x += 1;
                }
            }
            Direction::Up => {
                if self.pos_y == 0 {
                    self.pos_y = 9;
                } else {
                    self.pos_y -= 1;
                }
            }
            Direction::Down => {
                if self.pos_y == 9 {
                    self.pos_y = 0;
                } else {
                    self.pos_y += 1;
                }
            }
        };
    }

    fn on_press(&mut self, args: &ButtonArgs) {
        let current_direction: &Direction = &self.dir;
        match args.button {
            Button::Keyboard(Key::Left) if current_direction != &Direction::Right => {
                self.dir = Direction::Left
            }
            Button::Keyboard(Key::Right) if current_direction != &Direction::Left => {
                self.dir = Direction::Right
            }
            Button::Keyboard(Key::Up) if current_direction != &Direction::Down => {
                self.dir = Direction::Up
            }
            Button::Keyboard(Key::Down) if current_direction != &Direction::Up => {
                self.dir = Direction::Down
            }
            _ => (),
        }
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
        snake: Snake {
            pos_x: 2,
            pos_y: 5,
            dir: Direction::Right,
        },
    };

    let mut events = Events::new(EventSettings::new()).ups(8);
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            game.render(&args);
        }

        if let Some(_args) = e.update_args() {
            game.update();
        }

        if let Some(args) = e.button_args() {
            if args.state == ButtonState::Press {
                game.on_press(&args);
            }
        }
    }
}
