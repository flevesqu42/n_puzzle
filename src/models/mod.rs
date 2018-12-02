/* use */

use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet};
use std::rc::Rc;
use heuristics;

/* PROTOTYPES */

/* public enums */

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum ParentMove{
    Node(Move, Rc<ParentMove>),
    Nil,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum SelectedAlgorithm{
    Astar,
    SmartAstar
}
#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum SelectedHeuristic{
    Manhattan,
    ManhattanSquared,
    LinearConflict,
    TilesOutOfRowAndColumn,
    Npuzzle
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum SelectedOutput{
    Graphic,
    Terminal,
    None
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct Options{
    pub greedy: bool,
    pub heuristic: SelectedHeuristic,
    pub algorithm: SelectedAlgorithm,
    pub output: SelectedOutput
}

/* public structs */

#[derive(Debug)]
pub struct States {
    pub opened: BinaryHeap<State>,
    pub closed: HashSet<Map>,
    pub total_states : usize,
}

#[derive(Debug, Clone)]
pub struct Heuristic {
    pub g: i32,
    pub h: i32,
    pub f: i32,
}

#[derive(Debug, Clone)]
pub struct State{
    pub map: Map,
    pub heuristic: Heuristic,
    pub parent_move: ParentMove,
}

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub struct Position {
    pub x: usize,
    pub y: usize
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct Move {
    pub from: Position,
    pub to: Position,
}

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub struct Map {
    pub size: usize,
    pub plus:Position,
    pub minus:Position,
    pub cells: Vec<Cell>,
    pub blank: Position,
}

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub struct Number {
    pub value : usize,
    pub target : Position,
}

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub struct Cell {
    pub number: Rc<Number>,
    pub h: i32
}

/* TRAITS */

impl Ord for State {
    fn cmp(&self, other : &State) -> Ordering {
//        if self.map.offset.is_some(){
//            let offset = self.map.offset.unwrap();
//            let other_offset = other.map.offset.unwrap();
//            let map_size = self.map.size - offset.plus.x - offset.minus.x;
//            let other_map_size = other.map.size - other_offset.plus.x - other_offset.minus.x;
//            if map_size != other_map_size{
//               return map_size.cmp(&other_map_size);
//            }
//            other.heuristic.f.cmp(&self.heuristic.f)
//        }
//        else
//        {
        if self.heuristic.f == other.heuristic.f
            {
                other.heuristic.h.cmp(&self.heuristic.h)
            }
        else {
            other.heuristic.f.cmp(&self.heuristic.f)
        }

       // }

    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &State) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

impl Eq for State {}

impl PartialEq for State {
    fn eq(&self, other: &State) -> bool {
        self.heuristic.f == other.heuristic.f
    }
}

/* IMPLEMENTATIONS */

impl ParentMove {
    pub fn new(my_move: Move, parent: Rc<ParentMove>) -> ParentMove {
        ParentMove::Node(my_move, parent)
    }

    pub fn expand_path(&self) -> Vec<Move> {

        fn append_in_place(vec : & mut Vec<Move>, parent_move : &ParentMove) {
            match *parent_move {
                ParentMove::Node(ref last_move, ref parent) => {
                    vec.push(last_move.clone());
                    append_in_place(vec, parent);
                }
                ParentMove::Nil => {
                    vec.reverse();
                }
            }
        }

        let mut path : Vec<Move> = vec![];

        append_in_place(& mut path, self);
        path
    }
}

impl State {
    pub fn new(map: Map, heuristic: Heuristic, parent_move: ParentMove) -> State {
        State{map, heuristic, parent_move}
    }
}

impl States {
    pub fn new(initial_state: State) -> States {
        let closed = HashSet::new(); //TODO set capacity (en fonction de la taille de la map)
        let mut opened = BinaryHeap::new();

//        let closed = HashSet::with_capacity(initial_state.heuristic.h as usize * (initial_state.map.size * initial_state.map.size)); //TODO set capacity (en fonction de la taille de la map)
//        let mut opened = BinaryHeap::with_capacity(initial_state.heuristic.h as usize * (initial_state.map.size * initial_state.map.size));

        opened.push(initial_state);
        States{opened, closed, total_states: 0}
    }
}

impl Heuristic {
    pub fn new(g : i32, h : i32) -> Heuristic {
        Heuristic {g, h, f: g + h}
    }
}

impl Position {
    pub fn new(x: usize, y: usize) -> Position {
        Position{x, y}
    }
}

impl Move {
    pub fn new(from: Position, to: Position) -> Move {
        Move{from, to}
    }
}

impl Map
{
    pub fn new(size: usize, cells: Vec<Cell>, blank: Position) -> Map {
        Map { size, plus:Position::new(0, 0), minus:Position::new(0, 0), cells, blank }
    }

    pub fn expand_map_from_move(&self, my_move: &Move, options : &Options, closed: &HashSet<Map>) -> Result<(Map), bool> {
        let mut new_map = (*self).clone();

        let from = my_move.from.x + my_move.from.y * new_map.size;
        let to = my_move.to.x + my_move.to.y * new_map.size;

        let tmp = new_map.cells[from].number.clone();
        new_map.cells[from].number = new_map.cells[to].number.clone();
        new_map.cells[to].number = tmp;
        new_map.blank = my_move.from.clone();
        let old_h = new_map.cells[from].h;
        new_map.cells[from].h = 0;
        new_map.cells[to].h = heuristics::distance(&new_map.cells[to].number.target, &options.heuristic, my_move.to.x as i32, my_move.to.y as i32, &new_map);
        if closed.contains(&new_map) {
            return Err(false);
        }
        new_map.cells[from].h = old_h;
        return Ok(new_map)
    }

    pub fn move_in_place(& mut self, my_move : &Move) {

        let from = my_move.from.x + my_move.from.y * self.size;
        let to = my_move.to.x + my_move.to.y * self.size;

        let tmp = self.cells[from].number.clone();
        self.cells[from].number = self.cells[to].number.clone();
        self.cells[to].number = tmp;

        self.blank = my_move.from.clone();
    }
}

impl Number {
    pub fn new(value : usize, target : Position ) -> Number {
        Number {value, target}
    }
}

impl Cell {
    pub fn new(number: usize, sz: usize) -> Self {
        let mut target = Position::new(0, 0);
        let mut max_s: usize = sz - 1;
        let mut rest: usize = number;
        let mut sens: usize = 0;
        if number == 0 {
            rest = sz * sz;
        }
        while max_s > 0
            {
                if rest > max_s * 4
                    {
                        rest = rest - (max_s * 4);
                        max_s -= 2;
                        target.y += 1;
                        target.x += 1;
                    } else {
                    sens = (rest - 1) / max_s;
                    rest = rest % max_s;
                    break;
                }
            }
        if rest == 0 {
            rest = max_s;
        }
        rest -= 1;
        let target = match sens {
            0 => Position { x: target.x + rest, y: target.y },
            1 => Position { x: target.x + max_s, y: target.y + rest },
            2 => Position { x: target.x + max_s - rest, y: target.y + max_s },
            3 => Position { x: target.x, y: target.y + max_s - rest },
            _ => Position { x: target.x, y: target.y },
        };
        let insert_number = Rc::new(Number::new(number, target));
        Cell {number: insert_number.clone(), h:0}
    }
}