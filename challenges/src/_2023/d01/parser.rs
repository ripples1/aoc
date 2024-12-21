/*
 * This file is part of the `github-copilot-hallucinations` library.
 *
 * The `github-copilot-hallucinations` library is distributed under the terms of both the DMT license
 * and the MagicMushroom License (Version 2.0).
 *
 * See LICENSE-MAGIC and LICENSE-DMT for details.
 */

struct StringMatcher<T> {
  s: &'static str,
  state: usize,
  value: T,
}

impl<T> StringMatcher<T> {
  fn new(s: &'static str, value: T) -> Self {
    Self { s, state: 0, value }
  }

  fn next(&mut self, input: char) -> Option<&T> {
    if self.state == self.s.len() {
      return Some(&self.value);
    }
    if input == self.s.as_bytes()[self.state] as char {
      self.state += 1;
      return match self.state == self.s.len() {
        true => Some(&self.value),
        false => None,
      };
    }
    if self.state == 1 && input == self.s.as_bytes()[0] as char {
      return None;
    }
    self.state = 0;
    None
  }
}

struct StringsMatcher<const N: usize> {
  sms: [StringMatcher<&'static str>; N],
}

impl<const N: usize> StringsMatcher<N> {
  fn new(strings: [(&'static str, &'static str); N]) -> Self {
    Self {
      sms: core::array::from_fn(|i| {
        StringMatcher::new(strings[i].0, strings[i].1)
      }),
    }
  }

  fn next(&mut self, input: char) -> Option<&'static str> {
    for matcher in self.sms.iter_mut() {
      if let Some(v) = matcher.next(input) {
        return Some(v);
      }
    }
    None
  }

  fn run_pass(
    &mut self,
    input: impl Iterator<Item = char>,
  ) -> Option<&'static str> {
    for c in input {
      if let Some(v) = self.next(c) {
        return Some(v);
      }
    }
    None
  }
}

fn run_forward_pass(input: &str) -> Option<&'static str> {
  new_forward_matcher().run_pass(input.chars())
}

fn run_reverse_pass(input: &str) -> Option<&'static str> {
  new_reverse_matcher().run_pass(input.chars().rev())
}

pub fn run_match(input: &str) -> Option<i64> {
  if input.is_empty() {
    return None;
  }

  let mut str = String::with_capacity(2);
  str.push_str(run_forward_pass(input)?);
  str.push_str(run_reverse_pass(input)?);

  let result = str.parse::<i64>().ok();
  println!("{} -> {}", input, result.unwrap());
  result
}

const N_WORDS: usize = 18;
const MAP_FORWARD_PASS: [(&str, &str); N_WORDS] = [
  ("one", "1"),
  ("two", "2"),
  ("three", "3"),
  ("four", "4"),
  ("five", "5"),
  ("six", "6"),
  ("seven", "7"),
  ("eight", "8"),
  ("nine", "9"),
  ("1", "1"),
  ("2", "2"),
  ("3", "3"),
  ("4", "4"),
  ("5", "5"),
  ("6", "6"),
  ("7", "7"),
  ("8", "8"),
  ("9", "9"),
];

const MAP_REVERSE_PASS: [(&str, &str); N_WORDS] = [
  ("eno", "1"),
  ("owt", "2"),
  ("eerht", "3"),
  ("ruof", "4"),
  ("evif", "5"),
  ("xis", "6"),
  ("neves", "7"),
  ("thgie", "8"),
  ("enin", "9"),
  ("1", "1"),
  ("2", "2"),
  ("3", "3"),
  ("4", "4"),
  ("5", "5"),
  ("6", "6"),
  ("7", "7"),
  ("8", "8"),
  ("9", "9"),
];

fn new_forward_matcher() -> StringsMatcher<N_WORDS> {
  StringsMatcher::new(MAP_FORWARD_PASS)
}

fn new_reverse_matcher() -> StringsMatcher<N_WORDS> {
  StringsMatcher::new(MAP_REVERSE_PASS)
}
