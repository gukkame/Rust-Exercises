#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Game {
    pub id: u32,
    pub p1: (String, u16),
    pub p2: (String, u16),
    pub nbr_of_games: u16,
}

impl Game {
    pub fn new(i: u32, pl1: String, pl2: String, n: u16) -> Box<Game> {
        Box::new(Game {
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



#[cfg(test)]
mod tests {
    use super::*;
    fn create_games() -> Vec<Box<Game>> {
        vec![
            Game::new(0, String::from("player1"), String::from("player2"), 1),
            Game::new(1, String::from("Alice"), String::from("Mark"), 3),
            Game::new(2, String::from("Jack"), String::from("Miller"), 5),
        ]
    }
    #[test]
    fn test_create() {
        let games = create_games();
        assert_eq!(
            *games[0],
            Game {
                id: 0,
                p1: (String::from("player1"), 0),
                p2: (String::from("player2"), 0),
                nbr_of_games: 1
            }
        );
        assert_eq!(
            *games[1],
            Game {
                id: 1,
                p1: (String::from("Alice"), 0),
                p2: (String::from("Mark"), 0),
                nbr_of_games: 3
            }
        );
        assert_eq!(
            *games[2],
            Game {
                id: 2,
                p1: (String::from("Jack"), 0),
                p2: (String::from("Miller"), 0),
                nbr_of_games: 5
            }
        );
    }
    #[test]
    fn test_update_and_read() {
        let mut games = create_games();
        games[0].update_score(String::from("player1"));
        assert_eq!(games[0].read_winner(), (String::from("player1"), 1));
        games[0].update_score(String::from("player2"));
        // this will stay the same because the nbr_of_games is 1 so if one
        // of the players wins just once it will no longer increment the score
        assert_eq!(games[0].read_winner(), (String::from("player1"), 1));
        games[2].update_score(String::from("Jack"));
        games[2].update_score(String::from("Jack"));
        games[2].update_score(String::from("Miller"));
        games[2].update_score(String::from("Miller"));
        // tie between players
        assert_eq!(
            games[2].read_winner(),
            (String::from("Same score! tied"), 2)
        );
        games[2].update_score(String::from("Jack"));
        assert_eq!(games[2].read_winner(), (String::from("Jack"), 3));
    }
    #[test]
    fn test_delete() {
        let game = Game::new(0, String::from("Alice"), String::from("Mark"), 3);
        let game1 = Game::new(23, String::from("Jack"), String::from("Miller"), 1);
        assert_eq!(game.delete(), String::from("game deleted: id -> 0"));
        assert_eq!(game1.delete(), String::from("game deleted: id -> 23"));
    }
    // #[test]
    // #[should_panic]
    // fn test_delete_ownership() {
    //     let game = new(0, String::from("Alice"), String::from("Mark"), 3);
    //     {
    //         let a = &game;
    //         // error! cant destroy boxed while the inner value is borrowed later in scope
    //         delete(game);
    //         read_winner(&a);
    //     }
    // }
}
