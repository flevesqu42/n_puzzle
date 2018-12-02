use super::*;
use super::common_functions::*;

pub fn a_star(states : & mut States, options: &Options) -> Option<ParentMove> {

    while states.opened.len() > 0
        {
            states.total_states += 1;
            let mut e = states.opened.pop().unwrap();
            if e.heuristic.h == 0 {
                match options.heuristic {
                    SelectedHeuristic::Npuzzle => {if is_solved(&e.map) {return Some(e.parent_move)}},
                    _                          => {return Some(e.parent_move)},
                }
            }
            let parent_move: Rc<ParentMove> = Rc::new(e.parent_move);
            let moves: Vec<Move> = get_valid_moves(&mut e.map);
            for valid_move in moves {
                let mut result = e.map.expand_map_from_move(&valid_move, &options, &states.closed);
                match result {
                    Ok(map) => {states.opened.push(new_state_from(map, &e.heuristic, parent_move.clone(), valid_move, &options));},
                    Err(_) => {}
                }
            }
            e.map.cells[e.map.blank.x + e.map.blank.y * e.map.size].h = 0;
            states.closed.insert(e.map);
        }
    None
}

pub fn is_solved(map: &Map) -> bool
{
    for cell in &map.cells {
        if cell.h != 0 && cell.number.value != 0
            {
                return false;
            }
    }
    return true;
}