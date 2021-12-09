use aoc_2021::d7;
/// Benchmarks on v1 solutions:
///
/// d7 part1 test           time:   [5.1057 us 5.1255 us 5.1487 us]
///                         change: [-1.3671% -0.7933% -0.1657%] (p = 0.01 < 0.05)
///                         Change within noise threshold.
/// Found 5 outliers among 100 measurements (5.00%)
///   3 (3.00%) high mild
///   2 (2.00%) high severe
///
/// d7 part1                time:   [43.079 us 43.288 us 43.542 us]
///                         change: [-2.1857% -1.6840% -1.0281%] (p = 0.00 < 0.05)
///                         Performance has improved.
/// Found 3 outliers among 100 measurements (3.00%)
///   2 (2.00%) high mild
///   1 (1.00%) high severe
///
/// d7 part2 test           time:   [5.2489 us 5.2597 us 5.2710 us]
///                         change: [-1.1802% -0.7722% -0.3924%] (p = 0.00 < 0.05)
///                         Change within noise threshold.
/// Found 1 outliers among 100 measurements (1.00%)
///   1 (1.00%) high mild
///
/// d7 part2                time:   [878.05 us 882.47 us 886.07 us]
///                         change: [-1.2070% -0.6396% -0.0663%] (p = 0.03 < 0.05)
///                         Change within noise threshold.
///
use criterion::{criterion_group, criterion_main, Criterion};

pub fn bench_d7_part1_test(c: &mut Criterion) {
    c.bench_function("d7 part1 test", |b| {
        b.iter(|| d7::d7_part1("inputs/d7_test"))
    });
}

pub fn bench_d7_part1(c: &mut Criterion) {
    c.bench_function("d7 part1", |b| b.iter(|| d7::d7_part1("inputs/d7")));
}

pub fn bench_d7_part2_test(c: &mut Criterion) {
    c.bench_function("d7 part2 test", |b| {
        b.iter(|| d7::d7_part2("inputs/d7_test"))
    });
}

pub fn bench_d7_part2(c: &mut Criterion) {
    c.bench_function("d7 part2", |b| b.iter(|| d7::d7_part2("inputs/d7")));
}

criterion_group! {
    name = benches;
    config = Criterion::default();
    targets = bench_d7_part1_test, bench_d7_part1, bench_d7_part2_test, bench_d7_part2
}
criterion_main!(benches);
