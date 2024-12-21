use std::collections::HashMap;

#[derive(PartialEq, PartialOrd, Eq, Ord, Clone, Copy)]
pub enum Type {
  HighCard,
  OnePair,
  TwoPair,
  ThreeOfAKind,
  FullHouse,
  FourOfAKind,
  FiveOfAKind,
}

pub fn count_map(cards: impl Iterator<Item = u8>) -> HashMap<u8, i32> {
  cards.fold(HashMap::new(), |mut map, c| {
    *map.entry(c).or_insert(0) += 1;
    map
  })
}

pub fn hand_type_from_count_map(map: &HashMap<u8, i32>) -> Type {
  match map.len() {
    5 => Type::HighCard,
    4 => Type::OnePair,
    3 => {
      if map.values().any(|&v| v == 3) {
        Type::ThreeOfAKind
      } else {
        Type::TwoPair
      }
    }
    2 => {
      if map.values().any(|&v| v == 4) {
        Type::FourOfAKind
      } else {
        Type::FullHouse
      }
    }
    _ => Type::FiveOfAKind,
  }
}

pub struct Hand {
  ver: u8,
  cards: [u8; 5],
  _type: Type,
}

impl Hand {
  pub fn new_v1(cards: [u8; 5], _type: Type) -> Hand {
    Hand {
      ver: 1,
      cards,
      _type,
    }
  }

  pub fn new_v2(cards: [u8; 5], _type: Type) -> Hand {
    Hand {
      ver: 2,
      cards,
      _type,
    }
  }
}

impl PartialEq for Hand {
  fn eq(&self, other: &Self) -> bool {
    if self._type != other._type {
      false
    } else {
      self.cards == other.cards
    }
  }
}

impl Eq for Hand {}

impl Ord for Hand {
  fn cmp(&self, other: &Self) -> std::cmp::Ordering {
    if self._type != other._type {
      self._type.cmp(&other._type)
    } else {
      self.cards.cmp(&other.cards)
    }
  }
}

impl PartialOrd for Hand {
  fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
    Some(self.cmp(other))
  }
}

impl core::fmt::Debug for Hand {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let _fn = match self.ver {
      1 => super::part_1::value_card,
      2 => super::part_2::value_card,
      _ => panic!("Bad hand version"),
    };
    write!(
      f,
      "{}({:?})",
      self.cards.map(_fn).iter().collect::<String>(),
      self._type as u8
    )
  }
}
