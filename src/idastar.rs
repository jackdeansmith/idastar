
//A struct represents an element in a solvable state space if we can test to see if
//it is the goal we are looking for.
pub trait Solvable{
    fn is_goal(&self) -> bool;
}

//A struct represents an element in a searchable state space if we can
//get an iterator to its children states.
pub trait Searchable: Sized{
    type Transition: Sized;
    type SuccIter: Iterator<Item=(Self, Self::Transition)>;
    fn successors(&self) -> Self::SuccIter; 
}

//A struct is an element of an admissable state space if there exists an
//admissable heuristic for it.
pub trait Admissable{
    fn heuristic(&self) -> u64;
}

fn search<T:Solvable + Searchable + Admissable>(state: T, cost_limit: u64) 
-> Option<Vec<T::Transition>>{

    //The base case, if the cost limit is 0, we can't do anyting
    if cost_limit == 0{
        return None;
    }

    //Otherwise, we need to look recursively into the successors
    for (next_state, next_transition) in state.successors(){

    }

    None
}
