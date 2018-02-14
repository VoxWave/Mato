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
    wanted_direction: Direction,
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
        let mut snake_body = VecDeque::new();
        snake_body.push_front((11,12));
        Game {
            snake_direction: Direction::Right,
            wanted_direction: Direction::Right,
            snake_head: (12,12),
            snake_body: snake_body,
            apple: (range.ind_sample(&mut rand), range.ind_sample(&mut rand)),
            apple_eaten: false,
            running: true,
            range: range,
            ticks: 0,
            score: 0,
        }
    }

    // Pelikentän uudelleenalustus
    fn reset(&mut self) {
        self.score = 0;
        self.ticks = 0;
        self.snake_head =  (12,12);
        self.snake_body.clear();
        self.snake_body.push_front((11,12));
        self.snake_direction = Direction::Right;
        self.wanted_direction = Direction::Right;
        self.apple_eaten = false;
        self.running = true;
        let mut rand = rand::thread_rng();
        self.apple = (self.range.ind_sample(&mut rand), self.range.ind_sample(&mut rand));
    }

    // Pelin nopeuden laskenta.
    fn tick_duration(&self) -> u64 {
        let speedup = ((self.ticks as f32 + 1.).log2() * 100.) as u64;
        let slowdown = 1000 + self.score*10;
        if slowdown < speedup {
            0
        } else {
            slowdown - speedup
        }
    }

    fn tick(&mut self) {
        use Direction::*;
        self.ticks += 1;

        // Vältetään madon kääntymistä itseään kohti.
        self.snake_direction = match (self.snake_direction, self.wanted_direction) {
            (Up, Down) | (Down, Up) | (Left, Right) | (Right, Left) => self.snake_direction,
            _ => self.wanted_direction,
        };

        // Madon suunnan kuvaaminen liikutusvektoriksi.
        let (dir_x, dir_y) = match self.snake_direction {
            Up => (0, 1),
            Down => (0, -1),
            Left => (-1, 0),
            Right => (1, 0),
        };

        // Madon tämänhetkisen pään sijainti lisätään kehon osaksi.
        self.snake_body.push_front(self.snake_head.clone());

        // Madon päätä siirretään liikutusvektorin mukaisesti.
        let sh_x = &mut self.snake_head.0;
        let sh_y = &mut self.snake_head.1;
        *sh_x += dir_x;
        *sh_y += dir_y;

        // Pelikentän rajojen törmäystarkistus.
        // TODO: refaktoroi kovakoodatut reunaehdot pois.
        if *sh_x >= 25 || *sh_x < 0 || *sh_y >= 25 || *sh_y < 0 {
            self.running = false;
        }

        // Madon kasvatus.
        if self.apple_eaten {
            self.apple_eaten = false;
        } else {
            self.snake_body.pop_back();
        }
        
        // Törmäystarkistus oman kehon kanssa.
        for p in &self.snake_body {
            if *sh_x == p.0 && *sh_y == p.1 {
                self.running = false;
                break;
            }
        }

        // Omenan syöminen.
        if *sh_x == self.apple.0 && *sh_y == self.apple.1 {
            self.apple_eaten = true;
            self.score += 1;
            let mut rand = rand::thread_rng();
            self.apple = (self.range.ind_sample(&mut rand), self.range.ind_sample(&mut rand));
        }
    }
}

fn main() {
    use Direction::*;
    println!("Starting game!");

    // Luodaan ikkuna.
    let mut window: PistonWindow =
        WindowSettings::new("Mato!", (500, 500))
            .exit_on_esc(true)
            .build()
            .unwrap_or_else(|e| { panic!("Failed to build PistonWindow: {}", e) });
    
    // Luodaan peli.
    let mut game = Game::new();

    // Viimeisin tick alustetaan.
    let mut last_tick = Instant::now();

    // Syötteiden käsittely.
    while let Some(e) = window.next() {
        // Käärmeen ohjaus.
        if let &Input(Button(ButtonArgs { button: Keyboard(ref k), state: Press, ..})) = &e {
            game.wanted_direction = match *k {
                Key::W => Up,
                Key::A => Left,
                Key::S => Down,
                Key::D => Right,
                _ => game.wanted_direction,
            };
            // Pelin uudelleenalustus.
            match *k {
                Key::Return | Key::R => {
                    game.reset();
                },
                _ => {},
            }
        }

        // Piirtofunktio.
        window.draw_2d(&e, |c, g| {
            clear([0.5, 0.5, 0.5, 1.0], g);
            // Piirretään käärmeen pää.
            let (head_x, head_y) = game.snake_head;
            rectangle([0.0, 0.5, 0.0, 1.0], // green
                      [head_x as f64 * 20., 500. - (head_y as f64 + 1.) * 20., 20.0, 20.0],
                      c.transform, g);
            // Piirretään käärmeen kehon palat.
            for &(x, y) in &game.snake_body {
                rectangle([0.0, 0.4, 0.0, 1.0], // less green
                      [x as f64 * 20., 500. - (y as f64 + 1.) * 20., 20.0, 20.0],
                      c.transform, g);
            }
            // Piirretään omena.
            let (apple_x, apple_y) = game.apple;
            rectangle([1.0, 0.0, 0.0, 1.0], // red
                      [apple_x as f64 * 20., 500. - (apple_y as f64 + 1.) * 20., 20.0, 20.0],
                      c.transform, g);
        });

        // Peliloogikan askellus. Pelilogiikkaa edistetään vain jos kulunut tarpeeksi aikaa viimeisestä askeleesta. 
        if last_tick.elapsed() > Duration::from_millis(game.tick_duration()) && game.running {
            game.tick();
            last_tick = Instant::now()
        }
    }
}
