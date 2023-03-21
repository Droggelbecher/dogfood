
#[macro_use]
mod ingredient;

mod nutrient;
mod diet_problem;
mod optimize_good_lp;

use crate::{
    nutrient::Nutrient,
    ingredient::Ingredient,
    diet_problem::DietProblem,
    optimize_good_lp::optimize_good_lp
};

fn main() {

    // /!\ DISCLAIMER /!\
    // All nutritional values below are completely made up.
    // Do not use any of this as nutrional advice, instead
    // consult a vet for info on a balanced diet.

    let minima = ingredient!{ (min) Energy: 1000.0, Protein: 100.0, Calcium: 1.0 };
    let maxima = ingredient!{ (max) Energy: 1100.0, Protein: 200.0, Calcium: 2.0 };

    let mut d = DietProblem::new(minima, maxima);

    // Potentially questionable use of macros here,
    // but it makes the `Ingredient` construction quite concise and readable.

    d.add(ingredient!{ (GobblerMeat)   Energy: 1.25, Protein: 0.28 });
    d.add(ingredient!{ (BulbFruit)     Energy: 1.0 });
    d.add(ingredient!{ (FrobnizerBone) Energy: 1.0, Calcium: 0.01 });

    optimize_good_lp(&d);
}

/*
use std::collections::HashMap;

use good_lp::variable::ProblemVariables;
use good_lp::{constraint, default_solver, Solution, SolverModel, variables, Expression, variable, Variable};

struct DietProblem {
    variables: ProblemVariables,
    minima: Ingredient,
    maxima: Ingredient,
    nutrient_sums: HashMap<Nutrient, Expression>,
}

impl DietProblem {
    /// We express upper and lower bounds as pseudo-ingredients.
    /// The mental model is there is a perfect ingredient of
    /// which one gram would exactly provide the minimum/maximum nutrients we want.
    /// Since ingredients can distinguish from nutrients not being specified and being 0,
    /// we can use that to express which nutrients the boundaries care about.
    fn new(minima: Ingredient, maxima: Ingredient) -> Self {
        DietProblem {
            variables: variables!(),
            minima, maxima,
            nutrient_sums: HashMap::<Nutrient, Expression>::default(),
        }
    }

    fn add(&mut self, ingredient: Ingredient) -> Variable {
        let amount = self.variables.add(variable().min(0));
        for (nutrient, contents) in ingredient.0 {
            *self.nutrient_sums.entry(nutrient).or_default() += amount * contents;
        }

        amount
    }

    fn optimize(self) -> (impl Solution, HashMap<Nutrient, Expression>) {
        let mut pb = self.variables
            .maximise(0)
            .using(default_solver);

        for (nutrient, contents) in self.minima.0 {
            pb.add_constraint(constraint!(self.nutrient_sums.get(&nutrient).unwrap().clone() >= contents));
        }
        for (nutrient, contents) in self.maxima.0 {
            pb.add_constraint(constraint!(self.nutrient_sums.get(&nutrient).unwrap().clone() <= contents));
        }

        (pb.solve().unwrap(), self.nutrient_sums)
    }
}

fn main() {

    let mut problem = DietProblem::new(
        ingredient!{ Energy: 100.0, Protein: 20.0 },
        ingredient!{ Energy: 130.0 },
    );

    let beef = ingredient!{ Energy: 1.25, Protein: 0.21 };

    let beef_amount = problem.add(beef);

    let (solution, nutrient_sums) = problem.optimize();

    println!();
    println!("Beef: {}", solution.value(beef_amount));
    println!();
    for (nutrient, sum) in nutrient_sums {
        println!("{:?}: {}", nutrient, solution.eval(sum));
    }
}
*/
