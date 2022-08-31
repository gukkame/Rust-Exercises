#[derive(Debug, Clone, PartialEq)]
pub enum Role {
    Underboss,
    Caporegime,
    Soldier, 
    Associate
}
#[derive(Debug, Clone, PartialEq)]
pub struct Member {
    pub name: String,
    pub age: u8,
    pub role: Role
}
pub fn new(name: &str, role: Role, age: u8) -> Member {
    return Member {
        name: name.to_string(), 
        age: age, 
        role: role
    }
}
impl Member {
    pub fn get_promotion(&mut self) {
        match self.role {
            Role::Associate => self.role = Role::Soldier,
            Role::Soldier => self.role = Role::Caporegime,
            Role::Caporegime => self.role = Role::Underboss,
            _ => {}
        }
    }
}