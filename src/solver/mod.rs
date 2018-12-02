extern crate ansi_term;

use heuristics::{sum_of_goal_distances, get_g, get_h, distance, get_completed_count};
use models::{Map, States, State, ParentMove, Move, Heuristic, Position, Options, SelectedAlgorithm};

use self::ansi_term::Colour::{Red};

use std::rc::Rc;
use output;

use std;

use self::solvability::check_solvability;

pub mod solvability;

mod astar;
mod smart_astar;
mod common_functions;

use self::astar::a_star;
use self::smart_astar::smart_a_star;

use models::SelectedHeuristic;

pub fn solve(map: Map, options: Options)
{
    if !check_solvability(&mut map.clone())
        {
            println!("{}", Red.bold().paint("Map cannot be solved \u{1F480}"));
            std::process::exit(1);
        }
//    std::process::exit(0);

    let mut states: States = States::new(get_initial_state_from(map.clone(),  &options));
    let path = match options.algorithm {

        SelectedAlgorithm::SmartAstar => smart_a_star(& mut states, &options, map.size * map.size),
        SelectedAlgorithm::Astar      => a_star(& mut states, &options),

    };
    output::display_result(&map, &path, &states, &options);
}


fn get_initial_state_from(mut initial_map: Map,  options: &Options) -> State {
    let mut h = 0;
    let mut i = 0;
    let mut x = 0;
    let mut y = 0;
    while i < initial_map.size * initial_map.size {
        if initial_map.cells[i].number.value != 0
            {
                let dist = distance(&initial_map.cells[i].number.target, &options.heuristic, x, y, &initial_map);
                initial_map.cells[i].h = dist;
                h += dist;
            }
        i += 1;
        x += 1;
        if x == initial_map.size as i32
            {
                y += 1;
                x = 0;
            }
    }

    if options.algorithm == SelectedAlgorithm::SmartAstar || options.heuristic == SelectedHeuristic::Npuzzle {
        h = sum_of_goal_distances(&mut initial_map);
    }
    let initial_heuristic = Heuristic::new(0, h);

    State::new(initial_map, initial_heuristic, ParentMove::Nil)
}

