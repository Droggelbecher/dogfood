
use std::collections::HashMap;

//use good_lp::variable::ProblemVariables;
use good_lp::{constraint, default_solver, Solution, SolverModel, variables, Expression, variable, Variable};

use crate::{
    diet_problem::DietProblem,
    nutrient::Nutrient
};

pub fn optimize_good_lp(problem: &DietProblem) {

    // Good LP variables. These are the values we want to optimize,
    // in our case amounts of ingredients.
    let mut variables = variables!();
    let mut amounts = HashMap::<String, Variable>::new();

    // Good LP expressions that express how much of each nutrient
    // our solutions will have.
    let mut nutrient_sums = HashMap::<Nutrient, Expression>::new();

    for ingredient in problem.ingredients.iter() {
        // Create variable tracking how much of this ingredient we'll use
        let amount = variables.add(variable().min(0));
        amounts.insert(ingredient.name.clone(), amount);

        // Extend nutrient expressions
        for (nutrient, contents) in ingredient.nutrients.iter() {
            *nutrient_sums.entry(*nutrient).or_default() += amount * *contents;
        }
    }

    // Here: Maximize "0" (i.e. just find any solution that obeys the constraints).
    // This could instead be minimizing the cost of the ingredients or
    // something along those lines

    let mut lp = variables.maximise(0).using(default_solver);

    // Add constraints: We want at least all the nutrients in
    // the `problem.minima` "ingredient"
    // and most those in `problem.maxima`.

    for (nutrient, contents) in problem.minima.nutrients.iter() {
        lp.add_constraint(
            constraint!(nutrient_sums[&nutrient].clone() >= *contents)
        );
    }

    for (nutrient, contents) in problem.maxima.nutrients.iter() {
        lp.add_constraint(
            constraint!(nutrient_sums[&nutrient].clone() <= *contents)
        );
    }

    // Actual work happens here:

    let solution = lp.solve().unwrap();

    for ingredient in problem.ingredients.iter() {
        println!("{:?}: {}", ingredient.name, solution.value(amounts[&ingredient.name]));
    }

    for (nutrient, sum) in nutrient_sums {
        println!("{:?}: {}", nutrient, solution.eval(sum));
    }
}
