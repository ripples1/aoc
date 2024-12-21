mod hand;
mod part_1;
mod part_2;

use hand::Hand;
use std::collections::HashMap;

pub fn part_1(input: &str) -> i64 {
  let lines = input.lines();
  let mut cards = lines
    .map(|l| {
      let (a, b) = l.split_at(5);
      (Hand::parse_v1(a), b.trim().parse::<i64>().unwrap())
    })
    .collect::<Vec<_>>();
  cards.sort_by(|a, b| a.0.cmp(&b.0));
  cards
    .iter()
    .enumerate()
    .map(|(i, c)| ((i + 1) as i64) * c.1)
    .sum()
}

pub fn part_2(input: &str) -> i64 {
  let lines = input.lines();
  let mut cards = lines
    .map(|l| {
      let (a, b) = l.split_at(5);
      (Hand::parse_v2(a), b.trim().parse::<i64>().unwrap())
    })
    .collect::<Vec<_>>();
  //println!("{:?}", cards);
  cards.sort_by(|a, b| a.0.cmp(&b.0));
  cards
    .iter()
    .enumerate()
    .map(|(i, c)| ((i + 1) as i64) * c.1)
    .sum()
}
