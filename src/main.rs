use std::f64::consts::E;

struct Solver {
    pub iterations: u64,
    pub initial_temperature: f64,
    pub temperature_cooling_rate: f64,
}

impl Solver {
    pub fn search_area(x: [f64; 2]) -> f64 {
        x[0].sin() + x[1].cos() + 
            (0.8 * E.powf(1.0 - (x[0].powi(2) + x[1].powi(2)).sqrt()))
    }
    
    pub fn generate_state() -> [f64; 2] {
        [1.0, 1.0]
    }

    pub fn mutate_state(state: [f64; 2]) -> [f64; 2] {
        state
    }

    pub fn random_trial(dE: f64, temperature: f64) -> bool {
        true
    }

    pub fn solve(&mut self) -> [f64; 2] {
        let mut current_state = Solver::generate_state();

        let mut  temperature = self.initial_temperature;
        loop {
            let new_state = Solver::mutate_state(current_state);
            let dE = 
                Solver::search_area(current_state) - Solver::search_area(new_state) ;
            if  dE > 0.0 {
                current_state = new_state;
            } else {
                if Solver::random_trial(dE, temperature) {
                    current_state = new_state; 
                }
            }
            temperature = temperature * self.temperature_cooling_rate;
            if temperature == 0.0 { break; }
        }

        current_state
    }
}

fn main() {
    let mut solver = Solver {
        iterations: 0,
        initial_temperature: 10000.0,
        temperature_cooling_rate: 0.1
    };
    println!("{:?}", solver.solve());
}
