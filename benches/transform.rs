use criterion::{black_box, criterion_group, criterion_main, Criterion};
use line_adjustment::transform;

fn get_bench_input() -> String {
    const PATTERN_REPEAT_TIMES: usize = 20000;
    const PATTERN: &str = "Quidni satius sit perpetuam infelicitatem aduocata uirtute sustinere quam infinitis atque inmodicis bonis rumpi";
    return (PATTERN.to_string() + " ").repeat(PATTERN_REPEAT_TIMES);
}

pub fn transform_benchmark(c: &mut Criterion) {
    const LINE_WIDTH: u32 = 13;
    let input = get_bench_input();

    c.bench_function("transform", 
    | b| b.iter(
        || transform(black_box(input.as_str()), black_box(LINE_WIDTH))
    ));
}
criterion_group!(benches, transform_benchmark);
criterion_main!(benches);
