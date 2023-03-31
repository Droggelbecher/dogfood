
use cobyla::{fmin_cobyla, CstrFn};
use crate::diet_problem::DietProblem;

// Our variables are one flat vector of values,
// we interleave lower and upper rectangle sides for each of the dimensions,
// these compute the indices into the variable vector.
pub fn lower(i: usize) -> usize { i * 2 }
pub fn upper(i: usize) -> usize { i * 2 + 1 }

pub fn optimize_cobyla(problem: &DietProblem) {

    // Cobyla constraints are CstrFn (constraint functions) defined
    // as Fn(&[f64]) -> f64, in our case, closures (which each have their own unique concrete type, hence the dyn).
    // In order to make these closures live longer than the scope they were defined in (which is
    // the loop body), we need to put them in a box.
    let mut boxed_constraints = Vec::<Box::<dyn CstrFn>>::new();

    // Create a constraint for each lower nutrient bound
    for (nutrient, minimum) in &problem.minima.nutrients {

        // Make a closure that computes by how much we violate this bound
        // and put it into the box. Move in the ref to `problem`.
        let constraint = Box::new(move |x: &[f64]| {
            // Sum up over all ingredients i
            //    The amount of nutrient `nutrient` in that ingredient times the lower rectangle
            //    side of ingredient i
            let s: f64 = problem.ingredients
                .iter()
                .enumerate()
                .map(|(i, ingr)| { ingr.nutrients.get(&nutrient.clone()).unwrap_or(&0.0) * x[lower(i)] })
                .sum();

            // ... and demand it is >= the minimum
            f64::min(minimum.clone() - s, 0.0)
        });
        boxed_constraints.push(constraint);
    }

    // Analog for upper bounds
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

    // Cobyla expects a Vec<&dyn CstrFn> which we can now obtain by "unboxing" the
    // `boxed_constraints`.
    // &** is because "reference to box of CstrFn" -> "reference to CstrFn"
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

    // For each dimension/ingredient we have a left- and right box side
    let mut x = vec![0.0; problem.ingredients.len() * 2];

    // Actually run cobyla with some parameters
    let (status, x_opt) = fmin_cobyla(cost, &mut x, &constraints, (), 0.5, 1e-4, 2000, 1);
    println!("COBYLA status {:?} x_opt {:?}", status, x_opt);

    for (i, ingredient) in problem.ingredients.iter().enumerate() {
        println!("{:20}: {:5.2} .. {:5.2}", ingredient.name, x[lower(i)], x[upper(i)]);
    }
}
