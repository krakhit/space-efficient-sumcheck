use ark_poly::{
    multivariate::{self, SparseTerm, Term},
    DenseMVPolynomial,
};
use ark_std;
use ark_test_curves::bls12_381::Fr as BenchField;
use std::thread;
use std::time::Duration;

use space_efficient_sumcheck::{
    provers::{test_helpers::TestHelperPolynomial, SpaceProver, TimeProver, TradeoffProver},
    Sumcheck,
};

// bench specific stuff
use criterion::{criterion_group, criterion_main, Criterion};
use jemalloc_ctl::{epoch, stats};

#[global_allocator]
static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;

pub type BenchPolynomial = multivariate::SparsePolynomial<BenchField, SparseTerm>;

fn test_terms(num_terms: usize) -> Vec<(BenchField, SparseTerm)> {
    let terms: Vec<(BenchField, SparseTerm)> = vec![
        (
            BenchField::from(4),
            multivariate::SparseTerm::new(vec![(0, 1)]),
        ),
        (
            BenchField::from(7),
            multivariate::SparseTerm::new(vec![(1, 1), (2, 1)]),
        ),
        (
            BenchField::from(2),
            multivariate::SparseTerm::new(vec![(0, 1)]),
        ),
        (
            BenchField::from(13),
            multivariate::SparseTerm::new(vec![(1, 1)]),
        ),
        (
            BenchField::from(27),
            multivariate::SparseTerm::new(vec![(3, 1)]),
        ),
        (
            BenchField::from(29),
            multivariate::SparseTerm::new(vec![(1, 1), (4, 1)]),
        ),
        (
            BenchField::from(18),
            multivariate::SparseTerm::new(vec![(1, 1), (4, 1)]),
        ),
        (
            BenchField::from(18),
            multivariate::SparseTerm::new(vec![(2, 1), (5, 1)]),
        ),
        (
            BenchField::from(18),
            multivariate::SparseTerm::new(vec![(3, 1), (6, 1)]),
        ),
        (
            BenchField::from(18),
            multivariate::SparseTerm::new(vec![(2, 1), (7, 1)]),
        ),
        (
            BenchField::from(18),
            multivariate::SparseTerm::new(vec![(3, 1), (8, 1)]),
        ),
        (
            BenchField::from(18),
            multivariate::SparseTerm::new(vec![(2, 1), (9, 1)]),
        ),
        (
            BenchField::from(18),
            multivariate::SparseTerm::new(vec![(3, 1), (10, 1)]),
        ),
        (
            BenchField::from(18),
            multivariate::SparseTerm::new(vec![(2, 1), (11, 1)]),
        ),
        (
            BenchField::from(18),
            multivariate::SparseTerm::new(vec![(3, 1), (12, 1)]),
        ),
        (
            BenchField::from(18),
            multivariate::SparseTerm::new(vec![(2, 1), (13, 1)]),
        ),
        (
            BenchField::from(18),
            multivariate::SparseTerm::new(vec![(3, 1), (14, 1)]),
        ),
        (
            BenchField::from(18),
            multivariate::SparseTerm::new(vec![(2, 1), (15, 1)]),
        ),
        (
            BenchField::from(18),
            multivariate::SparseTerm::new(vec![(3, 1), (16, 1)]),
        ),
        (
            BenchField::from(18),
            multivariate::SparseTerm::new(vec![(2, 1), (17, 1)]),
        ),
        (
            BenchField::from(18),
            multivariate::SparseTerm::new(vec![(3, 1), (18, 1)]),
        ),
        (
            BenchField::from(18),
            multivariate::SparseTerm::new(vec![(2, 1), (19, 1)]),
        ),
        (
            BenchField::from(18),
            multivariate::SparseTerm::new(vec![(3, 1), (20, 1)]),
        ),
        (
            BenchField::from(18),
            multivariate::SparseTerm::new(vec![(2, 1), (21, 1)]),
        ),
        (
            BenchField::from(18),
            multivariate::SparseTerm::new(vec![(3, 1), (22, 1)]),
        ),
        (
            BenchField::from(18),
            multivariate::SparseTerm::new(vec![(2, 1), (23, 1)]),
        ),
        (
            BenchField::from(18),
            multivariate::SparseTerm::new(vec![(3, 1), (24, 1)]),
        ),
        (
            BenchField::from(18),
            multivariate::SparseTerm::new(vec![(2, 1), (25, 1)]),
        ),
        (
            BenchField::from(18),
            multivariate::SparseTerm::new(vec![(3, 1), (26, 1)]),
        ),
        (
            BenchField::from(18),
            multivariate::SparseTerm::new(vec![(2, 1), (27, 1)]),
        ),
        (
            BenchField::from(18),
            multivariate::SparseTerm::new(vec![(3, 1), (28, 1)]),
        ),
        (
            BenchField::from(18),
            multivariate::SparseTerm::new(vec![(2, 1), (29, 1)]),
        ),
        (
            BenchField::from(18),
            multivariate::SparseTerm::new(vec![(3, 1), (30, 1)]),
        ),
    ];
    return terms[0..num_terms].to_vec();
}

