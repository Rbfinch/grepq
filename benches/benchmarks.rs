use criterion::{criterion_group, criterion_main, Criterion};
use grepq::initialise;
use grepq::quality;
use std::hint::black_box;
use std::io::Write;
use tempfile::NamedTempFile;

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("quality_calculation", |b| {
        let quality = vec![b'I'; 1000];
        b.iter(|| quality::average_quality(black_box(&quality), black_box("Phred+33")))
    });

    c.bench_function("iupac_conversion", |b| {
        let pattern = "ACGTYRWSKMBDHVN";
        b.iter(|| initialise::convert_iupac_to_regex(black_box(pattern)))
    });

    c.bench_function("pattern_parsing", |b| {
        let temp_file = NamedTempFile::new().unwrap();
        let patterns = "[AG]AAT[AT]G[AG]CGGGG\n\
                       CCCCG[CT]C[AT]ATT[CT]\n\
                       GG[AG][ACGT]GGC[ACGT]GCAG\n\
                       CTGC[ACGT]GCC[ACTG][CT]CC\n\
                       G[CT][CT]G[CT]CGTCAGC\n\
                       GCTGACG[AG]C[AG][AG]C\n\
                       C[ACG]GC[ACGT]GC[CT]GCGG\n\
                       CCGC[AG]GC[AGCT]GC[CGT]G\n\
                       TAGA[AT]ACCC[ACGT][ACGT]G\n\
                       C[ACGT][ACGT]GGGT[AT]TCTA\n\
                       CGAGCGCAACCC\n\
                       GGGTTGCGCTCG\n\
                       AGG[CT]GGGGA[CT]GA\n\
                       TC[AG]TCCCC[AG]CCT\n\
                       [GC][CT]GGCG[ACGT]ACGGG\n\
                       CCCGT[ACGT]CGCC[AG][GC]\n\
                       GA[AG]GAACCTTAC\n\
                       GTAAGGTTC[CT]TC\n\
                       GTGGTTTAATTC\n\
                       GAATTAAACCAC\n\
                       G[CT]AC[AT]C[AT]CCGCC\n\
                       GGCGG[AT]G[AT]GT[AG]C\n\
                       GC[TG]ACACACG[CT]G\n\
                       C[AG]CGTGTGT[AC]GC\n\
                       G[AC]GGTGAAAT[TG]C\n\
                       G[AC]ATTTCACC[TG]C\n\
                       AT[CT][AC]TGGCTCAG\n\
                       CTGAGCCA[TG][AG]AT\n\
                       AGTC[AG]TAACAAG\n\
                       CTTGTTA[CT]GACT";
        temp_file.as_file().write_all(patterns.as_bytes()).unwrap();
        b.iter(|| initialise::parse_patterns_file(black_box(temp_file.path().to_str().unwrap())))
    });

    c.bench_function("quality_encoding", |b| {
        let quality = vec![b'I'; 10000];
        b.iter(|| quality::average_quality(black_box(&quality), black_box("Phred+33")))
    });
}

criterion_group! {
    name = benches;
    config = Criterion::default().measurement_time(std::time::Duration::from_secs(6)).sample_size(60);
    targets = criterion_benchmark
}
criterion_main!(benches);
