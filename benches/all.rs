use criterion::{criterion_group, criterion_main, Criterion, black_box};
extern crate rust_1_brc;

// const FILE_LOCATION: &str = "../1brc/measurements.txt";
const FILE_LOCATION: &str = "../1brc/measurements_10_000.txt";

use rust_1_brc::t001_basic;
use rust_1_brc::t002_fixed_point;
use rust_1_brc::t003_branchless_parsing;
use rust_1_brc::t004_remove_bound_checks;
use rust_1_brc::t005_remove_bound_check_unsafe;
use rust_1_brc::t006_memmap;
use rust_1_brc::t007_memmap_fixed_point;
use rust_1_brc::t008_bittwiddeling_parsing_extreme;
use rust_1_brc::t009_btree;
use rust_1_brc::t010_radix_trie;
use rust_1_brc::t011_fxhash;
use rust_1_brc::t012_xxhash1;
use rust_1_brc::t013_xxhash2;
fn bench_1brcs(c: &mut Criterion) {
    let mut group = c.benchmark_group("1brcs");
    group.bench_function("001_basic", |b| b.iter(|| t001_basic::one_brc(black_box(FILE_LOCATION))));
    group.bench_function("002_fixed_point", |b| b.iter(|| t002_fixed_point::one_brc(black_box(FILE_LOCATION))));
    group.bench_function("003_branchless_parsing", |b| b.iter(|| t003_branchless_parsing::one_brc(black_box(FILE_LOCATION))));
    group.bench_function("004_remove_bound_check", |b| b.iter(|| t004_remove_bound_checks::one_brc(black_box(FILE_LOCATION))));
    group.bench_function("005_remove_bound_check_usafe", |b| b.iter(|| t005_remove_bound_check_unsafe::one_brc(black_box(FILE_LOCATION))));
    group.bench_function("006_memmap", |b| b.iter(|| t006_memmap::one_brc(black_box(FILE_LOCATION))));
    group.bench_function("007_memmap_fixed_point", |b| b.iter(|| t007_memmap_fixed_point::one_brc(black_box(FILE_LOCATION))));
    group.bench_function("008_bittwiddeling_parsing_extreme", |b| b.iter(|| t008_bittwiddeling_parsing_extreme::one_brc(black_box(FILE_LOCATION))));
    group.bench_function("009_btree", |b| b.iter(|| t009_btree::one_brc(black_box(FILE_LOCATION))));
    group.bench_function("010_radix_trie", |b| b.iter(|| t010_radix_trie::one_brc(black_box(FILE_LOCATION))));
    group.bench_function("011_fxhash", |b| b.iter(|| t011_fxhash::one_brc(black_box(FILE_LOCATION))));
    group.bench_function("012_xxhash1", |b| b.iter(|| t012_xxhash1::one_brc(black_box(FILE_LOCATION))));
    group.bench_function("013_xxhash2", |b| b.iter(|| t013_xxhash2::one_brc(black_box(FILE_LOCATION))));

    group.finish();
}

criterion_group!(benches, bench_1brcs);
criterion_main!(benches);
