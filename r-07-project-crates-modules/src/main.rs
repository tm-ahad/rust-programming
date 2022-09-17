
use crate::meal::HealthyFood;

pub fn cook_healthy_dinner(emoji: char) -> Option<HealthyFood>
{
    println!("{}", "Cooking 😋😋!");

    match emoji
    {
        '🥪' => return Option::Some(HealthyFood::VegetableSandwich),
        '🐟' => return Option::Some(HealthyFood::Salmon),
        '🌮' => return Option::Some(HealthyFood::Taco),
        '🍱' => return Option::Some(HealthyFood::LentilsWithFihAndRiceVegetable),
        _ => None
    }
}

pub mod meal {

    #[derive(Debug)]
    pub enum HealthyFood
    {
        VegetableSandwich,
        Taco,
        LentilsWithFihAndRiceVegetable,
        Salmon
    }

    pub enum Dinner {
        FirstFood,
        HealthyFood
    }
}

fn main() {
    let meal: Option<HealthyFood> = cook_healthy_dinner('🍱');

    println!("{:?}", meal)
}