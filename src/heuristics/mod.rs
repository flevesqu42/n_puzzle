use models::{Map, Move, Heuristic, Position, SelectedAlgorithm, SelectedHeuristic};

extern crate ansi_term;
pub use self::ansi_term::Style;
pub use self::ansi_term::Colour::{Blue, Cyan, Yellow, Red, Green, Black, White};
pub use self::ansi_term::{ANSIString, ANSIStrings};

pub fn get_g(parent_heuristic : &Heuristic, greedy : bool) -> i32 {
    match greedy {

        true => 0,
        _    => parent_heuristic.g + 1,

    }
}

pub fn get_h(map: &mut Map, algorithm : &SelectedAlgorithm, heuristic : &SelectedHeuristic, parent_heuristic : &Heuristic, map_move : &Move) -> i32 {
    match algorithm {

        &SelectedAlgorithm::SmartAstar => sum_of_goal_distances(map),
        _                              => {
            if heuristic == &SelectedHeuristic::Npuzzle {
                sum_of_goal_distances(map)
            }
            else {
                sum_of_distances(map, parent_heuristic, map_move)
            }
        },

    }
}

pub fn get_completed_count(map: &Map, x_min: usize, x_max: usize, y_min: usize, y_max: usize, line: usize) -> usize
{
    let mut count = 0;
    let mut x;
    let mut y;
    let mut i;

    match line {
        1 => {
            y = y_min;
            x = x_min;
            i = x + (y * map.size);
            while x <= x_max
                {
                    if map.cells[i].h == 0 && map.cells[i].number.value != 0
                        {
                            count = count + 1;
                        }
                    x = x + 1;
                    i = i + 1;
                }
                    return count;
        },
        2 => {
            y = y_min;
            x = x_max;
            i = x + (y * map.size);
            while y <= y_max
                {
                    if map.cells[i].h == 0 && map.cells[i].number.value != 0
                        {
                            count = count + 1;
                        }
                    y = y + 1;
                    i = i + map.size;
                }
            return count;

        },
        3 => {
            y = y_max;
            x = x_min;
            i = x + (y * map.size);
            while x <= x_max
                {
                    if map.cells[i].h == 0 && map.cells[i].number.value != 0
                        {
                            count = count + 1;
                        }
                    x = x + 1;
                    i = i + 1;
                }
                    return count;
        },
        4 => {
            y = y_min;
            x = x_min;
            i = x + (y * map.size);
            while y <= y_max
                {
                    if map.cells[i].h == 0 && map.cells[i].number.value != 0
                        {
                            count = count + 1;
                        }
                    y = y + 1;
                    i = i + map.size;
                }
              return count;
        },
        _ => return 0
    }

}

pub fn sum_of_goal_distances(map: &mut Map) -> i32 {
    const COEFF : i32 = 2;

    let mut h = 0;

    let map_width = (map.size - map.plus.x - map.minus.x) as i32;
    let map_height = (map.size - map.plus.y - map.minus.y) as i32;

    let x_min = map.plus.x;
    let x_max = map.size - 1 - map.minus.x;
    let y_min = map.plus.y;
    let y_max = map.size - 1 - map.minus.y;

    let mut check_x1 = true;
    let mut check_x2 = true;
    let mut check_y1 = true;
    let mut check_y2 = true;

    if map_width == 2 && map_height > 2
        {
            check_x1 = false;
            check_x2 = false;

        }
    else if map_height == 2 && map_width > 2
        {
            check_y1 = false;
            check_y2 = false;
        }

    let mut i = x_min + (map.size * y_min);
    let i_max = x_max + (map.size * y_max);
    let mut x = x_min;
    let mut y = y_min;

    while i <= i_max {
        let target = &map.cells[i].number.target;
        if map.cells[i].number.value != 0 {
            if check_y1 == true
                {
                   if target.y == y_min {
                       h += map.cells[i].h * COEFF;
                   }
                    else {
                        if y == y_min
                            {
                                h += 1;
                            }
                    }
                }
            if check_y2 == true
                {
                    if target.y == y_max
                        {
                            h += map.cells[i].h * COEFF;
                        }
                        else {
                            if y == y_max
                                {
                                    h += 1;
                                }
                        }
                }
            if check_x1 == true

                {
                    if target.x == x_min
                        {
                            h += map.cells[i].h * COEFF;
                        }
                    else {
                        if x == x_min
                            {
                                h += 1;
                            }
                    }
                }
            if check_x2 == true
                {
                    if target.x == x_max
                        {
                            h += map.cells[i].h * COEFF;
                        }
                    else {
                        if x == x_max
                            {
                                h += 1;
                            }
                    }
                }
        }
        i = i + 1;
        x = x + 1;
        if x > x_max
            {
                y = y + 1;
                x = 0;
                i = i + map.size - x_max - 1;
            }
    }
    return h;
}


