use std::{
    fs::File,
    hint::black_box,
    io::{BufRead, BufReader},
};

use criterion::{Criterion, criterion_group, criterion_main};
use snekkja::Parser;

fn load_samples() -> Vec<String> {
    let file = File::open("samples.txt").unwrap();
    let rdr = BufReader::new(file);

    let mut lines: Vec<String> = Vec::new();

    for line in rdr.lines() {
        lines.push(line.unwrap());
    }

    lines
}

fn parse_lines(parser: &mut Parser, lines: &Vec<Vec<u8>>) {
    for line in lines {
        let _ = parser.parse(line);
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    let mut parser = Parser::default();

    let lines = load_samples().into_iter().map(|s| s.into_bytes()).collect();

    c.bench_function("snekkja_74156", |b| {
        b.iter(|| parse_lines(black_box(&mut parser), black_box(&lines)))
    });
}

criterion_group!(snekkja, criterion_benchmark);
criterion_main!(snekkja);
