mod models;
mod parser;
mod solver;
mod output;
mod heuristics;

extern crate ansi_term;

use self::ansi_term::Colour::{Red};


fn main() {
    match parser::get_map() {
        Result::Ok((map, options)) => {
            let mut map = *map;

            solver::solve(map, options);
        },
        Result::Err(err) => println!("{}", Red.bold().paint(err)),
    }
}