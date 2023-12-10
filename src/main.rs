mod modules;

use crate::modules::{battle::Battle, unity_factory::UnitFactory};

fn main() {
    let attackers = UnitFactory::random_fleet(5);
    let defenders = UnitFactory::random_fleet(5);

    let mut battle = Battle::new(attackers, defenders);
    battle.print_result();
    battle.mean_results(2000);
}
