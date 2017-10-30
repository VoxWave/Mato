extern crate rand;
extern crate piston_window;

use std::collections::VecDeque;
use std::time::{Instant, Duration};

use rand::distributions::{Range, IndependentSample};

use piston_window::{PistonWindow, WindowSettings, ButtonArgs, Key, rectangle, clear};
use piston_window::Event::*;
use piston_window::Input::*;
use piston_window::Button::*;
use piston_window::ButtonState::*;

type Coord = (i32, i32);
 ///Peli on täällä
struct Game {
    snake_direction: Direction,
    snake_head: Coord,
    snake_body: VecDeque<Coord>,
    apple: Coord,
    running: bool,
    apple_eaten: bool,
    range: Range<i32>,
    ticks: u64,
    score: u64
}
///Kertoo mikä suunta on kyseessä.
#[derive(Clone, Copy)] 
enum Direction {
    Up, Down, Left, Right,
}

impl Game {
    fn new() -> Self {
        let mut rand = rand::thread_rng();
        let range = Range::new(0, 24);
        Game{
            snake_direction: Direction::Right,
            snake_head: (12,12),
            snake_body: VecDeque::new(),
            apple: (range.ind_sample(&mut rand), range.ind_sample(&mut rand)),
            apple_eaten: false,
            running: true,
            range: range,
            ticks: 0,
            score: 0,
        }
    }

    fn tick(&mut self) {
        use Direction::*;
        self.ticks += 1;
        let (dir_x, dir_y) = match self.snake_direction {
            Up => (0, 1),
            Down => (0, -1),
            Left => (-1, 0),
            Right => (1, 0),
        };

        self.snake_body.push_front(self.snake_head.clone());

        /* referenssi magiaa */
        let &mut (ref mut sh_x, ref mut sh_y) = &mut self.snake_head;
        *sh_x += dir_x;
        *sh_y += dir_y;

        // TODO: refaktoroi kovakoodatut reunaehdot pois.
        if *sh_x >= 25 || *sh_x < 0 || *sh_y >= 25 || *sh_y < 0 {
            self.running = false;
        }

        if !self.apple_eaten {
            self.snake_body.pop_back();
        } else {
            self.apple_eaten = false;
        }
        
        for p in &self.snake_body {
            if *sh_x == p.0 && *sh_y == p.1 {
                self.running = false;
                break;
            }
        }
        if self.running {
            if *sh_x == self.apple.0 && *sh_y == self.apple.1 {
                self.apple_eaten = true;
                self.score += 1;
                let mut rand = rand::thread_rng();
                self.apple = (self.range.ind_sample(&mut rand), self.range.ind_sample(&mut rand));
            }
        }
    }
}

fn main() {
    println!("Starting game!");
    let mut window: PistonWindow =
        WindowSettings::new("Mato!", (500, 500))
            .exit_on_esc(true)
            .build()
            .unwrap_or_else(|e| { panic!("Failed to build PistonWindow: {}", e) });
    
    let mut game = Game::new();
    let mut last_tick = Instant::now();
    let mut wanted_direction = game.snake_direction;
    while let Some(e) = window.next() {
        if let &Input(Button(ButtonArgs { button: Keyboard(ref k), state: Press, ..})) = &e {
            use Direction::*;
            wanted_direction = match *k {
                Key::W => Up,
                Key::A => Left,
                Key::S => Down,
                Key::D => Right,
                _ => wanted_direction,
            };
        }
        window.draw_2d(&e, |c, g| {
            clear([0.5, 0.5, 0.5, 1.0], g);
            let (head_x, head_y) = game.snake_head;
            rectangle([0.0, 0.5, 0.0, 1.0], // green
                      [head_x as f64 * 20., 500. - head_y as f64 * 20., 20.0, 20.0],
                      c.transform, g);
            for &(x, y) in &game.snake_body {
                rectangle([0.0, 0.4, 0.0, 1.0], // less green
                      [x as f64 * 20., 500. - y as f64 * 20., 20.0, 20.0],
                      c.transform, g);
            }
            let (apple_x, apple_y) = game.apple;
            rectangle([1.0, 0.0, 0.0, 1.0], // red
                      [apple_x as f64 * 20., 500. - apple_y as f64 * 20., 20.0, 20.0],
                      c.transform, g);
        });
        
        let speedup = (game.ticks as f32 + 1.).log2() * 100.;
        if last_tick.elapsed() > Duration::from_millis((1000 + game.score*10)-speedup as u64) {
            game.snake_direction = wanted_direction;
            game.tick();
            last_tick = Instant::now()
        }
        //game.tick();
    }
}
