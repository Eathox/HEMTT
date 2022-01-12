use criterion::{criterion_group, criterion_main, Criterion};
use hemtt_arma_config::tokenizer::tokenize;

fn criterion_benchmark(c: &mut Criterion) {
    let input = std::fs::read_to_string("benches/tokenizer/resources/cba_script_macros_common.hpp")
        .unwrap();
    c.bench_function("cba_script_macros_common", |b| {
        b.iter(|| tokenize(&input, "cba_script_macros_common").unwrap())
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
