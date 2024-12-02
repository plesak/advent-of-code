use criterion::{criterion_group, criterion_main, Criterion};

fn bench_day1_part1(c: &mut Criterion) {
    let input = include_str!("../input_files/y24q1q1.txt");
    c.bench_function("day1_part1", |b| b.iter(|| advent_of_code::day1::part1(input)));
}

fn bench_day1_part2(c: &mut Criterion) {
    let input = include_str!("../input_files/y24q1q2.txt");
    c.bench_function("day1_part2", |b| b.iter(|| advent_of_code::day1::part2(input)));
}

fn bench_day2_part1(c: &mut Criterion) {
    let input = include_str!("../input_files/y24q2q1.txt");
    c.bench_function("day2_part1", |b| b.iter(|| advent_of_code::day2::part1(input)));
}

fn bench_day2_part2(c: &mut Criterion) {
    let input = include_str!("../input_files/y24q2q2.txt");
    c.bench_function("day2_part2", |b| b.iter(|| advent_of_code::day2::part2(input)));
}

fn bench_day3_part1(c: &mut Criterion) {
    let input = include_str!("../input_files/y24q3q1.txt");
    c.bench_function("day3_part1", |b| b.iter(|| advent_of_code::day3::part1(input)));
}

fn bench_day3_part2(c: &mut Criterion) {
    let input = include_str!("../input_files/y24q3q2.txt");
    c.bench_function("day3_part2", |b| b.iter(|| advent_of_code::day3::part2(input)));
}

criterion_group!(benches,
    bench_day1_part1,
    bench_day1_part2,
    bench_day2_part1,
    bench_day2_part2,
    bench_day3_part1,
    bench_day3_part2,
);
criterion_main!(benches);