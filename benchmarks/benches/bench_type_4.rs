use benchmarks::{load_samples, parse_lines};
use criterion::{Criterion, criterion_group, criterion_main};
use snekkja::Parser;
use std::hint::black_box;

fn bench(c: &mut Criterion) {
    let mut parser = Parser::default();
    let lines = load_samples("samples_type_4.txt");
    c.bench_function("type_4_base_station_report", |b| {
        b.iter(|| parse_lines(black_box(&mut parser), black_box(&lines)))
    });
}

criterion_group!(benches, bench);
criterion_main!(benches);
