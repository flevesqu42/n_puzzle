use std::env;
use std::fs::File;
use std::io::prelude::*;
extern crate regex;
use self::regex::Regex;
extern crate ansi_term;
pub use self::ansi_term::Style;
pub use self::ansi_term::Colour::{Blue, Cyan, Yellow, Red, Green, Black, White, Purple};
pub use self::ansi_term::{ANSIString, ANSIStrings};
use models::{Cell, Map, Position, Options, SelectedAlgorithm, SelectedHeuristic, SelectedOutput};

pub fn get_cells(to_parse: &str, size: usize) -> Result<(Vec<Cell>, Position), String> {
    let mut lines = to_parse.lines();
    let mut indiv_line = lines.next();
    let comment = self::Regex::new(r"(?P<numbers>[0-9 ]+)").unwrap();
    let mut cells: Vec<Cell> =  vec![];
    let mut blank = Position::new(0, 0);
    let mut y : usize = 0;
    loop {
        match indiv_line {
            Some(s) => {
                let mut x : usize = 0;
                let caps = comment.captures(s).unwrap();
                let mut numbers = caps["numbers"].split_whitespace();
                let mut nb = numbers.next();
                loop {
                    match nb {
                        Some(s) => {
                            let mut nb : usize = s.parse().unwrap();
                            if nb >= size * size
                                {
                                    return Err(String::from(format!("Number {} has obviously nothing to do here..", nb)))
                                }
                            let new_cell = Cell::new(nb, size);
                            if cells.iter().any(|s| s == &new_cell)
                                {
                                    return Err(String::from(format!("No identitical numbers allowed \u{1F51E}\n{} == {} !", nb, nb)))
                                }
                            cells.push(new_cell);
                            if nb == 0
                                {
                                   blank.x = x;
                                   blank.y = y;
                                }
                            x += 1;
                        },
                        None => break,
                    }
                    nb = numbers.next();
                };
            },
            None => break,
        }
        indiv_line = lines.next();
        y += 1;
    }
    return Ok((cells, blank));
}

pub fn get_options(args: &String) -> Options{
    let mut heuristic = SelectedHeuristic::ManhattanSquared;
    let mut algorithm = SelectedAlgorithm::Astar;
    let mut output = SelectedOutput::None;
    let mut greedy = false;
    let mut selected_h = "Manhattan Squared";
    let mut selected_a = "Astar";
    let mut selected_o = "None";

    let re = Regex::new(r"^(?P<program>[^\s]+)([\s]+)(?P<file_name>[^\s]+)((?P<arguments>((?P<a>([\s]+)-a)[\s]+((?P<algo>(?P<astar>[A|a]star)|(?P<sastar>[S|s]mart[\s]*[A|a]star)))|(?P<g>([\s]+)-g)|(?P<o>([\s]+)-o)[\s]+((?P<output>(?P<graphic>[G|g]raphic)|(?P<terminal>[T|t]erminal)))|(?P<h>([\s]+)-h)[\s]+((?P<heuristic>(?P<manhattan>[M|m]anhattan)|(?P<msquared>[M|m]anhattan[\s]*[S|s]quared)|(?P<lconflict>[L|l]inear[\s]*[C|c]onflict)|(?P<npuzzle>[N|n][ -]?puzzle)|(?P<tiles>[T|t]iles))))+)?$)").unwrap();
    if re.is_match(args)
        {
            let caps = re.captures(args).unwrap();
            if caps.name("arguments").is_some() {
                //println!("{:?}",caps.name("arguments").unwrap());
                         if caps.name("a").is_some() {
                    if !caps["algo"].to_string().is_empty()
                        {
                            if caps.name("sastar").is_some() {
                                selected_a = "Smart Astar";
                                algorithm = SelectedAlgorithm::SmartAstar;
                            }
                            println!("{}", Green.bold().paint(format!("Selected algorithm : {}", selected_a)));
                        }
                }
                else {
                    println!("{}", Purple.bold().paint("No algorithm specified, using default settings."));
                }
                if caps.name("o").is_some() {
                    if !caps["output"].to_string().is_empty()
                        {
                            if caps.name("graphic").is_some() {
                                selected_o = "Graphic";
                                output = SelectedOutput::Graphic;
                            }
                            else if caps.name("terminal").is_some() {
                                selected_o = "Terminal";
                                output = SelectedOutput::Terminal;
                            }
                            println!("{}", Green.bold().paint(format!("Selected output : {}", selected_o)));
                        }
                }
                    else {
                        println!("{}", Purple.bold().paint("No output specified, using default settings."));
                    }
                if caps.name("h").is_some() {
                    if !caps["heuristic"].to_string().is_empty()
                        {
                            if caps.name("manhattan").is_some() {
                                selected_h = "Manhattan";
                                heuristic = SelectedHeuristic::Manhattan;
                            }
                            else if caps.name("npuzzle").is_some() {
                                selected_h = "N-Puzzle";
                                heuristic = SelectedHeuristic::Npuzzle;
                            }
                                else if caps.name("lconflict").is_some() {
                                    selected_h = "Linear Conflict";
                                    heuristic = SelectedHeuristic::LinearConflict;
                                }
                                    else if caps.name("tiles").is_some() {
                                        selected_h = "Tiles out of Row and Column";
                                        heuristic = SelectedHeuristic::TilesOutOfRowAndColumn;
                                    }
                            println!("{}", Green.bold().paint(format!("Selected heuristic : {}", selected_h)));
                        }
                }
                    else {
                        println!("{}", Purple.bold().paint("No heuristic specified, using default settings."));
                    }
                if caps.name("g").is_some() {
                    greedy = true;
                    println!("{}", Green.bold().paint("Greedy mode selected."));
                }
                else {
                    println!("{}", Purple.bold().paint("Using default non-greedy mode."));
                }
            }
            else {
                println!("{}", Purple.bold().paint("No arguments given, using default settings."));
            }
        }
        else {
            println!("{}", Red.bold().paint("Wrong arguments given, using default settings."));
        }
    Options{greedy, heuristic, algorithm, output}
}

