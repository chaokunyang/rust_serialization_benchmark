use criterion::{black_box, Criterion};
use fory::{Fory, Serializer, ForyDefault};

/// Trait for types that can register themselves and their dependencies with Fory
pub trait ForyRegister {
    fn register_fory_types(fory: &mut Fory);
}

pub fn bench<T>(name: &'static str, c: &mut Criterion, data: &T)
where
    T: Serializer + ForyDefault + ForyRegister + PartialEq,
{
    let mut group = c.benchmark_group(format!("{}/fory", name));

    let mut fory = Fory::default();
    T::register_fory_types(&mut fory);

    group.bench_function("serialize", |b| {
        let mut buffer = Vec::new();
        b.iter(|| {
            buffer.clear();
            black_box(fory.serialize_to(black_box(data), &mut buffer).unwrap());
        })
    });

    let mut encoded = Vec::new();
    fory.serialize_to(data, &mut encoded).unwrap();

    group.bench_function("deserialize", |b| {
        b.iter(|| {
            black_box(fory.deserialize::<T>(black_box(&encoded)).unwrap());
        })
    });

    crate::bench_size(name, "fory", &encoded);

    assert!(fory.deserialize::<T>(&encoded).unwrap() == *data);

    group.finish();
}
