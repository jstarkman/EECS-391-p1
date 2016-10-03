// Inspiration: https://github.com/murarth/rusti/blob/master/src/rusti/repl.rs

use std::io;
use std::io::prelude;
use std::io::Write;

static mut counter: i32 = 0;

fn execute_statement(cmd: &str) {
    unsafe {
        counter += 1;
        println!("{} &&& {}", cmd, counter);
    }
}

/// Borrows generously from L145 of reference
fn repl() {
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
            _ => execute_statement(line.trim()),
        }
        line.clear();
    }
}

fn main() {
    println!("Hello, world!");
    repl();
}
