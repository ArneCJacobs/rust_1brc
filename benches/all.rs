use criterion::{criterion_group, criterion_main, Criterion, black_box};
extern crate rust_1_brc;

// const FILE_LOCATION: &str = "/Users/steam/git/1brc/measurements.txt";
const FILE_LOCATION: &str = "/Users/steam/git/1brc/measurements_10_000.txt";

use rust_1_brc::t001_basic;
use rust_1_brc::t002_fixed_point;
use rust_1_brc::t003_branchless_parsing;
fn bench_1brcs(c: &mut Criterion) {
    let mut group = c.benchmark_group("1brcs");
    group.bench_function("001_basic", |b| b.iter(|| t001_basic::one_brc(black_box(FILE_LOCATION))));
    group.bench_function("002_basic", |b| b.iter(|| t002_fixed_point::one_brc(black_box(FILE_LOCATION))));
    group.bench_function("003_branchless_parsing", |b| b.iter(|| t003_branchless_parsing::one_brc(black_box(FILE_LOCATION))));

    group.finish();
}

criterion_group!(benches, bench_1brcs);
criterion_main!(benches);
