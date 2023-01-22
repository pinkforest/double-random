use criterion::{criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    let mut rng = rand::thread_rng();

    c.bench_function("thread_rng double_random_unsafe", |b| {
        b.iter(|| play_transmute::generate_double_random_unsafe(&mut rng))
    });
    c.bench_function("thread_rng double_random_safe", |b| {
        b.iter(|| play_transmute::generate_double_random_safe(&mut rng))
    });

    // 32 byte seed
    let seed = [
        1, 0, 52, 0,  0,  0, 0, 0,
        1, 0, 10, 0, 22, 32, 0, 0,
        2, 0, 55, 49, 0, 11, 0, 0,
        3, 0,  0,  0, 0, 0, 2, 92,
    ];

    use rand::SeedableRng;
    let mut chacha20rng = rand_chacha::ChaCha20Rng::from_seed(seed);

    c.bench_function("chacha20 double_random_unsafe", |b| {
        b.iter(|| play_transmute::generate_double_random_unsafe(&mut chacha20rng))
    });
    c.bench_function("chacha20 double_random_safe", |b| {
        b.iter(|| play_transmute::generate_double_random_safe(&mut chacha20rng))
    });

}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
