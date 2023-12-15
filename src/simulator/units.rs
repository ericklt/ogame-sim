use rand::random;

#[derive(Debug)]
pub struct BattleTechs {
    atk: i8,
    shield: i8,
    hull: i8,
}

impl BattleTechs {
    pub fn new(atk: i8, shield: i8, hull: i8) -> Self {
        Self { atk, shield, hull }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct UnitStats {
    base_hull: i32,
    base_shield: i32,
    base_atk: i32,

    hull: i32,
    shield: i32,
    survival_chance: f64,
}

impl UnitStats {

    pub fn new(hull: i32, shield: i32, atk: i32) -> Self {
        Self { base_hull: hull, base_shield: shield, base_atk: atk, hull, shield, survival_chance: 1.0 }
    }

    pub fn apply_techs(&self, techs: &BattleTechs) -> Self {
        fn apply_tech(tech_level: i8, value: i32) -> i32 {
            ((1.0 + (tech_level as f32) / 10.0) * value as f32) as i32
        }
        Self::new(
            apply_tech(techs.hull, self.base_hull), 
            apply_tech(techs.shield, self.base_shield), 
            apply_tech(techs.atk, self.base_atk),
        )
    }

    pub(super) fn receive_damage(&mut self, dmg: i32) {
        let rounded_dmg = if dmg > self.shield { dmg } else {
            let percentage_dmg_to_shield = (100 * dmg) / self.base_shield;
            (self.base_shield * percentage_dmg_to_shield) / 100
        };

        if rounded_dmg <= 0 {
            return
        }

        self.hull -= (rounded_dmg - self.shield).max(0);
        self.shield = (self.shield - rounded_dmg).max(0);

        if self.survival_chance > 1e-9 {
            let remaining_hull = (self.hull.max(0) as f64) / (self.base_hull as f64);
            self.survival_chance *= if remaining_hull >= 0.7 { 1.0 } else { remaining_hull };
        }
    }

    pub fn round_reset(&mut self) {
        self.shield = self.base_shield;
        self.survival_chance = 1.0;
    }

}

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
pub enum UnitKind {
    LightFighter,
    HeavyFighter,
    Cruiser,
    Battleship,
    Battlecruiser,
    Bomber,
    Destroyer,
    Deathstar,
    Reaper,
    Pathfinder,

    RocketLauncher,
    LightLaser,
    HeavyLaser,
    GaussCannon,
    IonCannon,
    PlasmaTurret,

    SmallShieldDome,
    LargeShieldDome,

    SmallCargo,
    LargeCargo,
    ColonyShip,
    Recycler,
    EspionageProbe,
    SolarSatellite,
    Crawler,
}


#[derive(Debug, Clone)]
pub struct Unit {
    kind: UnitKind,
    stats: UnitStats,
}

impl Unit {
    pub fn new(kind: UnitKind, stats: UnitStats) -> Self {
        Unit { kind, stats }
    }

    pub fn attack(&self, unit: &mut Unit) { 
        unit.receive_damage(self.stats.base_atk) 
    }

    pub fn round_reset(&mut self) {
        self.stats.round_reset();
    }

    pub fn survives(&self, explosion_threshold: f64) -> bool {
        explosion_threshold < self.stats.survival_chance
    }

    pub fn get_rapidfire(&self, other: &Unit) -> i32 {
        match self.kind {
            UnitKind::LightFighter => match other.kind {
                UnitKind::EspionageProbe => 5,
                UnitKind::SolarSatellite => 5,
                UnitKind::Crawler => 5,
                _ => 1,
            }
            UnitKind::HeavyFighter => match other.kind {
                UnitKind::EspionageProbe => 5,
                UnitKind::SolarSatellite => 5,
                UnitKind::Crawler => 5,
                UnitKind::SmallCargo => 3,
                _ => 1,
            },
            UnitKind::Cruiser => match other.kind {
                UnitKind::EspionageProbe => 5,
                UnitKind::SolarSatellite => 5,
                UnitKind::Crawler => 5,
                UnitKind::LightFighter => 6,
                UnitKind::RocketLauncher => 10,
                _ => 1,
            },
            UnitKind::Battleship => match other.kind {
                UnitKind::EspionageProbe => 5,
                UnitKind::SolarSatellite => 5,
                UnitKind::Crawler => 5,
                UnitKind::Pathfinder => 5,
                _ => 1,
            },
            UnitKind::Battlecruiser => match other.kind {
                UnitKind::EspionageProbe => 5,
                UnitKind::SolarSatellite => 5,
                UnitKind::Crawler => 5,
                UnitKind::HeavyFighter => 4,
                UnitKind::Cruiser => 4,
                UnitKind::Battleship => 7,
                UnitKind::SmallCargo => 3,
                UnitKind::LargeCargo => 3,
                _ => 1,
            },
            UnitKind::Bomber => match other.kind {
                UnitKind::EspionageProbe => 5,
                UnitKind::SolarSatellite => 5,
                UnitKind::Crawler => 5,
                UnitKind::RocketLauncher => 20,
                UnitKind::LightLaser => 20,
                UnitKind::HeavyLaser => 10,
                UnitKind::IonCannon => 10,
                UnitKind::GaussCannon => 5,
                UnitKind::PlasmaTurret => 5,
                _ => 1,
            },
            UnitKind::Destroyer => match other.kind {
                UnitKind::EspionageProbe => 5,
                UnitKind::SolarSatellite => 5,
                UnitKind::Crawler => 5,
                UnitKind::Battlecruiser => 2,
                UnitKind::LightLaser => 10,
                _ => 1,
            },
            UnitKind::Deathstar => match other.kind {
                UnitKind::EspionageProbe => 250,
                UnitKind::SolarSatellite => 250,
                UnitKind::Crawler => 250,
                UnitKind::LightFighter => 200,
                UnitKind::HeavyFighter => 100,
                UnitKind::Cruiser => 33,
                UnitKind::Battleship => 30,
                UnitKind::Battlecruiser => 15,
                UnitKind::Bomber => 25,
                UnitKind::Destroyer => 5,
                UnitKind::Reaper => 10,
                UnitKind::SmallCargo => 250,
                UnitKind::LargeCargo => 250,
                UnitKind::ColonyShip => 250,
                UnitKind::Recycler => 250,
                UnitKind::RocketLauncher => 200,
                UnitKind::LightLaser => 200,
                UnitKind::HeavyLaser => 100,
                UnitKind::IonCannon => 100,
                UnitKind::GaussCannon => 50,
                UnitKind::Pathfinder => 30,
                _ => 1,
            },
            UnitKind::Reaper => match other.kind {
                UnitKind::EspionageProbe => 5,
                UnitKind::SolarSatellite => 5,
                UnitKind::Crawler => 5,
                UnitKind::Battleship => 7,
                UnitKind::Bomber => 4,
                UnitKind::Destroyer => 3,
                _ => 1,
            },
            UnitKind::Pathfinder => match other.kind {
                UnitKind::EspionageProbe => 5,
                UnitKind::SolarSatellite => 5,
                UnitKind::Crawler => 5,
                UnitKind::LightFighter => 3,
                UnitKind::HeavyFighter => 2,
                UnitKind::Cruiser => 3,
                _ => 1,
            },
            
            UnitKind::IonCannon => match other.kind {
                UnitKind::Reaper => 2,
                _ => 1,
            },

            UnitKind::SmallCargo | UnitKind::LargeCargo | UnitKind::ColonyShip | UnitKind::Recycler => match other.kind {
                UnitKind::EspionageProbe => 5,
                UnitKind::SolarSatellite => 5,
                UnitKind::Crawler => 5,
                _ => 1,
            }
            
            _ => 1,
        }
    }
    
    fn receive_damage(&mut self, dmg: i32) {
        self.stats.receive_damage(dmg)
    }
}

impl ToString for Unit {
    fn to_string(&self) -> String {
        match self.kind {
            UnitKind::LightFighter => String::from("LightFighter"),
            UnitKind::HeavyFighter => String::from("HeavyFighter"),
            UnitKind::Cruiser => String::from("Cruiser"),
            UnitKind::Battleship => String::from("Battleship"),
            UnitKind::Battlecruiser => String::from("Battlecruiser"),
            UnitKind::Bomber => String::from("Bomber"),
            UnitKind::Destroyer => String::from("Destroyer"),
            UnitKind::Deathstar => String::from("Deathstar"),
            UnitKind::Reaper => String::from("Reaper"),

            UnitKind::RocketLauncher => String::from("RocketLauncher"),
            UnitKind::LightLaser => String::from("LightLaser"),
            UnitKind::HeavyLaser => String::from("HeavyLaser"),
            UnitKind::GaussCannon => String::from("GaussCannon"),
            UnitKind::IonCannon => String::from("IonCannon"),
            UnitKind::PlasmaTurret => String::from("PlasmaTurret"),
            UnitKind::SmallShieldDome => String::from("SmallShieldDome"),
            UnitKind::LargeShieldDome => String::from("LargeShieldDome"),

            UnitKind::SmallCargo => String::from("SmallCargo"),
            UnitKind::LargeCargo => String::from("LargeCargo"),
            UnitKind::ColonyShip => String::from("ColonyShip"),
            UnitKind::Recycler => String::from("Recycler"),
            UnitKind::EspionageProbe => String::from("EspionageProbe"),
            UnitKind::Pathfinder => String::from("Pathfinder"),
            UnitKind::SolarSatellite => String::from("SolarSatellite"),
            UnitKind::Crawler => String::from("Crawler"),
        }
    }
}