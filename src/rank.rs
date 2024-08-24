#[derive(Debug, PartialEq, Clone, Copy, PartialOrd, Eq, Ord)]
pub enum Rank {
    Two = 2,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl std::fmt::Display for Rank {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Rank::Ace => {
                write!(f, "A")
            }
            Rank::King => {
                write!(f, "K")
            }
            Rank::Queen => {
                write!(f, "Q")
            }
            Rank::Jack => {
                write!(f, "J")
            }
            Rank::Ten => {
                write!(f, "T")
            }
            Rank::Nine => {
                write!(f, "9")
            }
            Rank::Eight => {
                write!(f, "8")
            }
            Rank::Seven => {
                write!(f, "7")
            }
            Rank::Six => {
                write!(f, "6")
            }
            Rank::Five => {
                write!(f, "5")
            }
            Rank::Four => {
                write!(f, "4")
            }
            Rank::Three => {
                write!(f, "3")
            }
            Rank::Two => {
                write!(f, "2")
            }
        }
    }
}

impl Rank {
    pub fn from_str(string: &str) -> Result<Self, &str> {
        match string {
            "A" | "a" => Ok(Rank::Ace),
            "2" => Ok(Rank::Two),
            "3" => Ok(Rank::Three),
            "4" => Ok(Rank::Four),
            "5" => Ok(Rank::Five),
            "6" => Ok(Rank::Six),
            "7" => Ok(Rank::Seven),
            "8" => Ok(Rank::Eight),
            "9" => Ok(Rank::Nine),
            "T" | "t" => Ok(Rank::Ten),
            "J" | "j" => Ok(Rank::Jack),
            "Q" | "q" => Ok(Rank::Queen),
            "K" | "k" => Ok(Rank::King),
            &_ => Err("Invalid string for Rank."),
        }
    }

    pub fn compare_ranks(a: Vec<u8>, b: Vec<u8>) -> Option<std::cmp::Ordering> {
        for it in a.iter().zip(b.iter()) {
            let (ai, bi) = it;
            let order = ai.partial_cmp(bi);
            if order != Some(std::cmp::Ordering::Equal) {
                return order;
            }
        }
        return Some(std::cmp::Ordering::Equal);
    }

    pub fn to_value(&self) -> u8 {
        match self {
            Rank::Two => 0,
            Rank::Three => 1,
            Rank::Four => 2,
            Rank::Five => 3,
            Rank::Six => 4,
            Rank::Seven => 5,
            Rank::Eight => 6,
            Rank::Nine => 7,
            Rank::Ten => 8,
            Rank::Jack => 9,
            Rank::Queen => 10,
            Rank::King => 11,
            Rank::Ace => 12,
        }
    }

    pub fn from_value(val: u8) -> Result<Self, &'static str> {
        match val {
            0 => Ok(Rank::Two),
            1 => Ok(Rank::Three),
            2 => Ok(Rank::Four),
            3 => Ok(Rank::Five),
            4 => Ok(Rank::Six),
            5 => Ok(Rank::Seven),
            6 => Ok(Rank::Eight),
            7 => Ok(Rank::Nine),
            8 => Ok(Rank::Ten),
            9 => Ok(Rank::Jack),
            10 => Ok(Rank::Queen),
            11 => Ok(Rank::King),
            12 => Ok(Rank::Ace),
            _ => Err("Invalid Rank Value"),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_rank_from_str() {
        test_rank("A", Rank::Ace);
        test_rank("2", Rank::Two);
    }

    fn test_rank(string: &str, expected_rank: Rank) {
        let rank = Rank::from_str(string).unwrap();
        assert_eq!(rank, expected_rank);
    }

    #[test]
    fn test_invalid_rank() {
        let value = Rank::from_str("x");
        assert!(value.is_err())
    }

    #[test]
    fn test_rank_order() {
        let ranks = vec![Rank::Ace, Rank::King, Rank::Queen, Rank::Jack];

        for (i, rank) in ranks.iter().enumerate() {
            for (j, rank_2) in ranks.iter().enumerate() {
                assert_eq!(rank == rank_2, j == i, "{}=={}", rank, rank_2);
                assert_eq!(rank >= rank_2, i <= j, "{}>={}", rank, rank_2);
                assert_eq!(rank <= rank_2, i >= j, "{}<={}", rank, rank_2);
                assert_eq!(rank < rank_2, i > j, "{}<{}: False", rank, rank_2);
                assert_eq!(rank > rank_2, i < j, "{}>{}: False", rank, rank_2);
            }
        }
    }

    #[test]
    fn test_get_bitfield() {
        assert_eq!(Rank::Ace.to_value(), 12);
        assert_eq!(Rank::Two.to_value(), 0);
        assert_eq!(Rank::Three.to_value(), 1);
        assert_eq!(Rank::Four.to_value(), 2);
        assert_eq!(Rank::Five.to_value(), 3);
        assert_eq!(Rank::Six.to_value(), 4);
        assert_eq!(Rank::Seven.to_value(), 5);
        assert_eq!(Rank::Eight.to_value(), 6);
        assert_eq!(Rank::Nine.to_value(), 7);
        assert_eq!(Rank::Ten.to_value(), 8);
        assert_eq!(Rank::Jack.to_value(), 9);
        assert_eq!(Rank::Queen.to_value(), 10);
        assert_eq!(Rank::King.to_value(), 11);
    }
}
