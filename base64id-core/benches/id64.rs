use criterion::{black_box, criterion_group, criterion_main, Criterion};

use base64id_core::Id64;

fn encode_u64_bench(c: &mut Criterion) {
    c.bench_function("encode u64 as Id64", |b| {
        b.iter(|| black_box(Id64::from(25519u64)))
    });
}

fn decode_u64_bench(c: &mut Criterion) {
    let id: Id64 = Id64::from(25519u64);
    c.bench_function("decode Id64 as u64", |b| {
        b.iter(|| black_box(u64::from(id)))
    });
}

fn encode_i64_bench(c: &mut Criterion) {
    c.bench_function("encode i64 as Id64", |b| {
        b.iter(|| black_box(Id64::from(25519i64)))
    });
}

fn decode_i64_bench(c: &mut Criterion) {
    let id: Id64 = Id64::from(25519i64);
    c.bench_function("decode Id64 as i64", |b| {
        b.iter(|| black_box(u64::from(id)))
    });
}

criterion_group!(
    benches,
    encode_u64_bench,
    decode_u64_bench,
    encode_i64_bench,
    decode_i64_bench,
);
criterion_main!(benches);
