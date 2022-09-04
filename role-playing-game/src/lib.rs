// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

pub struct Player {
    pub health: u32,
    pub mana: Option<u32>,
    pub level: u32,
}

impl Player {
    pub fn revive(&self) -> Option<Player> {
        if self.health == 0 {
            if self.level >= 10 {
                Some(Player {
                    health: 100,
                    mana: Some(100),
                    ..*self
                })
            } else {
                Some(Player {
                    health: 100,
                    ..*self
                })
            }
        } else {
            None
        }
    }

    pub fn cast_spell(&mut self, mana_cost: u32) -> u32 {
        if let Some(mana) = self.mana {
            if mana >= mana_cost {
                self.mana = Some(mana - mana_cost);
                mana_cost * 2
            } else {
                0
            }
        } else {
            if self.health > mana_cost {
                self.health -= mana_cost;
                0
            } else {
                self.health = 0;
                0
            }
        }
    }
}