pub fn sum_of_distances(map: &mut Map, parent_heuristic : &Heuristic, map_move : &Move) -> i32 {
        let old_h = map.cells[map_move.from.x + map_move.from.y * map.size].h;
        let new_h = map.cells[map_move.to.x + map_move.to.y * map.size].h;
        return parent_heuristic.h - old_h + new_h;
}

pub fn distance(target: &Position, heuristic: &SelectedHeuristic, x: i32, y: i32, map: &Map) -> i32 {
    match heuristic {
        &SelectedHeuristic::Manhattan => {
            manhattan_distance(target, x, y)
        },
        &SelectedHeuristic::ManhattanSquared => {
            manhattan_distance_squared(target, x, y)
        },
        &SelectedHeuristic::Npuzzle => {
            npuzzle_distance(target, x, y)
        },
        &SelectedHeuristic::LinearConflict => {
            linear_conflict(target, x, y, map)
        },
        &SelectedHeuristic::TilesOutOfRowAndColumn => {
            tiles(target, x, y)
        },
    }
}


pub fn get_conflicts_count(target: &Position, x: usize, y: usize, map: &Map) -> i32{
    let mut count = 0;
    let mut i;
    let limit;

    if target.x == x
        {
            if target.y > y
                {
                    i = x + map.size * y;
                    limit = x + (map.size * (map.size - 1));
                    while i <= limit
                        {
                            if map.cells[i].number.target.y < y && map.cells[i].number.target.x == x
                                {
                                    count = count + 1;
                                }
                            i = i + map.size;
                        }
                }
            else {
                i = x;
                limit = x + map.size * y;
                while i <= limit
                    {
                        if map.cells[i].number.target.y > y && map.cells[i].number.target.x == x
                            {
                                count = count + 1;
                            }
                        i = i + map.size;
                    }
            }
        }
    else if target.y == y
        {
            if target.x > x
                {
                    i = x + map.size * y;
                    limit = map.size - 1 + (map.size * y);
                    while i <= limit
                        {
                            if map.cells[i].number.target.x < x  && map.cells[i].number.target.y == y
                                {
                                    count = count + 1;
                                }
                            i = i + 1;
                        }
                }
                else {
                    i = map.size * y;
                    limit = x + map.size * y;
                    while i <= limit
                        {
                            if map.cells[i].number.target.x > x && map.cells[i].number.target.y == y
                                {
                                    count = count + 1;
                                }
                            i = i + 1;
                        }
                }
        }
    return count as i32;
}

pub fn npuzzle_distance(target: &Position, x: i32, y: i32) -> i32 {
    (x - target.x as i32).abs() + (y - target.y as i32).abs()
}

pub fn tiles(target: &Position, x: i32, y: i32) -> i32 {
    let mut h = 0;
    if x != target.x as i32 {
        h = h + 1;
    }
    if y != target.y as i32 {
        h = h + 1;
    }
    return h;
}

pub fn linear_conflict(target: &Position, x: i32, y: i32, map: &Map) -> i32 {
    match (x - target.x as i32).abs() + (y - target.y as i32).abs() {
        h if h == 0 => h,
        h           => h + get_conflicts_count(target, x as usize, y as usize, map)
    }
}

pub fn manhattan_distance(target: &Position, x: i32, y: i32) -> i32 {
    (x - target.x as i32).abs() + (y - target.y as i32).abs()
}

pub fn manhattan_distance_squared(target: &Position, x: i32, y: i32) -> i32 {
    let x = x - target.x as i32;
    let y = y - target.y as i32;
    x * x + y * y
}
