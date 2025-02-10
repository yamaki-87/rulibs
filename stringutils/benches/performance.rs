use criterion::{criterion_group, criterion_main, Criterion};
use regex::Regex;


fn remove_whitespace_filter(input:&str) -> String{
    input.chars().filter(|c| !c.is_whitespace()).collect()
}

fn remove_whitespace_regex(input:&str,re:&Regex) ->String{
    re.replace_all(input, "").to_string()
}

fn benchmark_remove_whitespace(c:&mut Criterion){
    let input = "  This is    a test string with   lots of    whitespace \n".repeat(1000);
    let re = Regex::new(r"\s+").unwrap();
    
    c.bench_function("remove_filter", |b|{
        b.iter(|| remove_whitespace_filter(&input));
    });

    c.bench_function("remove_regex", |b|{
        b.iter(||remove_whitespace_regex(&input,&re));
    });
}

criterion_group!(benches,benchmark_remove_whitespace);
criterion_main!(benches);