use crate::{rank::Rank, suit::Suit};

#[derive(Copy, Clone)]
pub struct Card {
    card_value: u8,
}

impl Card {
    pub fn new(rank: &Rank, suit: &Suit) -> Self {
        Self {
            card_value: suit.to_value() * 13 + rank.to_value(),
        }
    }
    pub fn from_str(card: &str) -> Result<Self, &str> {
        if card.len() != 2 {
            return Err("Invalid string for Card.");
        }

        let rank_result = Rank::from_str(&card[0..1]);
        let suit_result = Suit::from_str(&card[1..2]);

        let rank = match rank_result {
            Ok(rank) => rank,
            Err(e) => return Err(e),
        };

        let suit = match suit_result {
            Ok(suit) => suit,
            Err(e) => return Err(e),
        };

        Ok(Self::new(&rank, &suit))
    }

    pub fn suit(&self) -> Suit {
        Suit::from_value(self.card_value / 13).expect("card got assigned invalid card value")
    }

    pub fn rank(&self) -> Rank {
        Rank::from_value(self.card_value % 13).expect("card got assigned invalid card value")
    }

    pub fn get_bitfield(&self) -> u64 {
        1 << self.card_value
    }

    pub fn from_value(val: u8) -> Self {
        Self { card_value: val }
    }
}

#[cfg(test)]
pub(crate) mod test {
    use super::*;

    #[test]
    fn test_card_from_str() {
        test_card_str("As", Rank::Ace, Suit::Spades);
        test_card_str("2s", Rank::Two, Suit::Spades);
    }

    fn test_card_str(string: &str, rank: Rank, suit: Suit) {
        let card = Card::from_str(string).unwrap();
        test_card(&card, rank, suit);
    }

    pub fn test_card(card: &Card, rank: Rank, suit: Suit) {
        assert_eq!(card.rank(), rank);
        assert_eq!(card.suit(), suit);
    }

    #[test]
    fn test_invalid_card() {
        test_card_invalid("");
        test_card_invalid("Asa");
        test_card_invalid("Ax");
        test_card_invalid("0s");
    }

    fn test_card_invalid(string: &str) {
        let value = Rank::from_str(string);
        assert!(value.is_err())
    }
}
