#[macro_use] extern crate lazy_static;
//extern crate regex;
extern crate rand;

use std::str;
use std::fmt;

//use regex::Regex;
use rand::{Rng, ThreadRng};

const SIDE_LENGTH: usize = 3;
const GOAL_STATE: [char; SIDE_LENGTH * SIDE_LENGTH] = ['b','1','2', '3','4','5', '6','7','8'];
const DIRECTIONS: [&'static str; 4] = ["up", "down", "left", "right"];

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
    rng: ThreadRng,
}

impl State {
    pub fn new() -> State {
        State { state: GOAL_STATE.to_vec(), max_nodes: 0, rng: rand::thread_rng(), }
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
        let direction = self.rng.choose(direction).unwrap_or(empty_string);
        while i > 0 && self.move_blank(direction) {
            i -= 1;
        }
    }

    /// Prints the state as a block.
    fn print_state(&self, payload: &str) {
        if payload.trim() == "" { // do not want to accept more arguments
            println!("{}", self.to_string());
        }
    }

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
        //FIXME
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
    std::str::from_utf8(s.trim().as_bytes())
        .expect("should be UTF-8")
        .parse()
        .expect("not a number")
}

