use std::collections::HashMap;
use super::hand::{count_map, hand_type_from_count_map, Hand, Type};

fn is_a_joker(c: &u8) -> bool {
  *c == 1
}

fn is_not_a_joker(c: &u8) -> bool {
  *c != 1
}

fn hand_type(cards: [u8; 5]) -> Type {
  match cards.into_iter().filter(is_a_joker).count() {
    0 => hand_type_from_count_map(
      &count_map(cards.into_iter())
    ),
    4 | 5 => Type::FiveOfAKind,
    jokers@_ => {
      let map = count_map(cards.into_iter().filter(is_not_a_joker));
      let max = *map.values().max().unwrap() as usize; // safe to unwrap
      match jokers + max {
        2 => Type::OnePair,
        3 => {
          match map.len() {
            2 => Type::FullHouse,
            _ => Type::ThreeOfAKind,
          }
        },
        4 => Type::FourOfAKind,
        _ => Type::FiveOfAKind,
      }
    }
  }
}

impl Hand {
  pub fn parse_v2(s: &str) -> Hand {
    if s.len() != 5 {
      panic!("Hand must have only 5 cards");
    }
    let mut cards = [0; 5];
    for (i, c) in s.chars().enumerate() {
      cards[i] = card_value(c);
    }
    Hand::new_v2(cards, hand_type(cards))
  }
}

pub const fn card_value(c: char) -> u8 {
  match c {
    'J' => 1,
    '2' => 2,
    '3' => 3,
    '4' => 4,
    '5' => 5,
    '6' => 6,
    '7' => 7,
    '8' => 8,
    '9' => 9,
    'T' => 10,
    'Q' => 12,
    'K' => 13,
    'A' => 14,
    _ => panic!("Bad card")
  }
}

pub const fn value_card(c: u8) -> char {
  match c {
    1 => 'J',
    2 => '2',
    3 => '3',
    4 => '4',
    5 => '5',
    6 => '6',
    7 => '7',
    8 => '8',
    9 => '9',
    10 => 'T',
    12 => 'Q',
    13 => 'K',
    14 => 'A',
    _ => panic!("Bad card")
  }
}