
#[macro_use]
mod ingredient;

mod nutrient;
mod diet_problem;
mod optimize_good_lp;
mod optimize_cobyla;

use crate::{
    nutrient::Nutrient,
    ingredient::Ingredient,
    diet_problem::DietProblem,
    optimize_good_lp::optimize_good_lp,
    optimize_cobyla::optimize_cobyla
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

    optimize_cobyla(&d);
}

