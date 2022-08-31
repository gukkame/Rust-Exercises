pub mod boss;
pub mod member;
use crate::boss::*;
use crate::member::*;
#[derive(Debug, Clone, PartialEq)]
pub struct Mob {
    pub name: String,
    pub boss: Boss,
    pub members: Vec<Member>,
    pub cities: Vec<(String, u8)>,
    pub wealth: u32
}
impl Mob {
    pub fn recruit(&mut self, name: &str, age: u8) {
        self.members.push(Member {
            name: name.to_string(),
            age: age,
            role: Role::Associate
        })
    }
    pub fn attack(&mut self, enemy: &mut Mob) {
        let own_power = self.add_power();
        let enemy_power = enemy.add_power();
        if enemy_power >= own_power {
            enemy.victory(self)
        } else {
            self.victory(enemy)
        }
    }
    fn add_power(&self) -> u32 {
        let mut power = 0;
        for member in self.members.iter() {
            match member.role {
                Role::Associate => power += 1,
                Role::Soldier => power += 2,
                Role::Caporegime => power += 3,
                Role::Underboss => power += 4
            }
        }
        return power
    }
    fn victory(&mut self, enemy: &mut Mob) {
        enemy.members.pop();
        if enemy.members.len() == 0 {
            self.wealth += enemy.wealth;
            for city in enemy.cities.iter() {
                self.cities.push(city.clone());
            }
            enemy.wealth = 0;
            enemy.cities.clear();
        }
    }
    pub fn steal(&mut self, enemy: &mut Mob, amount: u32) {
        if enemy.wealth < amount {
            self.wealth += enemy.wealth;
            enemy.wealth = 0;
        } else {
            self.wealth += amount;
            enemy.wealth -= amount;
        }
    }
    pub fn conquer_city(&mut self, enemies: Vec<Mob>, needed_city: String, value: u8) {
        let mut check = true;
        for enemy in enemies {
            for city in &enemy.cities {
                if city.0 == needed_city {
                    check = false;
                    break
                }
            }
            if !check {break}
        }
        if check {
            self.cities.push((needed_city, value))
        }
    }
}