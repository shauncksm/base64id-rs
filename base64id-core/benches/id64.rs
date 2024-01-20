use criterion::{black_box, criterion_group, criterion_main, Criterion};

use base64id_core::base64;

fn encode_i64_bench(c: &mut Criterion) {
    c.bench_function("encode i64 as [char; 11]", |b| {
        b.iter(|| black_box(base64::encode_i64(25519i64)))
    });
}

fn decode_i64_bench(c: &mut Criterion) {
    c.bench_function("decode [char; 11] as i64", |b| {
        b.iter(|| {
            black_box(base64::decode_i64([
                'A', 'A', 'A', 'A', 'A', 'A', 'A', 'A', 'Y', '6', '8',
            ]))
        })
    });
}

criterion_group!(benches, encode_i64_bench, decode_i64_bench,);
criterion_main!(benches);
