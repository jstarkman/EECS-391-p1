#[macro_use] extern crate lazy_static;
extern crate regex;
extern crate rand;

use std::io;
//use std::io::prelude;
use std::io::Write;

use regex::Regex;

// use std::cell::{RefCell, RefMut};
// use std::rc::Rc;
// use std::borrow::BorrowMut;

mod puzzle;
use puzzle::{Puzzle, State};

/// Common backing function to handle input from either the REPL or a file.
fn execute_statement(state: &mut State, cmd: &str) {
    lazy_static! {
        static ref SPLIT_OPCODE_AND_DATA: Regex = Regex::new(r"^(\w+)(.*)$").unwrap();
    }
    
    //let mut state = state.borrow();
    
    let parts = SPLIT_OPCODE_AND_DATA.captures(cmd);
    if parts.is_none() { return; }
    let parts = parts.unwrap();
    let opcode = parts.at(1).unwrap_or("");
    let payload = parts.at(2).unwrap_or("");

    match opcode {
        "setState"       => state.set_state(payload),
        "randomizeState" => state.randomize_state(payload),
        "printState"     => state.print_state(payload),
        "move"           => if !state.move_blank(payload) { println!("Illegal move"); },
        "solve"          => state.solve(payload), // more splitting inside
        "maxNodes"       => state.set_max_nodes(payload),
        ""               => println!(""),
        _                => println!("unrecognized command, no action taken"),
    };
}

fn repl() {
    let mut state = State::new(); //Rc::new(RefCell::new(State::new()));
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
    println!("Hello, world!");
    repl();
}
