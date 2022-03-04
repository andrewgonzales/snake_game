extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate rand;

use rand::{thread_rng, Rng};
use std::collections::LinkedList;

use glutin_window::GlutinWindow as Window;
use graphics::types::Rectangle;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventLoop, EventSettings, Events};
use piston::input::{
    Button, ButtonArgs, ButtonEvent, ButtonState, Key, RenderArgs, RenderEvent, UpdateEvent,
};
use piston::window::WindowSettings;

struct Game {
    gl: GlGraphics,
    snake: Snake,
    food: Food,
    score: i32,
}

impl Game {
    fn render(&mut self, args: &RenderArgs) {
        // Sky blue
        let screen_color: [f32; 4] = [0.26, 0.89, 0.96, 0.9];

        self.gl.draw(args.viewport(), |_c, gl| {
            // Clear the screen.
            graphics::clear(screen_color, gl);
        });

        self.snake.render(&mut self.gl, &args);

        self.food.render(&mut self.gl, &args);
    }

    fn update(&mut self) {
        self.snake.update();

        if is_collision(self.snake.body.front().unwrap(), &self.food.coord) {
            println!("Yum!");
            // update score
            self.score += 1;
            println!("Score: {}", self.score);
            // re-spawn food
            self.food.spawn();
            // grow snake
            self.snake.grow();
        }
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

#[derive(Clone)]
struct Point {
    x: i32,
    y: i32,
}

fn is_collision(p1: &Point, p2: &Point) -> bool {
    p1.x == p2.x && p1.y == p2.y
}

struct Snake {
    body: LinkedList<Point>,
    dir: Direction,
}

impl Snake {
    fn render(&mut self, gl: &mut GlGraphics, args: &RenderArgs) {
        let snake_color: [f32; 4] = [0.0, 1.0, 0.0, 0.9];
        let cell_width: f64 = 20.0;
        let squares: Vec<Rectangle> = self
            .body
            .iter()
            .map(|&Point { x, y }| -> Rectangle {
                graphics::rectangle::square(x as f64 * cell_width, y as f64 * cell_width, 20_f64)
            })
            .collect();

        gl.draw(args.viewport(), |c, gl| {
            let transform = c.transform;
            // Draw the snake.
            squares
                .into_iter()
                .for_each(|square| graphics::rectangle(snake_color, square, transform, gl));
        });
    }

    fn update(&mut self) {
        let mut new_head: Point = self.body.front().expect("Snake has no head!").clone();

        match self.dir {
            Direction::Left => match new_head {
                Point { x: 0, .. } => new_head.x = 9,
                _ => new_head.x -= 1,
            },
            Direction::Right => match new_head {
                Point { x: 9, .. } => new_head.x = 0,
                _ => new_head.x += 1,
            },
            Direction::Up => match new_head {
                Point { y: 0, .. } => new_head.y = 9,
                _ => new_head.y -= 1,
            },
            Direction::Down => match new_head {
                Point { y: 9, .. } => new_head.y = 0,
                _ => new_head.y += 1,
            },
        };

        self.body.pop_back();
        self.body.push_front(new_head);
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

    fn grow(&mut self) {
        let mut new_tail: Point = self.body.back().expect("Snake has no tail!").clone();
        let current_direction: &Direction = &self.dir;

        match current_direction {
            &Direction::Left => new_tail.x += 1,
            &Direction::Right => new_tail.x -= 1,
            &Direction::Up => new_tail.y += 1,
            &Direction::Down => new_tail.y -= 1,
        }

        self.body.push_back(new_tail);
    }
}

struct Food {
    coord: Point,
}

impl Food {
    fn new() -> Food {
        Food {
            coord: Point { x: 2, y: 5 },
        }
    }

    fn spawn(&mut self) {
        let mut rng = thread_rng();
        let x: i32 = rng.gen_range(0..9);
        let y: i32 = rng.gen_range(0..9);

        self.coord = Point { x, y };
    }

    fn render(&mut self, gl: &mut GlGraphics, args: &RenderArgs) {
        let food_color: [f32; 4] = [1.0, 0.0, 0.0, 0.9];
        let cell_width: f64 = 20.0;
        let x = self.coord.x;
        let y = self.coord.y;

        let square: Rectangle =
            graphics::rectangle::square(x as f64 * cell_width, y as f64 * cell_width, 20_f64);

        gl.draw(args.viewport(), |c, gl| {
            let transform = c.transform;
            // Draw the food.
            graphics::rectangle(food_color, square, transform, gl);
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
        snake: Snake {
            body: LinkedList::from([Point { x: 2, y: 5 }, Point { x: 1, y: 5 }]),
            dir: Direction::Right,
        },
        food: Food::new(),
        score: 0,
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
