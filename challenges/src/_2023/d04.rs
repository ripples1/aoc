use std::{collections::HashSet, fmt::Debug};

type Num = u32;

fn is_not_empty_str(s: &&str) -> bool {
  s.len() != 0
}

fn parse_set(input: &str) -> HashSet<Num> {
  input
    .split(" ")
    .filter(is_not_empty_str)
    .map(|s| s.parse::<Num>().expect("Invalid number"))
    .collect::<HashSet<Num>>()
}

//#[derive(Debug)]
struct Card {
  id: usize,
  matches: Num,
}

impl Debug for Card {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{{#{}={}}}", self.id, self.matches)
  }
}

impl Card {
  fn parse(input: &str) -> Self {
    let [game, sets] = *input.split(":").collect::<Box<[_]>>() else {
      panic!("Invalid input")
    };
    let id = game
      .split(" ")
      .filter(is_not_empty_str)
      .nth(1)
      .expect("Invalid input")
      .parse::<usize>()
      .expect("Invalid number");
    let [winning_str, available_str] = *sets.split("|").collect::<Box<[_]>>()
    else {
      panic!("Invalid input")
    };
    let winning = parse_set(winning_str);
    let available = parse_set(available_str);
    let matches = winning.intersection(&available).count() as Num;
    Self { id, matches }
  }

  fn slice_cards<'a>(&self, cards: &'a [Card]) -> &'a [Card] {
    let start = self.id;
    if start >= cards.len() {
      return &[];
    }
    let end = std::cmp::min(cards.len(), start + self.matches as usize);
    &cards[start..end]
  }

  fn score(&self) -> Num {
    match self.matches {
      0 => 0,
      m => (2 as Num).pow((m - 1) as u32),
    }
  }
}

fn count_copies(cards: &[Card]) -> i64 {
  fn _count(subset: &[Card], cards: &[Card]) -> i64 {
    let mut count = 0;
    for card in subset {
      count += 1 + _count(card.slice_cards(cards), cards);
    }
    count
  }
  _count(cards, cards)
}

pub fn part_1(input: &str) -> i64 {
  input
    .lines()
    .map(|line| Card::parse(line).score())
    .sum::<Num>() as i64
}

pub fn part_2(input: &str) -> i64 {
  let cards = input.lines().map(Card::parse).collect::<Vec<Card>>();
  count_copies(cards.as_slice())
}
