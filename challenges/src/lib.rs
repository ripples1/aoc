#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

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
    "2023/7/1" => _2023::d07::part_1(input),
    "2023/7/2" => _2023::d07::part_2(input),
    "2023/8/1" => _2023::d08::part_1(input),
    "2023/8/2" => _2023::d08::part_2(input),
    "2023/9/1" => _2023::d09::part_1(input),
    "2023/9/2" => _2023::d09::part_2(input),
    "2023/10/1" => _2023::d10::part_1(input),
    "2023/10/2" => _2023::d10::part_2(input),
    _ => panic!("Bad challenge path")
  }
}
