mod parser;
mod _2023;

pub fn run(input: &str, challenge: &str) -> i64 {
  match challenge {
    "2023/1/1" => _2023::d01::part_1(input),
    "2023/1/2" => _2023::d01::part_2(input),
    "2023/3/1" => _2023::d03::part_1(input),
    "2023/3/2" => _2023::d03::part_2(input),
    "2023/4/1" => _2023::d04::part_1(input),
    "2023/4/2" => _2023::d04::part_2(input),
    "2023/5/1" => _2023::d05::part_1(input),
    "2023/5/2" => _2023::d05::part_2(input),
    _ => panic!("Bad challenge path")
  }
}
