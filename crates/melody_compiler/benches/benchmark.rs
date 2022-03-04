use std::iter::repeat;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
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

    benchmark_group.bench_function("normal (8 lines)", |bencher| {
        bencher.iter(|| compiler(black_box(normal_source)))
    });

    let source = r##"16 of "na";

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
    ".";"##;

    let source = format!("{}\n", source);

    let long_source: String = repeat(source).take(20000).collect();

    benchmark_group.bench_function("long input (1M lines)", |bencher| {
        bencher.iter(|| compiler(black_box(&long_source)))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