fn test_polynomial(num_terms: usize) -> BenchPolynomial {
    return BenchPolynomial::from_coefficients_vec(num_terms, test_terms(num_terms));
}

// fn run_bench<F: Field + std::convert::From<i32>, P: Prover<F>>(c: &mut Criterion) {
//     let mut rng = ark_std::test_rng();

//     let polynomial = test_polynomial(8);
//     let evaluations = polynomial.to_evaluations();
//     epoch::advance().unwrap();
//     c.bench_function("tradeoff_prover", |b: &mut criterion::Bencher<'_>| {
//         b.iter(|| {
//             let prover = P::new(evaluations.clone(), 4);
//             Sumcheck::prove(prover, &mut rng);
//         });
//     });
//     let allocated = stats::allocated::read().unwrap();
//     let resident = stats::resident::read().unwrap();
//     println!("{} bytes allocated/{} bytes resident", allocated, resident);
//     thread::sleep(Duration::from_secs(10));
// }

fn time_prover_benchmark(c: &mut Criterion) {
    let mut rng = ark_std::test_rng();

    let polynomial = test_polynomial(26);
    let evaluations = polynomial.to_evaluations();
    epoch::advance().unwrap();
    c.bench_function("time_prover", |b: &mut criterion::Bencher<'_>| {
        b.iter(|| {
            let prover = TimeProver::<BenchField>::new(evaluations.clone());
            Sumcheck::prove(prover, &mut rng);
        });
    });
    let allocated = stats::allocated::read().unwrap();
    let resident = stats::resident::read().unwrap();
    println!("{} bytes allocated/{} bytes resident", allocated, resident);
    thread::sleep(Duration::from_secs(10));
}

fn space_prover_benchmark(c: &mut Criterion) {
    let mut rng = ark_std::test_rng();

    let polynomial = test_polynomial(22);
    let evaluations = polynomial.to_evaluations();
    epoch::advance().unwrap();
    c.bench_function("space_prover", |b: &mut criterion::Bencher<'_>| {
        b.iter(|| {
            let prover = SpaceProver::<BenchField>::new(evaluations.clone());
            Sumcheck::prove(prover, &mut rng);
        });
    });
    let allocated = stats::allocated::read().unwrap();
    let resident = stats::resident::read().unwrap();
    println!("{} bytes allocated/{} bytes resident", allocated, resident);
    thread::sleep(Duration::from_secs(10));
}

fn tradeoff_prover_benchmark(c: &mut Criterion) {
    let mut rng = ark_std::test_rng();

    let polynomial = test_polynomial(8);
    let evaluations = polynomial.to_evaluations();
    epoch::advance().unwrap();
    c.bench_function("tradeoff_prover", |b: &mut criterion::Bencher<'_>| {
        b.iter(|| {
            let prover = TradeoffProver::<BenchField>::new(evaluations.clone(), 4);
            Sumcheck::prove(prover, &mut rng);
        });
    });
    let allocated = stats::allocated::read().unwrap();
    let resident = stats::resident::read().unwrap();
    println!("{} bytes allocated/{} bytes resident", allocated, resident);
    thread::sleep(Duration::from_secs(10));
}

criterion_group! {
    name = benches;
    // This can be any expression that returns a `Criterion` object.
    config = Criterion::default().sample_size(10);
    targets = tradeoff_prover_benchmark
}
// criterion_group!(benches, tradeoff_prover_benchmark);
criterion_main!(benches);
