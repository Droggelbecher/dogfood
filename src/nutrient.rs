
// We could've also added a lot of fields to the Ingredient struct
// instead, but the enum approach allows us to do things like iterating
// over all nutrients much easier.

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
pub enum Nutrient {
    Energy,
    Protein,
    Calcium,
    // ...
}