pub fn get_map() -> Result<(Box<Map>, Options), String> {
    let args: Vec<String> = env::args().collect();
    let string_args: String = args.join(" ");
    if env::args().count() < 2 {
        return Err(String::from("Gimme filz plz \u{1F64F}\n\nUsage: ./npuzzle [map] [-a (astar | smart astar)] [-g] [-o (terminal|graphic)] [-h (manhattan | manhattan squared | linear conflict | npuzzle | tiles)]"))
    };
            let filename = &args[1];
            let f = File::open(filename);
    match f {
        Result::Ok(mut file) => {
            let mut contents = String::new();
            match file.read_to_string(&mut contents) {
                Err(_e) => return Err(String::from("Cannot read given file. :(")),
                _       => {}
            }
//            .expect("Cannot read given file.");
            let re = Regex::new(r"^([ ]*[#][^\n]*\n)*([ ]*(?P<map_size>[\d]+)[ ]*([#][^\n]*)*\n){1}");
            match re {
                Ok(regex) => {
                    if regex.is_match(&contents)
                        {
                            let caps = regex.captures(&contents).unwrap();
                            let map_size_result = caps["map_size"].parse();
                            let mut map_size:usize;
                            match map_size_result {
                                Ok(size) => { map_size = size},
                                Err(_) => return Err(String::from("Wrong format \u{1F63E}"))
                            }
                            if map_size < 2
                                {
                                    println!("{}", map_size);
                                    return Err(String::from("The map is too small :(\nNO FUNZ IN DAT \u{1F63F}"))
                                }
                            let string = format!(r"^((#[^\n]*\n)*([ ]*(\d+)[ ]*([#][^\n]*)*[\n]){0}(?P<map>(([ ]*(\d+)[ ]+){1}{2}{3}(\d+)[ ]*([#][^\n]*)?\n){4}{5}{6})[\n ]*)$", "{1}", "{", map_size - 1, "}", "{", map_size, "}");
                            let re2 = Regex::new(&string);
                            match re2 {
                                Ok(regex) => {

                                    if regex.is_match(&contents)
                                        {
                                            let title = format!("{} {} x {} {}", "\u{1F5FA} ", map_size, map_size, "\u{1F408}");
                                            println!("{}", Cyan.bold().paint(title));
                                            let caps = regex.captures(&contents).unwrap();
                                            match get_cells(&caps["map"], map_size) {
                                                Result::Ok((cells, blank)) => {
                                                    return Ok((Box::new(Map::new(map_size, cells, blank)), get_options(&string_args)));
                                                },
                                                Result::Err(err) => return Err(err)
                                            }
                                        } else {
                                        return Err(format!("Given puzzle does not match expected {} * {} format.", map_size, map_size));
                                    }

                                },
                                Err(_) => return Err(format!("Map {} x {} is too large.", map_size, map_size)),
                            }

                        } else {
                        Err(String::from("Wrong format \u{1F63E}"))
                    }
                }
                ,
                Err(_) => return Err(String::from("Wrong format \u{1F63E}")),
            }
        },
        Result::Err(_err) =>
            return Err(String::from("Filz no find \u{1F640}"))
    }
}