use std::fmt::Display;

//A struct represents an element in a solvable state space if we can test to see if
//it is the goal we are looking for.
pub trait Solvable: Display{
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

//Subtract unsigned numbers without wraping around past 0
fn saturate_subtract(lhs: u64, rhs: u64) -> u64{
    if lhs > rhs{
        lhs - rhs
    }
    else{
        0
    }
}

fn search<T:Solvable + Searchable + Admissable>(state: &T, cost_limit: u64) 
-> Option<Vec<T::Transition>>{

    //If our state is the solution, we are set, return an empty vector.
    if state.is_goal(){
        return Some(Vec::new());
    }

    //The base case, if the cost limit is 0, we can't do anyting
    if cost_limit == 0{
        return None;
    }

    //Otherwise, we need to look recursively into the successors
    for (next_state, next_transition) in state.successors(){

        //Compute the new cost limit, being safe to keep this from wrapping around
        let mut next_cost_limit = saturate_subtract(cost_limit, 1);
        next_cost_limit = saturate_subtract(next_cost_limit, 
                                            next_state.heuristic());

        //If any of our successors succeded in finding the goal, we are good!
        if let Some(mut _transition_vec) = search(&next_state, next_cost_limit){
            _transition_vec.push(next_transition);
            return Some(_transition_vec);
        }
    }

    //Finally, if we searched everything we could and still couldn't find a
    //an acceptable solution, we return none.
    None
}

pub fn idastar<T:Solvable + Searchable + Admissable>(state: T, cost_limit: u64) 
-> Option<Vec<T::Transition>>{
    for limit in 0..cost_limit{
        if let Some(mut transition_vec) = search(&state, limit){
            transition_vec.reverse();
            return Some(transition_vec);
        }
    }
    None
}
