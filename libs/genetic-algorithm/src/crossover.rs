pub use uniform::*;

use crate::*;

mod uniform;

pub trait CrossoverMethod {
    fn crossover(
        &self,
        rng: &mut dyn RngCore,
        parent_a: &Chromosome,
        parent_b: &Chromosome,
    ) -> Chromosome;
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;
    use rand_chacha::ChaCha8Rng;

    #[test]
    fn test_crossover() {
        let mut rng = ChaCha8Rng::from_seed(Default::default());
        let parent_a: Chromosome = (1..=100).map(|n| n as f32).collect();
        let parent_b: Chromosome = (1..=100).map(|n| -n as f32).collect();

        let child = UniformCrossover.crossover(&mut rng, &parent_a, &parent_b);

        // Numbers of genes different between `child` and `parent_b`
        let diff_a = child
            .iter()
            .zip(parent_a)
            .filter(|(c, p)| (*c - p).abs() > f32::EPSILON)
            .count();
        let diff_b = child
            .iter()
            .zip(parent_b)
            .filter(|(c, p)| (*c - p).abs() > f32::EPSILON)
            .count();

        assert_eq!(diff_a, 49);
        assert_eq!(diff_b, 51);
    }
}
