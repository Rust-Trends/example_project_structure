use criterion::{criterion_group, criterion_main, Criterion, black_box};
mod bench_module;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("loop 100", |b| b.iter(|| 
            mylib::really_complicated_code(black_box(100), black_box(100))));

    bench_module::show_bench_module();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
