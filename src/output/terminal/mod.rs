extern crate ansi_term;

pub use self::ansi_term::Style;
pub use self::ansi_term::Colour::{Blue, Cyan, Yellow, Red, Green, Black, White};
pub use self::ansi_term::{ANSIString, ANSIStrings};

use models::{Move, Map};

pub fn print_map(map: &Map, solved: bool){
    let mut x: usize;
    let mut y: usize = 0;
    let max_w = (map.size * map.size).to_string().chars().count();
    let mut i: usize = 0;
    let mut color = Green;
    while y <= map.size
        {
            let mut j = (max_w * map.size) + ((map.size - 1) * 3) + 4;
            while j > 0
                {
                    print!("{}", "-");
                    j -= 1;
                }
            print!("{}", "\n");
            if y == map.size
                {
                    break;
                }
            x = 0;
            while x < map.size
                {
                    if x == 0
                        {
                            print!("{}", "| ");
                        }
                    let mut display = 0;
                    if solved == false
                        {
                            color = Red;
                            display = map.cells[i].number.value;
                            if map.cells[i].number.target.x == x && map.cells[i].number.target.y == y {
                                color = Green;
                            }
                            if map.cells[i].number.value == 0 {
                                color = Blue;
                            }
                        }
                        else {
                            let mut index = 0;
                            while index < map.size * map.size
                                {
                                    if map.cells[index].number.target.x == x && map.cells[index].number.target.y == y
                                        {
                                            display = map.cells[index].number.value;
                                        }
                                    index += 1;
                                }
                        }
                    let s = format!("{nb:>ws$}", nb=display, ws=max_w);
                    print!("{}", Style::new().fg(color).bold().paint(s));
                    x += 1;
                    if x < map.size
                        {
                            print!("{}", " | ");
                        }
                    i += 1;
                }
            y += 1;
            print!("{}", " |\n");
        }
    return
}

pub fn print_path(original_map : &Map, path : &Vec<Move>) {
    let mut map = original_map.clone();

    for my_move in path {
        print_map(&map, false);
        map.move_in_place(my_move);
    }

    print_map(&map, false);
}

pub fn print_soft_path(original_map : &Map, path : & Vec<Move>) {
    let mut map = original_map.clone();

    print_map(&map, false);

    for (i, my_move) in path.iter().enumerate() {
        match i {
            i if i > 0 => print!("; {} -> 0", map.cells[my_move.from.x + my_move.from.y * map.size].number.value),
            _          => print!("{} -> 0", map.cells[my_move.from.x + my_move.from.y * map.size].number.value),
        }
        map.move_in_place(my_move);
    }

    print!("\n");
    print_map(&map, false);
}