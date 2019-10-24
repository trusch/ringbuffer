#[macro_use]
extern crate criterion;

use criterion::Criterion;

use ringbuffer::RingBuffer;

fn check_if_any_benchmark(c: &mut Criterion) {
    c.bench_function("check_if_any", |b| {
        let mut buf = RingBuffer::new(1024);
        for i in 0..1024 {
            buf.push(i);
        }
        b.iter(|| buf.check_if_any(|_| false));
    });
}

fn for_each_benchmark(c: &mut Criterion) {
    c.bench_function("for_each", |b| {
        let mut buf = RingBuffer::new(1024);
        for i in 0..1024 {
            buf.push(i);
        }
        b.iter(|| buf.for_each(|_| Ok(())));
    });
}

fn contains_benchmark(c: &mut Criterion) {
    c.bench_function("contains", |b| {
        let mut buf = RingBuffer::new(1024);
        for i in 0..1024 {
            buf.push(i);
        }
        b.iter(|| buf.contains(&1024));
    });
}

criterion_group!(
    benches,
    check_if_any_benchmark,
    for_each_benchmark,
    contains_benchmark
);
criterion_main!(benches);
