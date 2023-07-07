use criterion::{criterion_group, criterion_main, Criterion};
use rust_monte_carlo::run_experiment;

pub fn criterion_benchmark(c: &mut Criterion) {
    // let threshold:f32 = 0.95;
    // let target_times:u32 = 6;
    // let total_features:usize = 10;
    // let subsample_amt:usize = 6;
    c.bench_function("run monte carlo experiment", |b| b.iter(|| 
        run_experiment(3,100,10,500,0.95)
    ));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
