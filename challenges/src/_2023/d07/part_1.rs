use std::collections::HashMap;
use super::hand::{count_map, hand_type_from_count_map, Hand, Type};

fn hand_type(cards: [u8; 5]) -> Type {
  hand_type_from_count_map(
    &count_map(cards.into_iter())
  )
}

impl Hand {
  pub fn parse_v1(s: &str) -> Hand {
    if s.len() != 5 {
      panic!("Hand must have only 5 cards");
    }
    let mut cards = [0; 5];
    for (i, c) in s.chars().enumerate() {
      cards[i] = card_value(c);
    }
    Hand::new_v1(cards, hand_type(cards))
  }
}

pub const fn card_value(c: char) -> u8 {
  match c {
    '2' => 2,
    '3' => 3,
    '4' => 4,
    '5' => 5,
    '6' => 6,
    '7' => 7,
    '8' => 8,
    '9' => 9,
    'T' => 10,
    'J' => 11,
    'Q' => 12,
    'K' => 13,
    'A' => 14,
    _ => panic!("Bad card")
  }
}

pub const fn value_card(c: u8) -> char {
  match c {
    2 => '2',
    3 => '3',
    4 => '4',
    5 => '5',
    6 => '6',
    7 => '7',
    8 => '8',
    9 => '9',
    10 => 'T',
    11 => 'J',
    12 => 'Q',
    13 => 'K',
    14 => 'A',
    _ => panic!("Bad card")
  }
}