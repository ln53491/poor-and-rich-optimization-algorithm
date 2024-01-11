use coco_rs::{Problem, RandomState};

const POP_SIZE: usize = 100;
const MUTATION_P: f64 = 0.5;
const MUTATION_F: f64 = 0.1;

const HALF_POP_SIZE: usize = (POP_SIZE / 4) as usize;

struct Solution {
    x: Vec<f64>,
    y: f64
}

pub fn poor_and_rich_optimization(problem: &mut Problem, max_budget: usize, rng: &mut RandomState) {
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
        println!("{}. iteration {}: {}, expected: {}", problem.name(), iter, population[0].y, problem.final_target_value());

        // new population
        let mut new_population = Vec::<Solution>::new();

        // calculate pattern
        let best_rich = &population[0].x;
        let worst_rich = &population[HALF_POP_SIZE - 1].x;
        let mean_rich = &mut vec![0.0; dimension];
        let pattern = &mut vec![0.0; dimension];
        for idx in 0..HALF_POP_SIZE {
            for jdx in 0..dimension {
                mean_rich[jdx] += population[idx].x[jdx];
            }
        }
        for idx in 0..dimension {
            pattern[idx] = (best_rich[idx] + worst_rich[idx] + mean_rich[idx] / HALF_POP_SIZE as f64) / 3.0;
        }

        // for each member
        for (idx, curr_member) in population.iter().enumerate() {
            let rand_coef = rng.uniform();
            let mut new_member = vec![0.0; dimension];

            // update rich
            if idx < HALF_POP_SIZE {
                let best_poor = &population[HALF_POP_SIZE].x;
                for jdx in 0..dimension {
                    let new_value = rand_coef * (curr_member.x[jdx] - best_poor[jdx]);
                    new_member[jdx] =  (curr_member.x[jdx] + new_value).clamp(*bounds.start(), *bounds.end());
                }
            }
            // update poor
            else {
                for jdx in 0..dimension {
                    let new_value = rand_coef * pattern[jdx] - curr_member.x[jdx];
                    new_member[jdx] = (curr_member.x[jdx] + new_value).clamp(*bounds.start(), *bounds.end());
                }
            }
            
            // mutation
            for jdx in 0..dimension {
                if rng.uniform() < MUTATION_P {
                    loop {
                        let new_value = new_member[jdx] + rng.normal() * MUTATION_F;
                        if new_value >= *bounds.start() && new_value <= *bounds.end() {
                            new_member[jdx] = new_value;
                            break;
                        }
                    }
                }
            }

            // evaluate and select
            let y = &mut vec![0.0];
            problem.evaluate_function(&new_member, y);
            new_population.push(Solution{
                x: if y[0] < curr_member.y { new_member.clone() } else { curr_member.x.clone() },
                y: y[0].clone()
            });
        }

        // new generation
        population = new_population;
        population.sort_by(|a, b| a.y.partial_cmp(&b.y).unwrap());
    }
}