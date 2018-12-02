pub mod terminal;
pub mod graphical;

use models::{Map, Move, Options, ParentMove, SelectedOutput, States};

pub fn display_result(map : &Map, opt_path : &Option<ParentMove>, states : & States, options : &Options) {
    match opt_path {
        &Some(ref last_parent_move)  => {
            let path = last_parent_move.expand_path();

            match options.output {
                SelectedOutput::Graphic => {
                    display_normal_output(path.len(), states);
                    display_graphical_path(map, &path);
                },
                SelectedOutput::Terminal => {
                    display_terminal_path(map, &path);
                    display_normal_output(path.len(), states);
                },
                SelectedOutput::None => {
                    display_none_path(map, &path);
                    display_normal_output(path.len(), states);
                }
            }
        }
        _ => {
            println!("{}", terminal::Red.bold().paint("Map cannot be solved \u{1F480}"));
            println!("{}", terminal::Blue.bold().paint(format!("Total states ever selected in the opened set : {}", states.total_states)));
        }
    }
}

fn display_normal_output(len_path : usize, states : & States) {
    if len_path == 0 {
        println!("{}", terminal::Green.bold().paint("Map is already solved ! No moves required !"));
        return ;
    }
    println!("{}", terminal::Green.bold().paint(format!("Map solved in {} moves ! Yipii ! \u{1F389}", len_path)));
    println!("{}", terminal::Blue.bold().paint(format!("Total states ever selected in the opened set : {}", states.total_states)));
    println!("{}", terminal::Blue.bold().paint(format!("Maximum number of states ever represented in memory : {}", states.opened.len() + states.closed.len())));
}

fn display_graphical_path(map : &Map, path : &Vec<Move>) {

    let mut display = graphical::init_display();

    graphical::display_path(& mut display, &map, &path);

    display.main_loop();

}

fn display_terminal_path(map : &Map, path : &Vec<Move>) {

    terminal::print_path(map, path);
}

fn display_none_path(map : &Map, path : &Vec<Move>) {

    terminal::print_soft_path(map, path);
}