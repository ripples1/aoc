use self::parser::run_match;

mod parser;

pub fn part_1(input: &str) -> i64 {
  let mut sum: i64 = 0;
  for line in input.lines() {
    sum +=
      format!(
        "{}{}",
        line.chars().find(|c| c.is_digit(10)).expect(
          "Invariant violated: expected that there is at least one digit in the input line"
        ),
        line.chars().rfind(|c| c.is_digit(10)).expect(
          "Invariant violated: expected that there is at least one digit in the input line"
        )
      )
      .parse::<i64>()
      .unwrap();
  }
  sum
}

pub fn part_2(input: &str) -> i64 {
  input
    .lines()
    .fold(0, |acc, line| acc + run_match(line).unwrap())
}
