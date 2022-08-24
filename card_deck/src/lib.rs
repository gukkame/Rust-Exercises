use rand::Rng;
#[derive(Debug)]
pub enum Suit {
    Heart,
    Diamond,
    Spade,
    Club,
}
#[derive(Debug)]
pub enum Rank {
    Ace,
    King,
    Queen,
    Jack,
    Number(u8),
}

impl Suit {
    pub fn random() -> Suit {
        let mut rng = rand::thread_rng();
        match rng.gen_range(0, 3) {
            0 => Suit::Heart,
            1 => Suit::Diamond,
            2 => Suit::Spade,
            _ => Suit::Club,
        }
    }

    pub fn translate(value: u8) -> Suit {
        match value {
            1 => Suit::Heart,
            2 => Suit::Diamond,
            3 => Suit::Spade,
            _ => Suit::Club,
        }
    }
}

impl Rank {
    pub fn random() -> Rank {
        let mut rng = rand::thread_rng();
        match rng.gen_range(0, 12) {
            0 => Rank::Ace,
            1 => Rank::King,
            2 => Rank::Queen,
            3 => Rank::Jack,
            4 => Rank::Number(2),
            5 => Rank::Number(3),
            6 => Rank::Number(4),
            7 => Rank::Number(5),
            8 => Rank::Number(6),
            9 => Rank::Number(7),
            10 => Rank::Number(8),
            11 => Rank::Number(9),
            _ => Rank::Number(10),
        }
    }

    pub fn translate(value: u8) -> Rank {
        match value {
            1 => Rank::Ace,
            2 => Rank::Number(2),
            3 => Rank::Number(3),
            4 => Rank::Number(4),
            5 => Rank::Number(5),
            6 => Rank::Number(6),
            7 => Rank::Number(7),
            8 => Rank::Number(8),
            9 => Rank::Number(9),
            10 => Rank::Number(10),
            11 => Rank::Jack,
            12 => Rank::Queen,
            _ => Rank::King,
        }
    }
}
#[derive(Debug)]
pub struct Card {
    pub suit: Suit,
    pub rank: Rank,
}

pub fn winner_card(car: &Card) -> bool {
    
    if matches!(car.rank, Rank::Ace) &&  matches!(car.suit, Suit::Spade) {
        true
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use card_deck::{self, Card, Rank, Suit};

    #[allow(dead_code)]
    fn main() {
        let your_card = Card {
            rank: Rank::random(),
            suit: Suit::random(),
        };
        println!("You're card is {:?}", your_card);
        // Now if the card is an Ace of Spades print "You are the winner"
        if card_deck::winner_card(your_card) {
            println!("You are the winner!");
        }
    }
    #[test]
    fn test_winner() {
        let winner = Card {
            rank: Rank::Ace,
            suit: Suit::Spade,
        };
        for rank in 1..14 {
            for suit in 1..5 {
                let card = Card {
                    rank: Rank::translate(rank),
                    suit: Suit::translate(suit),
                };
                if card != winner {
                    assert!(!card_deck::winner_card(card));
                } else {
                    assert!(card_deck::winner_card(card));
                }
            }
        }
    }
}
