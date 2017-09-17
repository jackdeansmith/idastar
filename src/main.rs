pub mod idastar;

//Out state will be a single u64

//Our solution is arbitrary, when we get to 20
impl idastar::Solvable for u64{
    fn is_goal(&self) -> bool{
        self == &1000000u64
    }
}

//We will first test without using the heuristic functionality
impl idastar::Admissable for u64{
    fn heuristic(&self) -> u64{
        0
    }
}

//We can either move right or left
#[derive(Debug)]
pub enum Direction{
    Right,
    Left,
}


pub struct MoveIter{
    number: u64,
    direction: Option<Direction>,
}

impl Iterator for MoveIter{
    type Item = (u64, Direction);
    fn next(&mut self) -> Option<Self::Item>{
        match self.direction{
            None => None,
            Some(Direction::Left) => {self.direction = Some(Direction::Right);
                                     Some((self.number*2, Direction::Left))}, 
            Some(Direction::Right) => {self.direction = None;
                                      Some((self.number*2 + 1, Direction::Right))}, 
        }
    }
}

impl idastar::Searchable for u64{
    type Transition = Direction;
    type SuccIter = MoveIter;
    fn successors(&self) -> MoveIter{
        MoveIter{number: self.clone(), direction: Some(Direction::Left)}
    }
}

fn main() {
    let moves = idastar::idastar(1, 1000);
    match moves{
        Some(vector) => println!("Moves: {:?}", vector),
        None => (),
    }
}
