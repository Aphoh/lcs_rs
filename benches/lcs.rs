use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use lcs_rs::{self, compute, read_file_and_preprocess};

pub fn criterion_benchmark(c: &mut Criterion) {
    let string_files: Vec<String> = (1..11)
        .map(|x| format!("test_files/sample.{}", x))
        .collect();
    let files: Vec<&str> = string_files.iter().map(AsRef::as_ref).collect();
    let mut data: Vec<Vec<u16>> = Vec::with_capacity(files.len());
    for f in &files {
        match read_file_and_preprocess(f) {
            Ok(bstr) => {
                data.push(bstr);
            }
            Err(why) => {
                eprintln!("Error reading file {}: {}", f, why);
            }
        }
    }

    {
        let mut vk = c.benchmark_group("variable k");
        for k in 2..11 {
            let k_files = files.clone();
            let k_data = data.clone();
            vk.bench_with_input(BenchmarkId::from_parameter(k), &k, |b, &k| {
                b.iter(|| compute(&k_files, &k_data, k))
            });
        }
    }
    {
        let mut vn = c.benchmark_group("variable n");
        for n in 2..11 {
            let n_files = &files[0..n];
            let n_data = data[0..n].to_vec();
            vn.bench_with_input(BenchmarkId::from_parameter(n), &n, |b, &_n| {
                b.iter(|| compute(n_files, &n_data, 2))
            });
        }
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
