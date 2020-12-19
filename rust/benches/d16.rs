use aoc2020::d16::Field;
use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use std::collections::HashSet;
use std::iter::FromIterator;

/// Fields from my day 16 input
const FIELDS: [Field; 20] = [
    Field {
        name: "departure location",
        r1: 45..=309,
        r2: 320..=962,
    },
    Field {
        name: "departure station",
        r1: 27..=873,
        r2: 895..=952,
    },
    Field {
        name: "departure platform",
        r1: 45..=675,
        r2: 687..=962,
    },
    Field {
        name: "departure track",
        r1: 42..=142,
        r2: 164..=962,
    },
    Field {
        name: "departure date",
        r1: 38..=433,
        r2: 447..=963,
    },
    Field {
        name: "departure time",
        r1: 39..=703,
        r2: 709..=952,
    },
    Field {
        name: "arrival location",
        r1: 34..=362,
        r2: 383..=963,
    },
    Field {
        name: "arrival station",
        r1: 26..=921,
        r2: 934..=954,
    },
    Field {
        name: "arrival platform",
        r1: 38..=456,
        r2: 480..=968,
    },
    Field {
        name: "arrival track",
        r1: 42..=295,
        r2: 310..=956,
    },
    Field {
        name: "class",
        r1: 29..=544,
        r2: 550..=950,
    },
    Field {
        name: "duration",
        r1: 44..=725,
        r2: 749..=963,
    },
    Field {
        name: "price",
        r1: 37..=494,
        r2: 509..=957,
    },
    Field {
        name: "route",
        r1: 25..=170,
        r2: 179..=966,
    },
    Field {
        name: "row",
        r1: 32..=789,
        r2: 795..=955,
    },
    Field {
        name: "seat",
        r1: 29..=98,
        r2: 122..=967,
    },
    Field {
        name: "train",
        r1: 45..=403,
        r2: 418..=956,
    },
    Field {
        name: "type",
        r1: 36..=81,
        r2: 92..=959,
    },
    Field {
        name: "wagon",
        r1: 25..=686,
        r2: 692..=955,
    },
    Field {
        name: "zone",
        r1: 37..=338,
        r2: 353..=960,
    },
];

pub fn bench_membership_test(c: &mut Criterion) {
    let mut group = c.benchmark_group("Vec vs HashSet membership test");

    let fields_vec: Vec<&Field> = Vec::from_iter(&FIELDS);
    let fields_set: HashSet<&Field> = HashSet::from_iter(&FIELDS);
    let size = FIELDS.len();

    group.bench_with_input(BenchmarkId::new("Vec", size), &FIELDS, |b, to_find| {
        b.iter(|| to_find.iter().map(|f| fields_vec.contains(&f)));
    });

    group.bench_with_input(BenchmarkId::new("HashSet", size), &FIELDS, |b, to_find| {
        b.iter(|| to_find.iter().map(|f| fields_set.contains(f)));
    });
}

criterion_group!(benches, bench_membership_test);
criterion_main!(benches);
