#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Suit {
    Clubs = 0,
    Diamonds = 1,
    Hearts,
    Spades,
}

impl Suit {
    pub fn from_str(string: &str) -> Result<Self, &str> {
        match string {
            "s" | "S" => Ok(Suit::Spades),
            "c" | "C" => Ok(Suit::Clubs),
            "d" | "D" => Ok(Suit::Diamonds),
            "h" | "H" => Ok(Suit::Hearts),
            &_ => Err("Invalid string for Suit."),
        }
    }

    pub fn to_value(&self) -> u8 {
        match self {
            Suit::Clubs => 0,
            Suit::Diamonds => 1,
            Suit::Hearts => 2,
            Suit::Spades => 3,
        }
    }

    pub fn from_value(val: u8) -> Result<Self, &'static str> {
        match val {
            0 => Ok(Suit::Clubs),
            1 => Ok(Suit::Diamonds),
            2 => Ok(Suit::Hearts),
            3 => Ok(Suit::Spades),
            _ => Err("Invalid Suit Value"),
        }
    }
}

#[cfg(test)]
mod test {

    use super::*;
    #[test]
    fn test_suit_from_str() {
        test_suit("c", Suit::Clubs);
        test_suit("d", Suit::Diamonds);
        test_suit("h", Suit::Hearts);
        test_suit("s", Suit::Spades);
        test_suit("C", Suit::Clubs);
        test_suit("D", Suit::Diamonds);
        test_suit("H", Suit::Hearts);
        test_suit("S", Suit::Spades);
    }

    fn test_suit(string: &str, expected_suit: Suit) {
        let suit = Suit::from_str(string).unwrap();
        assert_eq!(suit, expected_suit);
    }

    #[test]
    fn test_invalid_suit() {
        let value = Suit::from_str("x");
        assert!(value.is_err())
    }

    #[test]
    fn test_bitfield() {
        assert_eq!(Suit::Clubs.to_value(), 0);
        assert_eq!(Suit::Diamonds.to_value(), 1);
        assert_eq!(Suit::Hearts.to_value(), 2);
        assert_eq!(Suit::Spades.to_value(), 3);
    }
}
