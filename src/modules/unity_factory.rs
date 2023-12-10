use rand::prelude::SliceRandom;

use super::units::{Unit, UnitStats};

pub struct UnitFactory {}

impl UnitFactory {
    pub fn light_fighter() -> Unit { 
        Unit::LightFighter(UnitStats::new(400, 10, 50))
    }

    pub fn heavy_fighter() -> Unit { 
        Unit::HeavyFighter(UnitStats::new(1000, 25, 150))
    }

    pub fn cruiser() -> Unit { 
        Unit::Cruiser(UnitStats::new(2700, 50, 400))
    }

    pub fn random_fleet(size: u32) -> Vec<Unit> {
        (0..size).map(|_| Self::random_unit()).collect()
    }

    fn random_unit() -> Unit {
        let possible: Vec<fn() -> Unit> = vec![
            Self::light_fighter, 
            Self::heavy_fighter,
            Self::cruiser,
        ];
        let mut rng = rand::thread_rng();
        possible.choose(&mut rng).unwrap()()
    }
}