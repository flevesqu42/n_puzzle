use super::*;
use super::common_functions::*;
use models::SelectedOutput;

pub fn smart_a_star(states : & mut States, options: &Options, mut smallest_map : usize) -> Option<ParentMove> {

    while states.opened.len() > 0
        {
            states.total_states += 1;
            let mut e = states.opened.pop().unwrap();

            while is_completed_goal(states, &mut e.map, &mut smallest_map) {
                if options.output == SelectedOutput::Terminal {
                    output::terminal::print_map(&e.map,  false);
                }
                if get_map_size(&e.map) <= 1 {
                    return Some(e.parent_move);
                }
            }
            let moves: Vec<Move> = get_valid_moves(&mut e.map);
            let parent_move: Rc<ParentMove> = Rc::new(e.parent_move);
            for valid_move in moves {
                let mut result = e.map.expand_map_from_move(&valid_move, &options, &states.closed);
                match result {
                    Ok(map) => { states.opened.push(new_state_from(map, &e.heuristic, parent_move.clone(), valid_move, &options));},
                    Err(_) => {}
                }
            }
            e.map.cells[e.map.blank.x + e.map.blank.y * e.map.size].h = 0;
            states.closed.insert(e.map);
        }
    None
}

fn is_completed_goal(states : & mut States, map : &mut Map, smallest_map: &mut usize) -> bool {

    let map_width = map.size - map.plus.x - map.minus.x;
    let map_height = map.size - map.plus.y - map.minus.y;

    let x_min = map.plus.x;
    let x_max = map.size - 1 - map.minus.x;
    let y_min = map.plus.y;
    let y_max = map.size - 1 - map.minus.y;


    let mut check_x = true;
    let mut check_y = true;

    if map_width == 2 && map_height > 2
        {
            check_x = false;
        } else if map_height == 2 && map_width > 2
        {
            check_y = false;
        }

    if check_y == true
        {
            if get_completed_count(&map, x_min, x_max, y_min, y_max, 1) == map_width
                {
                    map.plus.y = map.plus.y + 1;
                    let new_size = get_map_size(map);
                    if new_size < *smallest_map{
                        set_new_smallest_map(states, smallest_map, new_size);
                    }
                    return true;
                }
                else if get_completed_count(&map, x_min, x_max, y_min, y_max, 3) == map_width
                    {
                        map.minus.y = map.minus.y + 1;
                        let new_size = get_map_size(map);
                        if new_size < *smallest_map{
                            set_new_smallest_map(states, smallest_map, new_size);
                        }
                        return true;
                    }
        }
    if check_x == true
        {
            if get_completed_count(&map, x_min, x_max, y_min, y_max, 2) == map_height
                {
                    map.minus.x = map.minus.x + 1;
                    let new_size = get_map_size(map);
                    if new_size < *smallest_map {
                        set_new_smallest_map(states, smallest_map, new_size);
                    }
                    return true;
                }
                else if get_completed_count(&map, x_min, x_max, y_min, y_max, 4) == map_height
                    {
                        map.plus.x = map.plus.x + 1;
                        let new_size = get_map_size(map);
                        if new_size < *smallest_map {
                            set_new_smallest_map(states, smallest_map, new_size);
                        }
                        return true;
                    }
        }
    false
}


pub fn get_map_size(map: &Map) -> usize
{
    (map.size - map.minus.x - map.plus.x) * (map.size - map.minus.y - map.plus.y)
}

pub fn set_new_smallest_map(states : & mut States, smallest_map : & mut usize, new_size : usize) {

    *smallest_map = new_size;

    for state in states.opened.drain() {
        states.closed.insert(state.map);
    }
}