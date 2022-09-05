#[derive(Debug, Clone, Eq, PartialEq)]
pub struct GameSession {
    pub id: u32,
    pub p1: (String, u16),
    pub p2: (String, u16),
    pub nbr_of_games: u16,
}

impl GameSession {
    pub fn new(i: u32, pl1: String, pl2: String, n: u16) -> Box<GameSession> {
        Box::new(GameSession {
            id: i,
            p1: (pl1, 0),
            p2: (pl2, 0),
            nbr_of_games: n,
        })
    }
    pub fn read_winner(&self) -> (String, u16) {
        println!("Game: {:?}", self);
        if self.p1.1 > self.p2.1 {
            return ((self.p1.0).clone(),self.p1.1)
        }else if self.p1.1 < self.p2.1 {
            return ((self.p2.0).clone(),self.p2.1)
        }else {
            return ("Same score! tied".to_string(), self.p1.1);
        }
    }
    pub fn update_score(&mut self, user_name: String) {

        if self.nbr_of_games > self.p1.1 + self.p2.1{
            if self.p1.0 == user_name {
                self.p1.1 += 1;
            }
            if self.p2.0 == user_name {
                self.p2.1 += 1;
            }
        }
    }
    pub fn delete(self) -> String {

        let mut str = String::new();
        str.push_str("game deleted: id -> ");
        str.push_str((self.id).to_string().as_str());
       str
    }
}



