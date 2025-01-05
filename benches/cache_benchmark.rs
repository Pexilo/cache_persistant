use cache_exam::{Cache, CacheTrait};
use criterion::{black_box, criterion_group, criterion_main, Criterion}; // https://github.com/bheisler/criterion.rs

fn benchmark_cache(c: &mut Criterion) {
    let filename = "cache_benchmark.txt";
    let mut cache: Cache<String, String> = Cache::new_persistent(3, filename);

    c.bench_function("cache_B", |b| {
        b.iter(|| {
            black_box(cache.get(&"B".to_string()));
        })
    });

    c.bench_function("cache_C", |b| {
        b.iter(|| {
            black_box(cache.get(&"C".to_string()));
        })
    });

    c.bench_function("cache_D", |b| {
        b.iter(|| {
            black_box(cache.get(&"D".to_string()));
        })
    });
}

criterion_group!(benches, benchmark_cache);
criterion_main!(benches);
