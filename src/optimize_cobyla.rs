
use std::cmp::min;

use argmin::core::{CostFunction, Error, Executor};
use cobyla::{fmin_cobyla, CobylaSolver, CstrFn};

use crate::{
    diet_problem::DietProblem,
    nutrient::Nutrient
};

pub fn lower(i: usize) -> usize { i * 2 }
pub fn upper(i: usize) -> usize { i * 2 + 1 }

pub fn optimize_cobyla(problem: &DietProblem) {

    let mut boxed_constraints = Vec::<Box::<dyn CstrFn>>::new();

    for (nutrient, minimum) in &problem.minima.nutrients {
        let constraint = Box::new(move |x: &[f64]| {
            let s: f64 = problem.ingredients
                .iter()
                .enumerate()
                .map(|(i, ingr)| { ingr.nutrients.get(&nutrient.clone()).unwrap_or(&0.0) * x[lower(i)] })
                .sum();
            f64::min(minimum.clone() - s, 0.0)
        });
        boxed_constraints.push(constraint);
    }

    for (nutrient, maximum) in &problem.maxima.nutrients {
        let constraint = Box::new(move |x: &[f64]| {
            let s: f64 = problem.ingredients
                .iter()
                .enumerate()
                .map(|(i, ingr)| { ingr.nutrients.get(&nutrient).unwrap_or(&0.0) * x[upper(i)] })
                .sum();
            f64::min(s - maximum, 0.0)
        });
        boxed_constraints.push(constraint);
    }

    let constraints: Vec<&dyn CstrFn> = boxed_constraints.iter().map(|b| &**b).collect();

    fn cost(x: &[f64], _data: &mut ()) -> f64 {

        // Compute volume

        let mut volume = 1.0;
        for i in 0..(x.len() / 2) {
            // x has, for each ingredient, a lower bound and an upper bound
            // volume is the product of all of those that are actually included
            if x[lower(i)] != 0.0 && x[upper(i)] != 0.0 {
                volume *= x[upper(i)] - x[lower(i)];
            }
        }

        volume
    }

    let mut x = vec![0.0; problem.ingredients.len() * 2];
    let (status, x_opt) = fmin_cobyla(cost, &mut x, &constraints, (), 0.5, 1e-4, 2000, 1);
    println!("COBYLA status {:?} x_opt {:?}", status, x_opt);

    for (i, ingredient) in problem.ingredients.iter().enumerate() {
        println!("{:20}: {:5.2} .. {:5.2}", ingredient.name, x[lower(i)], x[upper(i)]);
    }
}
