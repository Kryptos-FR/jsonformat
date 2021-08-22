use criterion::{criterion_group, criterion_main, Criterion};
use jsonformat::{format_json, format_json_buffered, Indentation};
use std::fs;
use std::io::{self, BufReader, BufWriter, Read, Write};

/// You need a json file called massive.json in your project root
fn format_massive_json(file: &str) -> io::Result<String> {
    Ok(format_json(&file, Indentation::Default))
}

fn criterion_benchmark(c: &mut Criterion) {
    let file = fs::read_to_string("massive.json").expect("massive.json file in project directory");

    c.bench_function("Format massive json", |b| {
        b.iter(|| format_massive_json(&file))
    });
}

fn format_massive_json_buffered<R, W>(
    reader: &mut BufReader<R>,
    writer: &mut BufWriter<W>,
) -> io::Result<()>
where
    R: Read,
    W: Write,
{
    format_json_buffered(reader, writer, Indentation::Default).unwrap();
    Ok(())
}

fn criterion_benchmark_buffered(c: &mut Criterion) {
    let file = fs::read_to_string("massive.json").expect("massive.json file in project directory");

    c.bench_function("Format massive json", |b| {
        b.iter(|| {
            let mut reader = BufReader::new(file.as_bytes());
            let mut writer = BufWriter::new(std::io::sink());
            format_massive_json_buffered(&mut reader, &mut writer)
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
