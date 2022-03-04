use criterion::{criterion_group, criterion_main, Criterion};
use melody_compiler::compiler;

fn criterion_benchmark(criterion: &mut Criterion) {
    let mut benchmark_group = criterion.benchmark_group("compiler");

    benchmark_group.sample_size(10);

    let normal_source = r#"16 of "na";

    2 of match {
      <space>;
      "batman";
    }

    /* 🦇🦸‍♂️ */"#;

    benchmark_group.bench_function("normal", |bencher| bencher.iter(|| compiler(normal_source)));

    let long_source = r##"16 of "na";

    2 of match {
      <space>;
      "batman";
    }

    /* 🦇🦸‍♂️ */
    
    "#";
    some of <word>;

    // #melody

    some of <word>;
    <space>;
    "1";
    2 of <digit>;
    
    // classname 1xx

    some of match {
      2 of <space>;
    }
    
    some of <char>;
    ";";
    
    // let value = 5;

    option of "v";
    
    capture major {
      some of <digit>;
    }
    
    ".";
    
    capture minor {
      some of <digit>;
    }
    
    ".";
    
    capture patch {
      some of <digit>;
    }
    
    // v1.0.0
    ".";
    ".";"##;

    let heavy_source: String = (0..20000).map(|_| long_source).collect();

    benchmark_group.bench_function("long input", |bencher| {
        bencher.iter(|| compiler(&heavy_source))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
