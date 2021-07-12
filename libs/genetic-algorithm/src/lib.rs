#![feature(min_type_alias_impl_trait)]
#![feature(crate_visibility_modifier)]

use std::{iter::FromIterator, ops::Index};

pub use self::{
    chromosome::*, crossover::*, individual::*, mutation::*, selection::*, statistics::*,
};

use rand::{seq::SliceRandom, Rng, RngCore};

mod chromosome;
mod crossover;
mod individual;
mod mutation;
mod selection;
mod statistics;

pub struct GeneticAlgorigthm<S> {
    selection_method: S,
    crossover_method: Box<dyn CrossoverMethod>,
    mutation_method: Box<dyn MutationMethod>,
}

impl<S> GeneticAlgorigthm<S>
where
    S: SelectionMethod,
{
    pub fn new(
        selection_method: S,
        crossover_method: impl CrossoverMethod + 'static,
        mutation_method: impl MutationMethod + 'static,
    ) -> Self {
        Self {
            selection_method,
            crossover_method: Box::new(crossover_method),
            mutation_method: Box::new(mutation_method),
        }
    }

    pub fn evolve<I>(&self, rng: &mut dyn RngCore, population: &[I]) -> (Vec<I>, Statistics)
    where
        I: Individual,
    {
        assert!(!population.is_empty());

        let new_population = (0..population.len())
            .map(|_| {
                let parent_a = self.selection_method.select(rng, population).chromosome();

                let parent_b = self.selection_method.select(rng, population).chromosome();

                let mut child = self.crossover_method.crossover(rng, parent_a, parent_b);

                self.mutation_method.mutate(rng, &mut child);

                I::create(child)
            })
            .collect();

        let stats = Statistics::new(population);

        (new_population, stats)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use rand::SeedableRng;
    use rand_chacha::ChaCha8Rng;

    use super::*;

    fn chromosome() -> Chromosome {
        Chromosome {
            genes: vec![3.0, 1.0, 2.0],
        }
    }

    fn actual(chance: f32, coeff: f32) -> Vec<f32> {
        let mut child = vec![1.0, 2.0, 3.0, 4.0, 5.0].into_iter().collect();

        let mut rng = ChaCha8Rng::from_seed(Default::default());

        GaussianMutation::new(chance, coeff).mutate(&mut rng, &mut child);
        child.into_iter().collect()
    }

    mod given_zero_change {
        fn actual(coeff: f32) -> Vec<f32> {
            super::actual(0.0, coeff)
        }

        mod and_zero_coefficient {
            use super::*;

            #[test]
            fn does_not_change_the_original_chromosome() {
                let actual = actual(0.0);
                let expected = vec![1.0, 2.0, 3.0, 4.0, 5.0];

                approx::assert_relative_eq!(actual.as_slice(), expected.as_slice());
            }
        }

        mod and_nonzero_coefficient {
            use super::*;
            #[test]
            fn does_not_change_the_original_chromosome() {
                let actual = actual(0.5);
                let expected = vec![1.0, 2.0, 3.0, 4.0, 5.0];

                approx::assert_relative_eq!(actual.as_slice(), expected.as_slice());
            }
        }
    }

    mod given_fifty_fifty_chance {
        fn actual(coeff: f32) -> Vec<f32> {
            super::actual(0.5, coeff)
        }

        mod and_zero_coefficient {
            use super::*;

            #[test]
            fn does_not_change_the_original_chromosome() {
                let actual = actual(0.0);
                let expected = vec![1.0, 2.0, 3.0, 4.0, 5.0];

                approx::assert_relative_eq!(actual.as_slice(), expected.as_slice());
            }
        }

        mod and_nonzero_coefficient {
            use super::*;

            #[test]
            fn slightly_changes_the_original_chromosome() {
                let actual = actual(0.5);
                let expected = vec![1.0, 1.7756249, 3.0, 4.1596804, 5.0];

                approx::assert_relative_eq!(actual.as_slice(), expected.as_slice());
            }
        }
    }

    mod given_max_chance {
        fn actual(coeff: f32) -> Vec<f32> {
            super::actual(1.0, coeff)
        }

        mod and_zero_coefficient {
            use super::*;
            #[test]
            fn does_not_change_the_original_chromosome() {
                let actual = actual(0.0);
                let expected = vec![1.0, 2.0, 3.0, 4.0, 5.0];

                approx::assert_relative_eq!(actual.as_slice(), expected.as_slice());
            }
        }

        mod and_nonzero_coefficient {
            use super::*;
            #[test]
            fn entirly_changes_the_original_chromosome() {
                let actual = actual(0.5);
                let expected = vec![1.4545316, 2.1162078, 2.7756248, 3.9505124, 4.638691];

                approx::assert_relative_eq!(actual.as_slice(), expected.as_slice());
            }
        }
    }

    mod len {
        use super::*;

        #[test]
        fn test() {
            assert_eq!(chromosome().len(), 3);
        }
    }

    mod iter {
        use super::*;

        #[test]
        fn test() {
            let chromosome = chromosome();
            let genes: Vec<_> = chromosome.iter().collect();

            assert_eq!(genes.len(), 3);
            approx::assert_relative_eq!(genes[0], &3.0);
            approx::assert_relative_eq!(genes[1], &1.0);
            approx::assert_relative_eq!(genes[2], &2.0);
        }
    }

    mod iter_mut {
        use super::*;

        #[test]
        fn test() {
            let mut chromosome = chromosome();

            chromosome.iter_mut().for_each(|gene| {
                *gene *= 10.0;
            });

            let genes: Vec<_> = chromosome.iter().collect();

            assert_eq!(genes.len(), 3);
            approx::assert_relative_eq!(genes[0], &30.0);
            approx::assert_relative_eq!(genes[1], &10.0);
            approx::assert_relative_eq!(genes[2], &20.0);
        }
    }

    mod index {
        use super::*;

        #[test]
        fn test() {
            let chromesome = Chromosome {
                genes: vec![3.0, 1.0, 2.0],
            };

            approx::assert_relative_eq!(chromesome[0], 3.0);
            approx::assert_relative_eq!(chromesome[1], 1.0);
            approx::assert_relative_eq!(chromesome[2], 2.0);
        }
    }

    mod from_iterator {
        use std::vec;

        use super::*;

        #[test]
        fn test() {
            let chromosome: Chromosome = vec![3.0, 1.0, 2.0].into_iter().collect();

            approx::assert_relative_eq!(chromosome[0], 3.0);
            approx::assert_relative_eq!(chromosome[1], 1.0);
            approx::assert_relative_eq!(chromosome[2], 2.0);
        }
    }

    mod into_iterator {
        use std::vec;

        use super::*;

        #[test]
        fn test() {
            let chromosome = Chromosome {
                genes: vec![3.0, 1.0, 2.0],
            };

            let genes: Vec<_> = chromosome.into_iter().collect();

            assert_eq!(genes.len(), 3);
            approx::assert_relative_eq!(genes[0], 3.0);
            approx::assert_relative_eq!(genes[1], 1.0);
            approx::assert_relative_eq!(genes[2], 2.0);
        }
    }

    #[test]
    fn test_crossover() {
        let mut rng = ChaCha8Rng::from_seed(Default::default());
        let parent_a: Chromosome = (1..=100).map(|n| n as f32).collect();
        let parent_b: Chromosome = (1..=100).map(|n| -n as f32).collect();

        let child = UniformCrossover::default().crossover(&mut rng, &parent_a, &parent_b);

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

    #[test]
    fn test() {
        let method = RouletteWheelSelection::new();
        let mut rng = ChaCha8Rng::from_seed(Default::default());

        let population = vec![
            TestIndividual::new(2.0),
            TestIndividual::new(1.0),
            TestIndividual::new(4.0),
            TestIndividual::new(3.0),
        ];

        let actual_histogram: BTreeMap<i32, _> = (0..1000)
            .map(|_| method.select(&mut rng, &population))
            .fold(Default::default(), |mut histogram, individual| {
                *histogram.entry(individual.fitness() as _).or_default() += 1;

                histogram
            });

        let expected_histogram = maplit::btreemap! {
            // fitness => how many times this fitness has been chosen
            1 => 98,
            2 => 202,
            3 => 278,
            4 => 422,
        };

        assert_eq!(actual_histogram, expected_histogram);
    }
}

#[cfg(test)]
mod test {
    use rand::SeedableRng;
    use rand_chacha::ChaCha8Rng;

    use super::*;

    fn individual(genes: &[f32]) -> TestIndividual {
        let chromosome = genes.iter().cloned().collect();
        TestIndividual::create(chromosome)
    }

    #[test]
    fn test() {
        let mut rng = ChaCha8Rng::from_seed(Default::default());

        let ga = GeneticAlgorigthm::new(
            RouletteWheelSelection::new(),
            UniformCrossover::default(),
            GaussianMutation::new(0.5, 0.5),
        );

        let mut population = vec![
            individual(&[0.0, 0.0, 0.0]), // fitness = 0.0
            individual(&[1.0, 1.0, 1.0]), // fitness = 3.0
            individual(&[1.0, 2.0, 1.0]), // fitness = 4.0
            individual(&[1.0, 2.0, 4.0]), // fitness = 7.0
        ];

        // We're running `.evolve()` a few times, so that the
        // differences between initial and output population are
        // easier to spot.
        //
        // No particilar reason for a number of 10 - this test would
        // be fine for 5, 20 or even 1000 generations; te only thing
        // that'd change is the *magnitude* of difference between
        // initial and output population
        for _ in 0..10 {
            population = ga.evolve(&mut rng, &population).0;
        }

        let expected_population = vec![
            individual(&[0.447_694_9, 2.0648358, 4.3058133]),
            individual(&[1.212_686_7, 1.5538777, 2.886_911]),
            individual(&[1.061_767_8, 2.265_739, 4.428_764]),
            individual(&[0.95909685, 2.4618788, 4.024_733]),
        ];

        assert_eq!(population, expected_population);
    }
}
