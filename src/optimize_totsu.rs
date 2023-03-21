
use std::mem::variant_count;
use totsu::prelude::*;
use totsu;

pub fn optimize_totsu(problem: &DietProblem) {

    // 2d case
    //   (u1 - l1) * (u2 - l2)
    // = u1 * u2 - u1 * l2 - l1 * u2 + l1 * l2
    //
    // ProbQCQP
    //
    // P0 =
    //      u1 l1 u2 l2
    // u1 [        1 -1 ]
    // l1 [       -1  1 ]
    // u2 [  1 -1       ]
    // l2 [ -1  1       ]

    // For each nutrient we track a lower bound (l)
    // and an upper bound (u) of our rectangle,
    // so there is twice as many variables as nutrients
    let mut p0 = AMatBuild::new(MatType::SymPack(
            2 * variant_count<Nutrient>()
    ));

    // TODO:
    // Problem: real case is much higher dimensional than 2.
    // Would need to read into convex optimization
    // to allow this as totsu doesnt seem to export anything
    // higher than quadratic constraints?
    //
    // (u1 - l1) (u2 - l2) (u3 - l3) ...
    //
    // Seems we need to define a coune ourselves by implementing
    // https://docs.rs/totsu_core/0.1.1/totsu_core/solver/trait.Cone.html
    //
    // Honestly this leads deeper into convex optimization than I care for atm...

    for (nutrient, limit) in problem.minima {

    }



    for ingredient in problem.ingredients.iter() {
    }

}
