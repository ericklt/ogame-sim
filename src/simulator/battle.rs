use rand::prelude::SliceRandom;
use rand::random;
use std::collections::HashMap;

use super::units::Unit;

#[derive(Debug)]
pub struct Battle {
    starting_attackers: Vec<Unit>,
    starting_defenders: Vec<Unit>,
    current_attackers: Vec<Unit>,
    current_defenders: Vec<Unit>,
}

impl Battle {

    pub fn new(attackers: Vec<Unit>, defenders: Vec<Unit>) -> Self {
        let current_attackers = attackers.clone();
        let current_defenders = defenders.clone();
        Self { 
            starting_attackers: attackers,
            starting_defenders: defenders,
            current_attackers, 
            current_defenders, 
        }
    }

    pub fn run(&mut self) {
        for _ in 0..6 {
            if self.current_attackers.is_empty() || self.current_defenders.is_empty() { return }
            self.compute_round();
            self.round_cleanup();
            self.print_result();
            // println!("\n===\n{:#?}\n===\n", self);
        }
    }

    pub fn reset(&mut self) {
        self.current_attackers = self.starting_attackers.clone();
        self.current_defenders = self.starting_defenders.clone();
    }

    pub fn print_result(&self) {
        let mut attacker_counts = HashMap::new();
        let mut defender_counts = HashMap::new();

        for unit in &self.current_attackers {
            *attacker_counts.entry(unit.to_string()).or_insert(0) += 1;
        }
        for unit in &self.current_defenders {
            *defender_counts.entry(unit.to_string()).or_insert(0) += 1;
        }
        println!("========================================================");
        println!("{:#?}", attacker_counts);
        println!("Total: {:#?}", self.current_attackers.len());
        println!("{:#?}", defender_counts);
        println!("Total: {:#?}", self.current_defenders.len());
        println!("========================================================");
    }

    pub fn mean_results(&mut self, rounds: i32) {
        let mut attacker_counts = HashMap::new();
        let mut defender_counts = HashMap::new();

        for _ in 0..rounds {
            self.reset();
            self.run();
            for unit in &self.current_attackers {
                *attacker_counts.entry(unit.to_string()).or_insert(0.0) += 1.0 / rounds as f32;
            }
            for unit in &self.current_defenders {
                *defender_counts.entry(unit.to_string()).or_insert(0.0) += 1.0 / rounds as f32;
            }
        }

        println!("========================================================");
        println!("{:#?}", attacker_counts);
        println!("Total: {:#?}", self.current_attackers.len());
        println!("{:#?}", defender_counts);
        println!("Total: {:#?}", self.current_defenders.len());
        println!("========================================================");
    }

    fn compute_round(&mut self) {
        self.current_attackers.iter().for_each(|u| Self::compute_unit_turn(u, &mut self.current_defenders));
        self.current_defenders.iter().for_each(|u| Self::compute_unit_turn(u, &mut self.current_attackers));
    }

    fn compute_unit_turn(unit: &Unit, targets: &mut Vec<Unit>) {
        while Self::attack_random_target(unit, targets) {}
    }

    fn attack_random_target<'a>(unit: &Unit, targets: &'a mut Vec<Unit>) -> bool {
        let target = targets.choose_mut(&mut rand::thread_rng()).unwrap();
        unit.attack(target);
        let rapid_fire = unit.get_rapidfire(target) as f32;
        rapid_fire > 1.0 && random::<f32>() < (rapid_fire - 1.0) / rapid_fire
    }

    fn round_cleanup(&mut self) {
        self.current_attackers.retain(|u| !u.has_exploded());
        self.current_defenders.retain(|u| !u.has_exploded());
        self.current_attackers.iter_mut().for_each(|u| u.restore_shield());
        self.current_defenders.iter_mut().for_each(|u| u.restore_shield());
    }
}