
#![allow(clippy::unit_arg)]

use std::hint::black_box;
use std::time::Duration;

use criterion::{
    BatchSize, BenchmarkGroup, BenchmarkId, Criterion, SamplingMode, Throughput, criterion_group,
    criterion_main, measurement::WallTime,
};
use emap::Map;

const CAPACITY: usize = 65_536;

fn criterion_config() -> Criterion {
    Criterion::default()
        .warm_up_time(Duration::from_secs(2))
        .measurement_time(Duration::from_secs(8))
        .sample_size(60)
        .noise_threshold(0.01)
}

fn setup_prefilled_map<T: Copy>(value: T) -> Map<T> {
    let mut m = Map::with_capacity_none(CAPACITY);
    for i in 0..CAPACITY {
        unsafe { m.insert_unchecked(i, value) };
    }
    m
}

fn bench_remove_safe<T: Copy + 'static>(
    group: &mut BenchmarkGroup<'_, WallTime>,
    label: &str,
    value: T,
) {
    group.bench_function(BenchmarkId::new(label, "safe"), |b| {
        b.iter_batched(
            || setup_prefilled_map(value),
            |mut m| {
                for i in 0..CAPACITY {
                    m.remove(i);
                }
                black_box(m);
            },
            BatchSize::SmallInput,
        );
    });
}

#[cfg(feature = "bench_unchecked")]
fn bench_remove_unchecked<T: Copy + 'static>(
    group: &mut BenchmarkGroup<'_, WallTime>,
    label: &str,
    value: T,
) {
    group.bench_function(BenchmarkId::new(label, "unsafe_unchecked"), |b| {
        b.iter_batched(
            || setup_prefilled_map(value),
            |mut m| {
                for i in 0..CAPACITY {
                    unsafe { m.remove_unchecked(i) };
                }
                black_box(m);
            },
            BatchSize::SmallInput,
        );
    });
}

fn remove_benchmarks(c: &mut Criterion) {
    let mut group = c.benchmark_group("emap_remove");
    group.sampling_mode(SamplingMode::Flat);
    group.throughput(Throughput::Elements(CAPACITY as u64));

    bench_remove_safe(&mut group, "bool", true);
    #[cfg(feature = "bench_unchecked")]
    bench_remove_unchecked(&mut group, "bool", true);

    bench_remove_safe(&mut group, "u32", 42_u32);
    #[cfg(feature = "bench_unchecked")]
    bench_remove_unchecked(&mut group, "u32", 42_u32);

    bench_remove_safe(&mut group, "u64", 42_u64);
    #[cfg(feature = "bench_unchecked")]
    bench_remove_unchecked(&mut group, "u64", 42_u64);

    bench_remove_safe(&mut group, "[u8;1024]", [0_u8; 1024]);
    #[cfg(feature = "bench_unchecked")]
    bench_remove_unchecked(&mut group, "[u8;1024]", [0_u8; 1024]);

    group.finish();
}

criterion_group! {
    name = benches;
    config = criterion_config();
    targets = remove_benchmarks
}
criterion_main!(benches);
