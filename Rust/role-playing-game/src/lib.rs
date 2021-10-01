// This stub file contains items which aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.

pub struct Player {
    pub health: u32,
    pub mana: Option<u32>,
    pub level: u32,
}

impl Player {
    pub fn revive(&self) -> Option<Player> {
        let mut new_player = Player {
            health: self.health,
            mana: self.mana,
            level: self.level,
        };
        if self.health == 0 {
            new_player.health = 100;
            if self.level >= 10 {
                new_player.mana = Some(100);
            }
            Some(new_player)
        } else {
            None
        }
    }

    pub fn cast_spell(&mut self, mana_cost: u32) -> u32 {
        match self.mana {
            Some(value) => {
                if value < mana_cost {
                    0
                } else {
                    self.mana = Some(value - mana_cost);
                    mana_cost * 2
                }
            }
            None => {
                if mana_cost >= self.health {
                    self.health = 0;
                } else {
                    self.health -= mana_cost;
                }
                0
            }
        }
    }
}
