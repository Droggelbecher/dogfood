
use crate::ingredient::Ingredient;

pub struct DietProblem {
    pub minima: Ingredient,
    pub maxima: Ingredient,
    pub ingredients: Vec<Ingredient>,
}

impl DietProblem {
    /// We express upper and lower bounds as pseudo-ingredients.
    /// The mental model is there is a perfect ingredient of
    /// which one gram would exactly provide the minimum/maximum nutrients we want.
    /// Since ingredients can distinguish from nutrients not being specified and being 0,
    /// we can use that to express which nutrients the boundaries care about.
    pub fn new(minima: Ingredient, maxima: Ingredient) -> Self {
        Self { minima, maxima, ingredients: Default::default() }
    }

    pub fn add(&mut self, ingredient: Ingredient) {
        self.ingredients.push(ingredient);
    }

}
