use criterion::{Criterion, criterion_group, criterion_main};
use grep::{Args, grep};
use std::{fs::File, hint::black_box, io::BufReader};
fn bench_grep(c: &mut Criterion) {
    let args = Args::default();
    let file = File::open(&args.files[0]).unwrap();
    c.bench_function("grep", |b| {
        b.iter(|| {
            let reader = black_box(BufReader::new(&file));
            grep(reader, &args);
        })
    });
}
criterion_group!(bench, bench_grep);
criterion_main!(bench);
