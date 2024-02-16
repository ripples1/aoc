use criterion::{criterion_group, criterion_main, Criterion};

fn ch_05_2(c: &mut Criterion) {
  let input = std::fs::read_to_string("../input/2023/05.txt").unwrap();
  c.bench_function("2023/5/2", |b| b.iter(|| challenges::run(&input, "2023/5/1")));
}

criterion_group!(benches, ch_05_2);
criterion_main!(benches);