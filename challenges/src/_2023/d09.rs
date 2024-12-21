// Here goes

fn parse_number_list(s: &str) -> Vec<i64> {
  s.split(" ").map(|n| n.parse().unwrap()).collect()
}

fn is_all_zero(history: &[i64]) -> bool {
  history.iter().all(|n| *n == 0)
}

fn diff(history: &[i64]) -> Vec<i64> {
  history.windows(2).map(|w| w[1] - w[0]).collect()
}

fn next_item(history: &[i64]) -> i64 {
  if is_all_zero(history) {
    return 0;
  }
  history.last().unwrap() + next_item(diff(history).as_slice())
}

fn prev_item(history: &[i64]) -> i64 {
  if is_all_zero(history) {
    return 0;
  }
  history.first().unwrap() - prev_item(diff(history).as_slice())
}

pub fn part_1(input: &str) -> i64 {
  input
    .lines()
    .map(parse_number_list)
    .map(|h| next_item(h.as_slice()))
    .sum()
}

pub fn part_2(input: &str) -> i64 {
  input
    .lines()
    .map(parse_number_list)
    .map(|h| prev_item(h.as_slice()))
    .sum()
}
