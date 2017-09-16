
//A struct represents an element in a solvable state space if we can test to see if
//it is the goal we are looking for.
pub trait Solvable{
    fn is_goal(&self) -> bool;
}

//A struct represents an element in a searchable state space if we can
//get an iterator to its children states.
pub trait Searchable{
    type Move;
    fn successors(&self) -> Iterator<Item=(Self::Move, Self)>;
}

//A struct is an element of an admissable state space if there exists an
//admissable heuristic for it.
pub trait Admissable{
    fn heuristic(&self) -> u64;
}

fn search<T:Solvable + Searchable + Admissable>(state: T, cost_limit: u64) 
-> Option<Vec<T::Move>>{

    None
}
