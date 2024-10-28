use rand::Rng;

#[derive(Debug, PartialEq, Eq)]
pub enum Suit {
    Heart,
    Diamond,
    Spade,
    Club,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Rank {
    Ace,
    Number(u8),
    Jack,
    Queen,
    King,
}

impl Suit {
	pub fn random() -> Suit {
      let  mut rng = rand::thread_rng();
      let  n : u8 = rng.gen_range(0,4);
      match n{
        0 => Suit::Heart,
        1 => Suit::Diamond,
        2 => Suit::Spade,
        3 => Suit::Club,
        4_u8..=u8::MAX => todo!(),
      }
	}

	pub fn translate(value: u8) -> Suit {
        match value{
            1 => Suit::Heart,
            2 => Suit::Diamond,
            3 => Suit::Spade,
            4 => Suit::Club,
            0_u8 | 5_u8..=u8::MAX => todo!(),

          }
	}
}

impl Rank {
	pub  fn random() -> Rank {
        let  mut rng = rand::thread_rng();
        let  x : u8 = rng.gen_range(1,14);
        match x {
            1 => Rank::Ace,
            11 => Rank::Jack,
            12 => Rank::Queen,
            13 => Rank::King,
            n => Rank::Number(n as u8),

        }
    }

	pub fn translate(value: u8) -> Rank {
            match value {
                1 => Rank::Ace,
                11 => Rank::Jack,
                12 => Rank::Queen,
                13 => Rank::King,
                n if n >= 2 && n <= 10 => Rank::Number(n as u8),
                0_u8 | 2_u8..=10_u8 | 14_u8..=u8::MAX => todo!(),
            }
	}
}

#[derive(Debug, PartialEq, Eq)]
pub struct Card {
	pub suit: Suit,
	pub rank: Rank,
}

pub fn winner_card(card: &Card) -> bool {
    matches!(card.rank, Rank::Ace) && matches!(card.suit, Suit::Spade)
}
