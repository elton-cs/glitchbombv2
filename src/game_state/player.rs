use bevy::prelude::*;
use rand::Rng;
use super::orb::Orb;

#[derive(Resource)]
pub struct PlayerGameState {
    pub health: u32,
    pub points: u32,
    pub game_id: u32,
    pub milestone: u32,
    pub orbs: Vec<Orb>,
    pub level: u32,
    pub moonrocks: u32,
    pub cheddah: u32,
}

impl Default for PlayerGameState {
    fn default() -> Self {
        Self::new_for_level(1)
    }
}

#[allow(dead_code)]
impl PlayerGameState {
    pub fn new_for_level(level: u32) -> Self {
        let orbs_per_type = 5 * 2_u32.pow(level.saturating_sub(1));
        let mut orbs = Vec::new();
        
        for _ in 0..orbs_per_type {
            orbs.push(Orb::Health);
            orbs.push(Orb::Point);
            orbs.push(Orb::Bomb);
        }
        
        let milestone = 5 + (level * 10);
        
        Self {
            health: 5,
            points: 0,
            game_id: 1,
            milestone,
            orbs,
            level,
            moonrocks: 0,
            cheddah: 0,
        }
    }
    pub fn health(&self) -> u32 { self.health }
    pub fn points(&self) -> u32 { self.points }
    pub fn game_id(&self) -> u32 { self.game_id }
    pub fn milestone(&self) -> u32 { self.milestone }
    pub fn orbs(&self) -> &Vec<Orb> { &self.orbs }
    pub fn level(&self) -> u32 { self.level }
    pub fn moonrocks(&self) -> u32 { self.moonrocks }
    pub fn cheddah(&self) -> u32 { self.cheddah }

    pub fn set_health(&mut self, value: u32) { self.health = value; }
    pub fn set_points(&mut self, value: u32) { self.points = value; }
    pub fn set_game_id(&mut self, value: u32) { self.game_id = value; }
    pub fn set_milestone(&mut self, value: u32) { self.milestone = value; }
    pub fn set_orbs(&mut self, orbs: Vec<Orb>) { self.orbs = orbs; }
    pub fn set_level(&mut self, value: u32) { self.level = value; }
    pub fn set_moonrocks(&mut self, value: u32) { self.moonrocks = value; }
    pub fn set_cheddah(&mut self, value: u32) { self.cheddah = value; }

    pub fn add_health(&mut self, amount: u32) { 
        self.health = (self.health + amount).min(5); 
    }
    pub fn add_points(&mut self, amount: u32) { self.points += amount; }
    pub fn add_orb(&mut self, orb: Orb) { self.orbs.push(orb); }
    pub fn add_moonrocks(&mut self, amount: u32) { self.moonrocks += amount; }
    pub fn add_cheddah(&mut self, amount: u32) { self.cheddah += amount; }
    
    pub fn increase_milestone(&mut self) { self.milestone += 1; }
    pub fn level_up(&mut self) { self.level += 1; }

    pub fn subtract_health(&mut self, amount: u32) { 
        self.health = self.health.saturating_sub(amount); 
    }
    pub fn subtract_points(&mut self, amount: u32) { 
        self.points = self.points.saturating_sub(amount); 
    }
    pub fn remove_orb(&mut self, orb_type: Orb) -> bool {
        if let Some(pos) = self.orbs.iter().position(|&orb| orb == orb_type) {
            self.orbs.remove(pos);
            true
        } else {
            false
        }
    }
    pub fn subtract_moonrocks(&mut self, amount: u32) { 
        self.moonrocks = self.moonrocks.saturating_sub(amount); 
    }
    pub fn subtract_cheddah(&mut self, amount: u32) { 
        self.cheddah = self.cheddah.saturating_sub(amount); 
    }

    pub fn is_dead(&self) -> bool { self.health == 0 }
    pub fn is_at_max_health(&self) -> bool { self.health == 5 }
    pub fn has_orb(&self, orb_type: Orb) -> bool { 
        self.orbs.contains(&orb_type) 
    }
    pub fn orb_count(&self, orb_type: Orb) -> usize {
        self.orbs.iter().filter(|&&orb| orb == orb_type).count()
    }
    pub fn total_orb_count(&self) -> usize {
        self.orbs.len()
    }
    pub fn health_orb_count(&self) -> usize {
        self.orb_count(Orb::Health)
    }
    pub fn point_orb_count(&self) -> usize {
        self.orb_count(Orb::Point)
    }
    pub fn bomb_orb_count(&self) -> usize {
        self.orb_count(Orb::Bomb)
    }
    pub fn has_moonrocks(&self, count: u32) -> bool { self.moonrocks >= count }
    pub fn has_cheddah(&self, count: u32) -> bool { self.cheddah >= count }
    
    pub fn reset_to_defaults(&mut self) {
        *self = Self::default();
    }

    pub fn advance_to_next_level(&mut self) {
        if self.level < 5 {
            let current_moonrocks = self.moonrocks;
            let current_cheddah = self.cheddah;
            
            self.level += 1;
            *self = Self::new_for_level(self.level);
            
            self.points = 0;
            self.moonrocks = current_moonrocks;
            self.cheddah = current_cheddah;
        }
    }

    pub fn is_final_level(&self) -> bool {
        self.level >= 5
    }

    pub fn pull_orb(&mut self) -> Option<Orb> {
        if self.orbs.is_empty() {
            return None;
        }

        let mut rng = rand::thread_rng();
        let random_index = rng.gen_range(0..self.orbs.len());
        
        let orb = self.orbs.remove(random_index);
        
        match orb {
            Orb::Health => {
                if !self.is_at_max_health() {
                    self.add_health(1);
                    info!("Consumed Health orb: +1 health (now {})", self.health);
                } else {
                    info!("Consumed Health orb: no effect (health already at max)");
                }
            },
            Orb::Point => {
                self.add_points(5);
                info!("Consumed Point orb: +5 points");
            },
            Orb::Bomb => {
                self.subtract_health(2);
                info!("Consumed Bomb orb: -2 health");
            },
        }

        Some(orb)
    }
}