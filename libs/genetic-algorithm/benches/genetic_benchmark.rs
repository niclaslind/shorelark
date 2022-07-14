use criterion::{black_box, criterion_group, criterion_main, Criterion};
use lib_genetic_algorithm::{CrossoverMethod, GaussianMutation, MutationMethod, UniformCrossover};
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;

fn guassian_mutation_benchmark(c: &mut Criterion) {
    let guassian_mutation = GaussianMutation::new(0.5, 0.5);
    let mut child = vec![1.0, 2.0, 3.0, 4.0, 5.0].into_iter().collect();
    let mut rng = ChaCha8Rng::from_seed(Default::default());

    c.bench_function("guassian mutation", |b| {
        b.iter(|| guassian_mutation.mutate(black_box(&mut rng), black_box(&mut child)))
    });
}

fn uniform_crossover_benchmark(c: &mut Criterion) {
    let uniform_crossover = UniformCrossover::default();
    let mut rng = ChaCha8Rng::from_seed(Default::default());
    let parent_a = vec![1.0, 2.0, 3.0, 4.0, 5.0].into_iter().collect();
    let parent_b = vec![5.0, 4.0, 3.0, 2.0, 1.0].into_iter().collect();

    c.bench_function("uniform crossover", |b| {
        b.iter(|| {
            uniform_crossover.crossover(
                black_box(&mut rng),
                black_box(&parent_a),
                black_box(&parent_b),
            )
        })
    });
}

criterion_group!(
    benches,
    guassian_mutation_benchmark,
    uniform_crossover_benchmark,
);
criterion_main!(benches);
