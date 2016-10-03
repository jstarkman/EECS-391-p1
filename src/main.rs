#[macro_use] extern crate lazy_static;
extern crate regex;

extern crate jas497_p1;

use std::io;
use std::io::prelude;
use std::io::Write;

use regex::Regex;

use jas497_p1::State;

mod lib;

/// Common backing function to handle input from either the REPL or a file.
fn execute_statement(state: jas497_p1::State, cmd: &str) {
    lazy_static! {
        static ref split_opcode_and_data: Regex = Regex::new(r"^(\w+)(.*)$").unwrap();
    }
    
    let parts = split_opcode_and_data.captures(cmd).unwrap();
    let opcode = parts.at(1).unwrap_or("");
    let payload = parts.at(2).unwrap_or("");

    match opcode {
        "setState"       => state.set_state(payload),
        "randomizeState" => state.randomize_state(payload),
        "printState"     => state.print_state(payload),
        "move"           => state.move_blank(payload),
        "solve"          => state.solve(payload), // more splitting inside
        "maxNodes"       => state.set_max_nodes(payload),
    }
}

fn repl() {
    let mut state = jas497_p1::State::new();
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
            _ => execute_statement(state, line.trim()),
        }
        line.clear();
    }
}

fn main() {
    println!("Hello, world!");
    repl();
}
