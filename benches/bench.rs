use criterion::{black_box, criterion_group, criterion_main, Criterion};

use slider_solver::{parse_board, solve};

const MEDIUM_INPUT: &str = "
######
#1AA2#
#1AA2#
#4335#
#4675#
#8  9#
######";

const MEDIUM_TARGET: &str = "
######
#    #
#    #
#    #
# AA #
# AA #
######";

const SIMPLE_INPUT: &str = "
######
#AA11#
#AA22#
#34  #
#5677#
#5688#
######
";
const SIMPLE_TARGET: &str = "
######
#    #
#    #
#    #
#AA  #
#AA  #
######";

const HARDER_INPUT: &str = "
######
#MAAN#
#MAAN#
#OWWP#
#ObcP#
#a  d#
######";

const HARDER_TARGET: &str = "
######
# MN #
#OMNP#
#OWWP#
#aAAb#
#cAAd#
######";

fn criterion_bench(c: &mut Criterion) {
    c.bench_function("simple", |b| {
        let input = parse_board(SIMPLE_INPUT);
        let target = parse_board(SIMPLE_TARGET);
        b.iter(|| {
            solve(black_box(&input), black_box(&target));
        })
    });

    c.bench_function("medium", |b| {
        let input = parse_board(MEDIUM_INPUT);
        let target = parse_board(MEDIUM_TARGET);
        b.iter(|| {
            solve(black_box(&input), black_box(&target));
        })
    });

    c.bench_function("harder", |b| {
        let input = parse_board(HARDER_INPUT);
        let target = parse_board(HARDER_TARGET);
        b.iter(|| {
            solve(black_box(&input), black_box(&target));
        })
    });
}

criterion_group!(benches, criterion_bench);
criterion_main!(benches);
