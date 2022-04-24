use criterion::{black_box, criterion_group, criterion_main, Criterion};
use melody_compiler::compiler;

fn criterion_benchmark(criterion: &mut Criterion) {
    let mut benchmark_group = criterion.benchmark_group("compiler");

    benchmark_group.sample_size(10);

    let source = r#"16 of "na";

    2 of match {
      <space>;
      "batman";
    }

    /* 🦇🦸‍♂️ */"#;

    benchmark_group.bench_function("normal (8 lines)", |bencher| {
        bencher.iter(|| compiler(black_box(source)))
    });

    let medium_source = r##"16 of "na";

    2 of match {
      <space>;
      "batman";
    }

    /* 🦇🦸‍♂️ */
    
    "#";
    some of <word>;

    // #melody

    <category::space_separator>;
    <category::other>;

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
    
    <category::letter>;
    <category::currency_symbol>;

    capture patch {
      some of <digit>;
    }
    
    // v1.0.0
    ".";"##;

    let medium_source = format!("{}\n", medium_source);

    let long_source: String = medium_source.repeat(20000);

    benchmark_group.bench_function("long input (1M lines)", |bencher| {
        bencher.iter(|| compiler(black_box(&long_source)))
    });

    let deeply_nested_source = r#"match {
      "a";
      "match {
        "a";
        "match {
          "a";
          "match {
            "a";
            "match {
              "a";
              "match {
                "a";
                "match {
                  "a";
                  "match {
                    "a";
                    "match {
                      "a";
                      "match {
                        "a";
                        "match {
                          "a";
                          "match {
                            "a";
                          }
                        }
                      }
                    }
                  }
                }
              }
            }
          }
        }
      }
    }
    "#;

    benchmark_group.bench_function("deeply nested", |bencher| {
        bencher.iter(|| compiler(black_box(deeply_nested_source)))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
