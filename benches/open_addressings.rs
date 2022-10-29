use all_of_hashtable::open_addressing::{
    FcfsDoubleHashing, FcfsLinearProbing, FcfsQuadraticProbing, LcfsLinearProbing,
    OpenAddressingHashTable,
};
use criterion::{criterion_group, criterion_main, Criterion, SamplingMode, Throughput};
use std::time::Duration;

use crate::util::*;

mod util;

const MAP_ALREADY_INSERTED: u64 = 1_000_000;
const MAP_TOTAL_OPS: usize = 1_000_000;

const OPS_RATE: [(usize, usize, usize); 7] = [
    (100, 0, 0),
    (0, 100, 0),
    (0, 0, 100),
    (5, 90, 5),
    (30, 50, 20),
    (40, 20, 40),
    (50, 0, 50),
];

fn bench_vs_btreemap(c: &mut Criterion) {
    for (insert, lookup, remove) in OPS_RATE {
        let logs = fuzz_logs(
            300,
            MAP_ALREADY_INSERTED,
            MAP_TOTAL_OPS * insert / 100,
            MAP_TOTAL_OPS * lookup / 100,
            MAP_TOTAL_OPS * remove / 100,
        );

        let mut group = c.benchmark_group(format!(
            "Inserted {:+e}, Ops (I: {}%, L: {}%, R: {}%, total: {:+e})",
            MAP_ALREADY_INSERTED, insert, lookup, remove, MAP_TOTAL_OPS
        ));
        group.measurement_time(Duration::from_secs(15));
        group.sampling_mode(SamplingMode::Flat);
        group.sample_size(20);
        group.throughput(Throughput::Elements(MAP_TOTAL_OPS as u64));

        bench_logs_hashmap(logs.clone(), &mut group);
        bench_logs_sequential_map::<OpenAddressingHashTable<_, _, FcfsLinearProbing>>(
            "FcfsLinearProbing",
            logs.clone(),
            &mut group,
        );
        bench_logs_sequential_map::<OpenAddressingHashTable<_, _, FcfsQuadraticProbing>>(
            "FcfsQuadraticProbing",
            logs.clone(),
            &mut group,
        );
        bench_logs_sequential_map::<OpenAddressingHashTable<_, _, FcfsDoubleHashing>>(
            "FcfsDoubleHashing",
            logs.clone(),
            &mut group,
        );
        bench_logs_sequential_map::<OpenAddressingHashTable<_, _, LcfsLinearProbing>>(
            "LcfsLinearProbing",
            logs.clone(),
            &mut group,
        );
    }
}

criterion_group!(bench, bench_vs_btreemap);
criterion_main! {
    bench,
}
