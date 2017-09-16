//Get the absolute difference between two unsigned integers
fn unsigned_distance(first: usize, second: usize) -> usize{
    let larger: usize;
    let smaller: usize;
    if first >= second{
        larger = first;
        smaller = second;
    }
    else{
        larger = second;
        smaller = first;
    }
    larger - smaller
}

//A point on a "manhattan style" grid. Distance is manhattan distance
struct ManhattanPoint{
    x: usize,
    y: usize,
}

impl ManhattanPoint{
    fn new(x_in: usize, y_in: usize) -> ManhattanPoint{
        ManhattanPoint{x: x_in, y: y_in}
    }
    fn distance(&self, other: &ManhattanPoint) -> usize{
        //Get the x and y distances
        let xdist = unsigned_distance(self.x, other.x);
        let ydist = unsigned_distance(self.y, other.y);

        //Return their sum
        xdist + ydist
    }
}

//A board is just represented as a slice of usize. 0 is the empty space.
#[derive(PartialEq)]
struct Puzzle{
    slice: [usize; 16]
}

impl Puzzle{
    fn from(in_slice: [usize; 16]) -> Puzzle{
        Puzzle{slice: [1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,0]}
    }
    fn check_move(move: Move) -> Option<Puzzle>{
        
    }
}

//A state consists of the current board position and the cost to obtain it.
struct State{
    puzzle: Puzzle,
    cost: usize,
}

//At each state, we have up to 4 possible moves
enum Move{
    up,
    down,
    left,
    right,
}


//A move procedure is a sequence of moves which gets us to the goal
struct MoveProcedure{
   moves: Vec<Move>,
}

fn search(puzzle: Puzzle, depth: usize) -> Option<MoveProcedure>{
    None
}

// fn solve(puzzle: Puzzle, depth_limit: usize) -> Option<MoveProcedure>{
// }

fn main() {
    let p1 = ManhattanPoint::new(1, 1);
    let p2 = ManhattanPoint::new(2, 2);
    println!("manhattan dist is: {}", p1.distance(&p2)); 
}
