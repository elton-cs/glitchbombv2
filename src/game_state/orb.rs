#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Orb {
    Health(u32),
    Point(u32),
    Bomb(u32),
}

impl Orb {
    pub fn create_base_set() -> Vec<Orb> {
        let mut orbs = Vec::new();
        for _ in 0..5 {
            orbs.push(Orb::Health(1));
            orbs.push(Orb::Point(5));
            orbs.push(Orb::Bomb(2));
        }
        orbs
    }
}