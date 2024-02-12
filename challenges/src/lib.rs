mod _2023;

pub fn run(input: &str, challenge: &str) -> i64 {
  match challenge {
    "2023/1/1" => _2023::d01::part_1(input),
    "2023/1/2" => _2023::d01::part_2(input),
    _ => panic!("Bad challenge path")
  }
}
