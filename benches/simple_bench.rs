use criterion::{criterion_group, criterion_main, Criterion, black_box};

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("loop 100", |b| b.iter(|| 
            mylib::really_complicated_code(black_box(100), black_box(100))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
