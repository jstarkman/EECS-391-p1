use std::str;
use std::fmt;

use rand;
use rand::{Rng, StdRng, SeedableRng};

const SIDE_LENGTH: usize = 3;
const GOAL_STATE: [char; SIDE_LENGTH * SIDE_LENGTH] = ['b','1','2', '3','4','5', '6','7','8'];
const DIRECTIONS: [&'static str; 4] = ["up", "down", "left", "right"];
const RNG_SEED: &'static[usize] = &[1,2,3,4];

pub trait Puzzle {
    fn set_state(&mut self, payload: &str);
    fn randomize_state(&mut self, payload: &str);
    fn print_state(&self, payload: &str);
    fn move_blank(&mut self, payload: &str) -> bool;
    fn solve(&self, payload: &str); // more splitting inside
    fn set_max_nodes(&mut self, payload: &str);
}

/// Represents the state of the puzzle in row-major order.  `max_nodes == 0`
/// means infinite.
pub struct State {
    state: Vec<char>,
    max_nodes: u32,
    rng: StdRng,
}

impl State {
    pub fn new() -> State {
        State { state: GOAL_STATE.to_vec(), max_nodes: 0, rng: SeedableRng::from_seed(RNG_SEED), }
    }
}

impl Puzzle for State {
    fn set_state(&mut self, payload: &str) {
        let compressed = str::replace(payload, " ", "");
        let mut i = 0;
        for c in compressed.chars() {
            self.state[i] = c;
            i += 1;
        }
    }

    /// Randomly chooses `payload` valid directions (invalid directions are
    /// re-rolled) and calls `move_blank()` to do each one.
    fn randomize_state(&mut self, payload: &str) {
        let mut i = convert_str_to_int(payload);
        let direction = &DIRECTIONS;
        let empty_string = &"";
        while i > 0 {
            let direction = self.rng.choose(direction).unwrap_or(empty_string);
            if self.move_blank(direction) {
                i -= 1;
            }
        }
    }

    /// Prints the state as a block.
    fn print_state(&self, payload: &str) {
        if payload.trim() == "" { // do not want to accept more arguments
            println!("{}", self.to_string());
        }
    }

    /// Returns true if swap was made, false otherwise.
    fn move_blank(&mut self, payload: &str) -> bool {
        let blank = self.state.iter().position(|&c| c == 'b').unwrap();
        let occupied = match payload.trim() {
            "up"    => if blank >= SIDE_LENGTH { blank - 3 } else { blank },
            "down"  => if blank < (SIDE_LENGTH * SIDE_LENGTH - SIDE_LENGTH) { blank + 3 } else { blank },
            "left"  => if blank % SIDE_LENGTH != 0 { blank - 1 } else { blank },
            "right" => if (blank + 1) % SIDE_LENGTH != 0 { blank + 1 } else { blank },
            _       => blank,
        };
        if blank != occupied { // pointless if true
            let tmp = self.state[blank]; // always 'b'
            self.state[blank] = self.state[occupied];
            self.state[occupied] = tmp;
        }
        blank != occupied
    }

    fn solve(&self, payload: &str) {
        lazy_static! {
            static ref TOKENIZER: Regex = Regex::new(r"(\w+)").unwrap();
        }
        
        let tokens = TOKENIZER.captures(payload);
        if tokens.is_none() { return; }
        let tokens = tokens.unwrap();
        let method = tokens.at(1).unwrap_or("");
        match method {
            "beam" => {
                self.solve_beam();
            },
            "A-star" => {
                let heuristic = tokens.at(2).unwrap_or("");
                match heuristic {
                    "h1" => self.solve_h1(),
                    "h2" => self.solve_h2(),
                    _    => println!("Please specify a heuristic.");
                };
            },
            _ => println!("Please specify a method."),
        };
    }

    fn h1(one: Vec<char>, other: Vec<char>) -> u32 {
        let mut out: u32 = 0;
        for it in one.iter().zip(other.iter()) {
            let (ai, bi) = it;
            out += (ai != bi) as u32;
        }
        out
    }
    
    /// always against the goal
    /// might be useful: `let goal_state = GOAL_STATE.to_vec();`
    fn h2(goal_state: Vec<char>, other: Vec<char>) -> u32 {
        let mut out: u32 = 0;
        for (other_pos, other_c) in other.iter().enumerate() {
            let other_pos = other_pos as i32;
            let goal_pos = goal_state.iter().position(|&c| c == *other_c).unwrap() as i32;
            let goal_x = goal_pos % SIDE_LENGTH as i32;
            let goal_y = goal_pos / SIDE_LENGTH as i32;
            let other_x = other_pos % SIDE_LENGTH as i32;
            let other_y = other_pos / SIDE_LENGTH as i32;
            let l1_norm = ((other_x - goal_x).abs() + (other_y - goal_y).abs()) as u32;
            out += l1_norm;
        }
        out
    }

    fn solve_beam(&self) {

    }

    fn solve_h1(&self) {

    }

    fn solve_h2(&self) {

    }

    fn set_max_nodes(&mut self, payload: &str) {
        self.max_nodes = convert_str_to_int(payload);
    }
}

impl fmt::Display for State {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let mut out = String::with_capacity((SIDE_LENGTH * (SIDE_LENGTH + 1)) as usize);
        let mut i = 0;
        for &c in &self.state {
            if c == 'b' {
                out.push(' ');
                //try!(fmt.write_char(' '));
            } else {
                out.push(c);
                //try!(fmt.write_char(c));
            }
            i += 1;
            if i % SIDE_LENGTH == 0 {
                //try!(fmt.write_char('\n'));
                out.push('\n');
            }
        }
        write!(fmt, "{}", out)
    }
}

fn convert_str_to_int(s: &str) -> u32 {
    match str::from_utf8(s.trim().as_bytes())
        .expect("should be UTF-8")
        .parse()
    {
        Ok(v)  => v,
        Err(e) => 0,
    }
}

