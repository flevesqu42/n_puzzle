use models::Map;
//use output;

pub fn check_solvability(map: &mut Map) -> bool {
    let map_size = map.size as i32;
    let mut size_even = false;
    if map_size % 2 == 0 {
        size_even = true;
    }
    let mut blank_even = false;
    let mut permut_count = 0;
    let permut_even;
    let mut i: usize = 0;
    let mut j;


    if (map.blank.x + map.blank.y) % 2 == 0 {
        blank_even = true;
    }
    while i < map.size * map.size
        {
            j = i + 1;

            if j < map.size * map.size
                {

                    if map.cells[i].number.target.x + (map.cells[i].number.target.y * map.size) > map.cells[j].number.target.x + (map.cells[j].number.target.y * map.size)
                        {
                            let number_i = map.cells[i].number.clone();
                            map.cells[i].number = map.cells[j].number.clone();
                            map.cells[j].number = number_i;
                            permut_count += 1;
                            i = 0;

                        }
                        else {
                            i += 1;
                        }
                }
                else {
                    i += 1;
                }
        }
    //output::terminal::print_map(&map,  false);


    if permut_count % 2 == 0 {
        permut_even = true;
    }
        else {
            permut_even = false;
        }
    //println!("permut count {}, permut even {}, blank even {}", permut_count, permut_even, blank_even);
    if size_even == false {
        if permut_even == true && blank_even == true{
            return true;
        }
        if permut_even == false && blank_even == false{
            return true;
        }
    }
        else {
            if permut_even == true
                {
                    return true;
                }
            if blank_even == true && permut_even == false
                {
                    return true;
                }
        }
    return false;
}