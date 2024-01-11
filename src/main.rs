mod ga;
mod pro;
use coco_rs::{LogLevel, Observer, ObserverName, RandomState, Suite, SuiteName};

const BUDGET_MULTIPLIER: usize = 10;
const INDEPENDENT_RESTARTS: u64 = 1 as u64;
const RANDOM_SEED: u32 = 0xdeadbeef;

fn main() {
    let random_generator = &mut RandomState::new(RANDOM_SEED);
    println!("Running the experiment... (might take time, be patient)");
    coco_rs::set_log_level(LogLevel::Info);
    example_experiment(
        SuiteName::Bbob,
        "",
        ObserverName::Bbob,
        "result_folder: PRO_on_bbob",
        random_generator,
    );
    println!("Done!");
}

fn example_experiment(
    suite_name: SuiteName,
    suite_options: &str,
    observer_name: ObserverName,
    observer_options: &str,
    random_generator: &mut RandomState,
) {
    let suite = &mut Suite::new(suite_name, "", suite_options).unwrap();
    let observer = &mut Observer::new(observer_name, observer_options).unwrap();

    while let Some(problem) = &mut suite.next_problem(Some(observer)) {
        let dimension = problem.dimension();

        for _ in 1..=INDEPENDENT_RESTARTS {
            let evaluations_done = problem.evaluations() + problem.evaluations_constraints();
            let evaluations_remaining =
                (dimension * BUDGET_MULTIPLIER).saturating_sub(evaluations_done as usize);

            if problem.final_target_hit() || evaluations_remaining == 0 {
                break;
            }

            // ga::genetic_algorithm(problem, evaluations_remaining, random_generator);
            pro::poor_and_rich_optimization(problem, evaluations_remaining, random_generator);
        }
    }
}