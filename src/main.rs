mod simulatedAnnealing;

use simulatedAnnealing::Solver;

fn main() {
    let mut solver = Solver::new(100000000000000.0, 0.1); 
    let solution = solver.solve();
    println!("{}", Solver::search_area(solution));
    println!("{:?}", solution);
}
