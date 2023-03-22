
use std::collections::HashMap;

use crate::nutrient::Nutrient;

#[derive(Default)]
pub struct Ingredient {
    pub name: String,
    pub nutrients: HashMap<Nutrient, f64>,
}

impl Ingredient {
    pub fn new(name: String) -> Self {
        Ingredient { name, ..Default::default() }
    }
}

// https://danielkeep.github.io/tlborm/book/README.html
macro_rules! ingredient {
    (($name:ident) $($nut_name:ident : $nut_value:expr),*) => {
        {
            let mut x = Ingredient::new(stringify!($name).into());
            $(
                x.nutrients.insert(Nutrient::$nut_name, $nut_value);
            )*
            x
        }
    }
}

