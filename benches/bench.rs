use criterion::{Criterion, criterion_group, criterion_main};
use std::hint::black_box;
use std::path::PathBuf;
use std::process::Command;

fn bench_de_volcano(c: &mut Criterion) {
    let bin = env!("CARGO_BIN_EXE_rsomics-de-volcano");
    let manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let tsv = manifest.join("tests/golden/de.tsv");
    c.bench_function("rsomics-de-volcano golden", |b| {
        b.iter(|| {
            let out = Command::new(black_box(bin))
                .arg(tsv.to_str().unwrap())
                .output()
                .unwrap();
            assert!(out.status.success());
        });
    });
}

criterion_group!(benches, bench_de_volcano);
criterion_main!(benches);
