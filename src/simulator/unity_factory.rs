use std::collections::HashMap;

use super::units::{Unit, UnitStats, BattleTechs, UnitKind};

#[derive(Debug)]
pub struct UnitFactory {
    techs: BattleTechs,
}

impl UnitFactory {
    
    pub fn new(atk: i8, shield: i8, hull: i8) -> Self {
        Self { techs: BattleTechs::new(atk, shield, hull) }
    }

    pub fn from_map(&self, kind_map: HashMap<UnitKind, i32>) -> Vec<Unit> {
        let mut units: Vec<Unit> = Vec::new();
        for (&kind, &n) in kind_map.iter() {
            for _ in 0..n {
                units.push(self.create(kind))
            }
        }
        units
    }

    pub fn create(&self, kind: UnitKind) -> Unit {
        let base_stats = match kind {
            UnitKind::LightFighter => UnitStats::new(400, 10, 50),
            UnitKind::HeavyFighter => UnitStats::new(1_000, 25, 150),
            UnitKind::Cruiser => UnitStats::new(2_700, 50, 400),
            UnitKind::Battleship => UnitStats::new(6_000, 200, 1_000),
            UnitKind::Battlecruiser => UnitStats::new(7_000, 400, 700),
            UnitKind::Bomber => UnitStats::new(7_500, 500, 1_000),
            UnitKind::Destroyer => UnitStats::new(11_000, 500, 2_000),
            UnitKind::Deathstar => UnitStats::new(2_340_000, 50_000, 200_000),
            UnitKind::Reaper => UnitStats::new(14_000, 700, 2_800),
            UnitKind::Pathfinder => UnitStats::new(2_300, 100, 200),

            UnitKind::RocketLauncher => UnitStats::new(200, 20, 80),
            UnitKind::LightLaser => UnitStats::new(200, 25, 100),
            UnitKind::HeavyLaser => UnitStats::new(800, 100, 250),
            UnitKind::GaussCannon => UnitStats::new(3_500, 200, 1_100),
            UnitKind::IonCannon => UnitStats::new(800, 500, 150),
            UnitKind::PlasmaTurret => UnitStats::new(10_000, 300, 3_000),

            UnitKind::SmallShieldDome => UnitStats::new(2_000, 2_000, 1),
            UnitKind::LargeShieldDome => UnitStats::new(10_000, 10_000, 1),

            UnitKind::SmallCargo => UnitStats::new(400, 10, 5),
            UnitKind::LargeCargo => UnitStats::new(1_200, 25, 5),
            UnitKind::ColonyShip => UnitStats::new(3_000, 100, 50),
            UnitKind::Recycler => UnitStats::new(1_600, 10, 1),
            UnitKind::EspionageProbe => UnitStats::new(100, 0, 0),
            UnitKind::SolarSatellite => UnitStats::new(200, 1, 1),
            UnitKind::Crawler => UnitStats::new(400, 1, 1),
        };
        Unit::new(kind, base_stats.apply_techs(&self.techs))
    }
}