use std::collections::HashMap;
use modules::battle::Battle;
use modules::units::UnitKind;
use modules::unity_factory::UnitFactory;

pub trait Examples {}

impl Examples {
    pub fn run_battle_example() {
        let attacker_factory = UnitFactory::new(20, 19, 20);
        let defender_factory = UnitFactory::new(14, 14, 15);

        let attackers = attacker_factory.from_map(HashMap::from([
            (UnitKind::Battleship, 15000),
            (UnitKind::Battlecruiser, 10000),
        ]));

        let defenders = defender_factory.from_map(HashMap::from([
            (UnitKind::LightFighter, 147),
            (UnitKind::Cruiser, 441),
            (UnitKind::Battleship, 60),
            (UnitKind::Pathfinder, 1429),
            (UnitKind::SmallCargo, 11479),
            (UnitKind::LargeCargo, 2981),
            (UnitKind::Recycler, 5),
            (UnitKind::EspionageProbe, 6717),
            (UnitKind::SolarSatellite, 30),
            (UnitKind::RocketLauncher, 50311),
            (UnitKind::LightLaser, 10007),
            (UnitKind::GaussCannon, 500),
            (UnitKind::PlasmaTurret, 46),
            (UnitKind::SmallShieldDome, 1),
            (UnitKind::LargeShieldDome, 1),
        ]));

        let mut battle = Battle::new(attackers, defenders);
        battle.print_result();
        // battle.mean_results(500);
        battle.run();
        battle.print_result();
    }
}