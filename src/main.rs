mod simulatedAnnealing;

use simulatedAnnealing::Solver;

extern crate gnuplot;



fn main() {

    use gnuplot::{Figure, Caption, Color, PointSymbol, ContourStyle, AutoOption};
	

    let mut solver = Solver::new(100000000000000.0, 0.1); 
    let solution = solver.solve();


    let mut stateX: Vec<f64> = Vec::new();
    let mut stateY: Vec<f64> = Vec::new();
    let mut stateZ: Vec<f64> = Vec::new();

    for state in solver.state_hist.into_iter() {
        stateX.push(state[0]);
        stateY.push(state[1]);
        stateZ.push(Solver::search_area(state));
    }
	
	let x = [solution[0]];
	let y = [solution[1]];
	let z = [Solver::search_area(solution)];

	let mut mat: [f64; 400] = [0.0; 400];
	for i in 0..20 {
		for j in 0..20 {
			mat[i * j] = Solver::search_area([i as f64 - 10.0, j as f64 - 10.0]);	
		}
	}

	let mut figure = Figure::new();
	figure
		.axes3d()
		.surface(
            mat.iter(), 20, 20, Some((-10.0, -10.0, 10.0, 10.0)), &[Caption("Surface")])
		.show_contours(
            true, false, ContourStyle::Linear, AutoOption::Auto, AutoOption::Auto)
		.points(&stateX, &stateY, &stateZ, &[Color("green"), PointSymbol('S')])
		.points(&x, &y, &z, &[Color("blue"), PointSymbol('S')]);
	figure.show();

	println!("{}", Solver::search_area(solution));
    println!("{:?}", solution);
}
