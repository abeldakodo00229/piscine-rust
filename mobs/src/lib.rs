pub mod boss {
    #[derive(Debug, Clone, PartialEq)]
    pub struct Boss {
        pub name: String,
        pub age: u8,
    }
    impl Boss {
        pub fn new(name: &str, age: u8) -> Boss {
            Boss {
                name: name.to_string(),
                age,
            }
        }
    }
}
pub mod member {
    #[derive(Debug, Clone, PartialEq)]
    pub enum Role {
        Underboss,
        Caporegime,
        Soldier,
        Associate,
    }
    #[derive(Debug, Clone, PartialEq)]
    pub struct Member {
        pub name: String,
        pub role: Role,
        pub age: u8,
    }
    impl Member {
        pub fn new(name: &str, role: Role, age: u8) -> Member {
            Member {
                name: name.to_string(),
                role,
                age,
            }
        }
        pub fn get_promotion(&mut self) {
            self.role = match self.role {
                Role::Associate => Role::Soldier,
                Role::Soldier => Role::Caporegime,
                Role::Caporegime => Role::Underboss,
                Role::Underboss => Role::Underboss,
            }
        }
    }
}
pub use boss::*;
pub use member::*;
#[derive(Debug, Clone, PartialEq)]
pub struct Mob {
    pub name: String,
    pub boss: Boss,
    pub members: Vec<Member>,
    pub cities: Vec<(String, u8)>,
    pub wealth: u32,
}
impl Mob {
    pub fn recruit(&mut self, name: &str, age: u8) {
        self.members.push(Member::new(name, Role::Associate, age));
    }
    pub fn attack(&mut self, opponent: &mut Mob) {
        let (weakest, strongest) = self.weakest_mob(opponent);
        weakest.members.pop();
        if weakest.members.len() == 0 {
            strongest.cities.extend(weakest.cities.clone());
            strongest.wealth += weakest.wealth;
            weakest.wealth = 0;
            weakest.cities.clear();
        }
    }
    fn weakest_mob<'a>(&'a mut self, opponent: &'a mut Mob) -> (&mut Mob, &mut Mob) {
        let my = self.power_combat();
        let him = opponent.power_combat();
        if my > him {
            (opponent, self)
        } else {
            (self, opponent)
        }
    }
    fn power_combat(&self) -> u32 {
        let mut powers: u32 = 0;
        for member in &self.members {
            powers += match member.role {
                Role::Associate => 1,
                Role::Soldier => 2,
                Role::Caporegime => 3,
                Role::Underboss => 4,
            };
        }
        powers
    }
    pub fn steal(&mut self, opponent: &mut Mob, value: u32) {
        if value < opponent.wealth {
            self.wealth += value;
            opponent.wealth -= value;
        } else {
            self.wealth += opponent.wealth;
            opponent.wealth = 0;
        }
    }
    pub fn conquer_city(&mut self, _mobs: Vec<Mob>, name: String, value: u8) {
        if !_mobs
            .iter()
            .any(|c| c.cities.iter().any(|city| city.0 == name))
        {
            self.cities.push((name, value))
        }
    }
}