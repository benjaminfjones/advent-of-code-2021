use aoc_2021::d3;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

// Amazingly, d3 part2 original implementation is ~100x faster than the "optimized" version!
//
// d3 part2                time:   [6.3879 us 6.4717 us 6.5636 us]
//                         change: [-7.4232% -4.3092% -1.4445%] (p = 0.01 < 0.05)
//                         Performance has improved.
// Found 46 outliers among 100 measurements (46.00%)
//   22 (22.00%) low severe
//   1 (1.00%) low mild
//   2 (2.00%) high mild
//   21 (21.00%) high severe
//
// d3 part2 alt2           time:   [781.16 us 781.41 us 781.66 us]
//                         change: [-0.1665% +0.1003% +0.3784%] (p = 0.48 > 0.05)
//                         No change in performance detected.
// Found 17 outliers among 100 measurements (17.00%)
//   11 (11.00%) low severe
//   2 (2.00%) low mild
//   2 (2.00%) high mild
//   2 (2.00%) high severe

pub fn bench_d3_part2(c: &mut Criterion) {
    let input_bvs = d3::input_to_bitvectors("inputs/d3");
    c.bench_function("d3 part2", |b| b.iter(|| black_box(d3::d3_part2(&input_bvs))));
}

pub fn bench_d3_part2_alt2(c: &mut Criterion) {
    let input_bvs = d3::input_to_bitvectors("inputs/d3");
    c.bench_function("d3 part2 alt2", |b| b.iter(|| black_box(d3::d3_part2_alt2(&input_bvs))));
}

criterion_group! {
    name = benches;
    config = Criterion::default();
    targets = bench_d3_part2, bench_d3_part2_alt2
}
criterion_main!(benches);
