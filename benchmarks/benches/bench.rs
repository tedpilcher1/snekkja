use benchmarks::{load_samples, parse_lines};
use criterion::{Criterion, criterion_group, criterion_main};
use snekkja::Parser;
use std::hint::black_box;

fn bench_mixed(c: &mut Criterion) {
    let mut parser = Parser::default();
    let lines = load_samples("samples_mixed.txt");
    c.bench_function("mixed_all_types", |b| {
        b.iter(|| parse_lines(black_box(&mut parser), black_box(&lines)))
    });
}

fn bench_type_1_2_3(c: &mut Criterion) {
    let mut parser = Parser::default();
    let lines = load_samples("samples_type_1_2_3.txt");
    c.bench_function("type_1_2_3_position_report", |b| {
        b.iter(|| parse_lines(black_box(&mut parser), black_box(&lines)))
    });
}

fn bench_type_4(c: &mut Criterion) {
    let mut parser = Parser::default();
    let lines = load_samples("samples_type_4.txt");
    c.bench_function("type_4_base_station_report", |b| {
        b.iter(|| parse_lines(black_box(&mut parser), black_box(&lines)))
    });
}

fn bench_type_5(c: &mut Criterion) {
    let mut parser = Parser::default();
    let lines = load_samples("samples_type_5.txt");
    c.bench_function("type_5_static_voyage_data", |b| {
        b.iter(|| parse_lines(black_box(&mut parser), black_box(&lines)))
    });
}

fn bench_type_6(c: &mut Criterion) {
    let mut parser = Parser::default();
    let lines = load_samples("samples_type_6.txt");
    c.bench_function("type_6_binary_addressed", |b| {
        b.iter(|| parse_lines(black_box(&mut parser), black_box(&lines)))
    });
}

fn bench_type_7_13(c: &mut Criterion) {
    let mut parser = Parser::default();
    let lines = load_samples("samples_type_7_13.txt");
    c.bench_function("type_7_13_binary_acknowledge", |b| {
        b.iter(|| parse_lines(black_box(&mut parser), black_box(&lines)))
    });
}

fn bench_type_8(c: &mut Criterion) {
    let mut parser = Parser::default();
    let lines = load_samples("samples_type_8.txt");
    c.bench_function("type_8_binary_broadcast", |b| {
        b.iter(|| parse_lines(black_box(&mut parser), black_box(&lines)))
    });
}

fn bench_type_9(c: &mut Criterion) {
    let mut parser = Parser::default();
    let lines = load_samples("samples_type_9.txt");
    c.bench_function("type_9_sar_aircraft", |b| {
        b.iter(|| parse_lines(black_box(&mut parser), black_box(&lines)))
    });
}

fn bench_type_10(c: &mut Criterion) {
    let mut parser = Parser::default();
    let lines = load_samples("samples_type_10.txt");
    c.bench_function("type_10_utc_date_inquiry", |b| {
        b.iter(|| parse_lines(black_box(&mut parser), black_box(&lines)))
    });
}

fn bench_type_12(c: &mut Criterion) {
    let mut parser = Parser::default();
    let lines = load_samples("samples_type_12.txt");
    c.bench_function("type_12_addressed_safety", |b| {
        b.iter(|| parse_lines(black_box(&mut parser), black_box(&lines)))
    });
}

fn bench_type_14(c: &mut Criterion) {
    let mut parser = Parser::default();
    let lines = load_samples("samples_type_14.txt");
    c.bench_function("type_14_safety_broadcast", |b| {
        b.iter(|| parse_lines(black_box(&mut parser), black_box(&lines)))
    });
}

fn bench_type_15(c: &mut Criterion) {
    let mut parser = Parser::default();
    let lines = load_samples("samples_type_15.txt");
    c.bench_function("type_15_interrogation", |b| {
        b.iter(|| parse_lines(black_box(&mut parser), black_box(&lines)))
    });
}

fn bench_type_16(c: &mut Criterion) {
    let mut parser = Parser::default();
    let lines = load_samples("samples_type_16.txt");
    c.bench_function("type_16_assignment_mode_cmd", |b| {
        b.iter(|| parse_lines(black_box(&mut parser), black_box(&lines)))
    });
}

fn bench_type_18(c: &mut Criterion) {
    let mut parser = Parser::default();
    let lines = load_samples("samples_type_18.txt");
    c.bench_function("type_18_class_b_position", |b| {
        b.iter(|| parse_lines(black_box(&mut parser), black_box(&lines)))
    });
}

fn bench_type_19(c: &mut Criterion) {
    let mut parser = Parser::default();
    let lines = load_samples("samples_type_19.txt");
    c.bench_function("type_19_extended_class_b", |b| {
        b.iter(|| parse_lines(black_box(&mut parser), black_box(&lines)))
    });
}

fn bench_type_20(c: &mut Criterion) {
    let mut parser = Parser::default();
    let lines = load_samples("samples_type_20.txt");
    c.bench_function("type_20_data_link_management", |b| {
        b.iter(|| parse_lines(black_box(&mut parser), black_box(&lines)))
    });
}

fn bench_type_21(c: &mut Criterion) {
    let mut parser = Parser::default();
    let lines = load_samples("samples_type_21.txt");
    c.bench_function("type_21_aid_to_navigation", |b| {
        b.iter(|| parse_lines(black_box(&mut parser), black_box(&lines)))
    });
}

fn bench_type_22(c: &mut Criterion) {
    let mut parser = Parser::default();
    let lines = load_samples("samples_type_22.txt");
    c.bench_function("type_22_channel_management", |b| {
        b.iter(|| parse_lines(black_box(&mut parser), black_box(&lines)))
    });
}

fn bench_type_23(c: &mut Criterion) {
    let mut parser = Parser::default();
    let lines = load_samples("samples_type_23.txt");
    c.bench_function("type_23_group_assignment", |b| {
        b.iter(|| parse_lines(black_box(&mut parser), black_box(&lines)))
    });
}

fn bench_type_24(c: &mut Criterion) {
    let mut parser = Parser::default();
    let lines = load_samples("samples_type_24.txt");
    c.bench_function("type_24_class_b_static_data", |b| {
        b.iter(|| parse_lines(black_box(&mut parser), black_box(&lines)))
    });
}

fn bench_type_25(c: &mut Criterion) {
    let mut parser = Parser::default();
    let lines = load_samples("samples_type_25.txt");
    c.bench_function("type_25_single_slot_binary", |b| {
        b.iter(|| parse_lines(black_box(&mut parser), black_box(&lines)))
    });
}

fn bench_type_27(c: &mut Criterion) {
    let mut parser = Parser::default();
    let lines = load_samples("samples_type_27.txt");
    c.bench_function("type_27_long_range_position", |b| {
        b.iter(|| parse_lines(black_box(&mut parser), black_box(&lines)))
    });
}

criterion_group!(
    snekkja,
    bench_mixed,
    bench_type_1_2_3,
    bench_type_4,
    bench_type_5,
    bench_type_6,
    bench_type_7_13,
    bench_type_8,
    bench_type_9,
    bench_type_10,
    bench_type_12,
    bench_type_14,
    bench_type_15,
    bench_type_16,
    bench_type_18,
    bench_type_19,
    bench_type_20,
    bench_type_21,
    bench_type_22,
    bench_type_23,
    bench_type_24,
    bench_type_25,
    bench_type_27,
);
criterion_main!(snekkja);
