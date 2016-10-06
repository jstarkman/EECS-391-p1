#[macro_use] extern crate lazy_static;
extern crate regex;
extern crate rand;
extern crate time;

use std::env;
use std::io;
use std::io::{Write, BufReader, BufRead};
use std::fs::File;

use regex::Regex;

use time::PreciseTime;

mod puzzle;
use puzzle::{Puzzle, State};

/// Common backing function to handle input from either the REPL or a file.
fn execute_statement(state: &mut State, cmd: &str) {
    lazy_static! {
        static ref SPLIT_OPCODE_AND_DATA: Regex = Regex::new(r"^(\w+)(.*)$").unwrap();
    }
    
    let parts = SPLIT_OPCODE_AND_DATA.captures(cmd);
    if parts.is_none() { return; }
    let parts = parts.unwrap();
    let opcode = parts.at(1).unwrap_or("");
    let payload = parts.at(2).unwrap_or("");

    match opcode {
        "s" | "setState"       => state.set_state(payload),
        "r" | "randomizeState" => state.randomize_state(payload),
        "p" | "printState"     => state.print_state(payload),
        "move"           => if !state.move_blank(payload) { println!("Illegal move"); },
        "solve"          => {
            let start = PreciseTime::now();
            state.solve(payload); // more splitting inside
            let end = PreciseTime::now();
            println!("Took: {} seconds.", start.to(end));
        }
        "maxNodes"       => state.set_max_nodes(payload),
        ""               => println!(""),
        _                => println!("unrecognized command, no action taken"),
    };
}

fn repl() {
    let mut state = State::new();
    let mut line = String::new();
    loop {
        // about halfway down: <https://dfockler.github.io/2016/09/15/lalrpop.html>
        print!("P1> ");
        // ensure print has finished
        let _ = io::stdout().flush();
        // read in the command
        io::stdin()
            .read_line(&mut line)
            .ok()
            .expect("Failed to read the line.");
        // cheap grammar parsing
        match line.trim() {
            "q" | "quit" | "exit" => break,
            _ => execute_statement(&mut state, line.trim()),
        }
        line.clear();
    }
}

fn main() {
    println!("Hello, 391 grader!");
    let argv = env::args();
    if argv.len() == 2 {
        let f = argv.last().unwrap().to_string();
        let f = match File::open(f) {
            Ok(file) => file,
            _        => {
                println!("Failed to open file.");
                return;
            }
        };
        let mut state = State::new();
        let file = BufReader::new(&f);
        for line in file.lines() {
            let l = line.unwrap();
            println!("P1> {}", l);
            execute_statement(&mut state, l.trim());
        }
    } else {
        repl();
    }
}
