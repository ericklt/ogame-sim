use rand::random;

#[derive(Debug)]
#[derive(Clone)]
#[derive(Copy)]
pub struct UnitStats {
    base_hull: i32,
    base_shield: i32,
    base_atk: i32,

    hull: i32,
    shield: i32,
    exploded: bool,
}

impl UnitStats {

    pub fn new(hull: i32, shield: i32, atk: i32) -> Self {
        Self { base_hull: hull, base_shield: shield, base_atk: atk, hull, shield, exploded: false }
    }

    pub(super) fn receive_damage(&mut self, dmg: i32) {
        if dmg * 100 < self.shield {
            return
        }

        self.hull -= (dmg - self.shield).max(0);
        self.shield = (self.shield - dmg).max(0);

        self.check_explosion();
    }

    fn check_explosion(&mut self) {
        let current = self.hull as f32;
        let initial_hull = self.base_hull as f32;
        if current < initial_hull * 0.7 {
            let explosion_prob = 1.0 - current / initial_hull;
            if random::<f32>() < explosion_prob {
                self.exploded = true;
            }
        }
    }
}

#[derive(Debug)]
#[derive(Clone)]
#[derive(Copy)]
pub enum Unit {
    LightFighter(UnitStats),
    HeavyFighter(UnitStats),
    Cruiser(UnitStats),
    Battleship(UnitStats),
    Battlecruiser(UnitStats),
    Bomber(UnitStats),
    Destroyer(UnitStats),
    Deathstar(UnitStats),
    Reaper(UnitStats),
}

impl Unit {
    

    pub fn has_exploded(&self) -> bool {
        match self {
            Unit::LightFighter(s) |
            Unit::HeavyFighter(s) |
            Unit::Cruiser(s) |
            Unit::Battleship(s) |
            Unit::Battlecruiser(s) |
            Unit::Bomber(s) |
            Unit::Destroyer(s) |
            Unit::Deathstar(s) |
            Unit::Reaper(s) => 
            
            s.exploded
        }
    }

    pub fn attack(&self, unit: &mut Unit) {
        match self {
            Unit::LightFighter(s) |
            Unit::HeavyFighter(s) |
            Unit::Cruiser(s) |
            Unit::Battleship(s) |
            Unit::Battlecruiser(s) |
            Unit::Bomber(s) |
            Unit::Destroyer(s) |
            Unit::Deathstar(s) |
            Unit::Reaper(s) => 

            unit.receive_damage(s.base_atk)
        }
    }

    pub fn restore_shield(&mut self) {
        match self {
            Unit::LightFighter(s) |
            Unit::HeavyFighter(s) |
            Unit::Cruiser(s) |
            Unit::Battleship(s) |
            Unit::Battlecruiser(s) |
            Unit::Bomber(s) |
            Unit::Destroyer(s) |
            Unit::Deathstar(s) |
            Unit::Reaper(s) => 
            
            s.shield = s.base_shield
        }
    }

    pub fn get_rapidfire(&self, other: Unit) -> i32 {
        match self {
            Self::LightFighter(_) | Self::HeavyFighter(_) => 1,
            Self::Cruiser(_) => match other {
                Self::LightFighter(_) => 6,
                _ => 1,
            },
            Unit::Battleship(_) => 1,
            Unit::Battlecruiser(_) => match other {
                Self::HeavyFighter(_) => 4,
                Self::Cruiser(_) => 4,
                Self::Battleship(_) => 7,
                _ => 1,
            },
            Unit::Bomber(_) => 1,
            Unit::Destroyer(_) => match other {
                Self::Battlecruiser(_) => 2,
                _ => 1,
            },
            Unit::Deathstar(_) => match other {
                Unit::LightFighter(_) => 200,
                Unit::HeavyFighter(_) => 100,
                Unit::Cruiser(_) => 33,
                Unit::Battleship(_) => 30,
                Unit::Battlecruiser(_) => 15,
                Unit::Bomber(_) => 25,
                Unit::Destroyer(_) => 5,
                Unit::Deathstar(_) => 1,
                Unit::Reaper(_) => 10,
            },
            Unit::Reaper(_) => match other {
                Self::Battleship(_) => 7,
                Self::Bomber(_) => 4,
                Self::Destroyer(_) => 3,
                _ => 1,
            },

        }
    }
    
    fn receive_damage(&mut self, dmg: i32) {
        match self {
            Unit::LightFighter(s) |
            Unit::HeavyFighter(s) |
            Unit::Cruiser(s) |
            Unit::Battleship(s) |
            Unit::Battlecruiser(s) |
            Unit::Bomber(s) |
            Unit::Destroyer(s) |
            Unit::Deathstar(s) |
            Unit::Reaper(s) => 
            
            s.receive_damage(dmg)
        }
    }
}

impl ToString for Unit {
    fn to_string(&self) -> String {
        match self {
            Unit::LightFighter(_) => String::from("LightFighter"),
            Unit::HeavyFighter(_) => String::from("HeavyFighter"),
            Unit::Cruiser(_) => String::from("Cruiser"),
            Unit::Battleship(_) => String::from("Battleship"),
            Unit::Battlecruiser(_) => String::from("Battlecruiser"),
            Unit::Bomber(_) => String::from("Bomber"),
            Unit::Destroyer(_) => String::from("Destroyer"),
            Unit::Deathstar(_) => String::from("Deathstar"),
            Unit::Reaper(_) => String::from("Reaper"),
        }
    }
}