use coco_rs::{Problem, RandomState};

const POP_SIZE: i32 = 100;
const CROSSOVER_P: f64 = 0.5;
const MUTATION_P: f64 = 0.5;
const MUTATION_F: f64 = 0.1;

struct Solution {
    x: Vec<f64>,
    y: f64
}

pub fn genetic_algorithm(problem: &mut Problem, max_budget: usize, rng: &mut RandomState) {
    // problem specifics
    let dimension = problem.dimension();
    let bounds = problem.get_ranges_of_interest()[0].clone();

    // initialize population and evaluate it
    let mut population = Vec::<Solution>::new();
    for _ in 0..POP_SIZE {
        let x = &mut vec![0.0; dimension];
        let y = &mut vec![0.0];
        for idx in 0..dimension {
            x[idx] = bounds.start() + rng.uniform() * (bounds.end() - bounds.start());
        }
        problem.evaluate_function(x, y);
        population.push(Solution{
            x: x.clone(),
            y: y[0].clone()
        });
        population.sort_by(|a, b| a.y.partial_cmp(&b.y).unwrap());
    }
    
    // main for loop
    for iter in 0..max_budget {
        let mut new_population = Vec::<Solution>::new();
        // elitism
        new_population.push(Solution{
            x: population[0].x.clone(),
            y: population[0].y
        });
        println!("{}. iteration {}: {}, expected: {}", problem.name(), iter, population[0].y, problem.final_target_value());

        while new_population.len() < population.len() {
        // selection
            let mut parents = Vec::<&Solution>::new();
            for _ in 0..2 {
                let mut parents_idx = Vec::<i32>::new();
                let mut selected = Vec::<&Solution>::new();
                loop {
                    let rand_idx = (rng.uniform() * POP_SIZE as f64) as i32 % POP_SIZE;
                    if !parents_idx.contains(&rand_idx) {
                        selected.push(population.get(rand_idx as usize).unwrap());
                        parents_idx.push(rand_idx);
                    }
                    if parents_idx.len() == 3 { break; }
                }
                selected.sort_by(|a, b| a.y.partial_cmp(&b.y).unwrap());
                parents.push(selected[0]);
            }

            // crossover
            let mut child_x = parents[0].x.clone();
            if rng.uniform() < CROSSOVER_P {
                child_x = Vec::<f64>::new();
                for idx in 0..parents[0].x.len() {
                    child_x.push((parents[0].x[idx] + parents[1].x[idx]) / 2.0);
                }
            }

            // mutation
            for idx in 0..child_x.len() {
                if rng.uniform() < MUTATION_P {
                    loop {
                        let new_value = child_x[idx] + rng.normal() * MUTATION_F;
                        if new_value >= *bounds.start() && new_value <= *bounds.end() {
                            child_x[idx] = new_value;
                            break;
                        }
                    }
                }
            }

            // add to new population
            let y = &mut vec![0.0];
            problem.evaluate_function(&child_x, y);
            new_population.push(Solution{
                x: child_x.clone(),
                y: y[0].clone()
            });
        }
        // new generation
        population = new_population;
        population.sort_by(|a, b| a.y.partial_cmp(&b.y).unwrap());
    }
}