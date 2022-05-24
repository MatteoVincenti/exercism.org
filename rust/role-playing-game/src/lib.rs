pub struct Player {
    pub health: u32,
    pub mana: Option<u32>,
    pub level: u32,
}

impl Player {
    pub fn revive(&self) -> Option<Player> {
        let mana = match self.level >= 10 {
            true => Some(100),
            false => self.mana
        };

        match self.health {
            0 => Some(Player{ health: 100, mana, level: self.level }),
            _ => None
        }
    }

    pub fn cast_spell(&mut self, mana_cost: u32) -> u32 {
        match self.mana {
            Some(mana) => {
                if mana >= mana_cost {
                    self.mana = Some(mana - mana_cost);
                    2 * mana_cost
                } else {
                    0
                }
            },
            None => {
                self.health = if self.health >= mana_cost { self.health - mana_cost } else { 0 };
                0
            }
        }
    }
}
