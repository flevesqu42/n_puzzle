use super::*;

pub fn get_valid_moves(map : &mut Map) -> Vec<Move> {
    let mut moves: Vec<Move> =  Vec::with_capacity(4);

    if map.blank.x > map.plus.x {
        moves.push(Move::new(Position::new(map.blank.x - 1, map.blank.y), map.blank.clone()));
    }
    if map.blank.x < map.size - 1 - map.minus.x {
        moves.push(Move::new(Position::new(map.blank.x + 1, map.blank.y), map.blank.clone()));
    }
    if map.blank.y > map.plus.y {
        moves.push(Move::new(Position::new(map.blank.x, map.blank.y - 1), map.blank.clone()));
    }
    if map.blank.y < map.size - 1 - map.minus.y {
        moves.push(Move::new(Position::new(map.blank.x, map.blank.y + 1), map.blank.clone()));
    }
    return moves;
}

pub fn new_state_from(mut map : Map, parent_heuristic : &Heuristic, parent_move : Rc<ParentMove>, valid_move : Move,  options: &Options) -> State {

    let g = get_g(parent_heuristic, options.greedy);
    let h = get_h(&mut map, &options.algorithm, &options.heuristic, parent_heuristic, &valid_move);

    let heuristic = Heuristic::new(g, h);
    State::new(map, heuristic, ParentMove::new(valid_move, parent_move))
}
