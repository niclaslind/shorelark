use crate::*;

#[derive(Clone, Debug)]
pub struct Statistics {
    min_fitness: f32,
    max_fitness: f32,
    avg_fitness: f32,
}

impl Statistics {
    pub fn new<I>(population: &[I]) -> Self
    where
        I: Individual,
    {
        assert!(!population.is_empty());

        let mut min_fitness = population[0].fitness();
        let mut max_fitness = min_fitness;
        let mut sum_fitness = 0.0;

        for individual in population {
            let fitness = individual.fitness();

            min_fitness = min_fitness.min(fitness);
            max_fitness = max_fitness.max(fitness);
            sum_fitness += fitness;
        }

        Self {
            min_fitness,
            max_fitness,
            avg_fitness: sum_fitness / (population.len() as f32),
        }
    }

    pub fn min_fitness(&self) -> f32 {
        self.min_fitness
    }

    pub fn max_fitness(&self) -> f32 {
        self.max_fitness
    }

    pub fn avg_fitness(&self) -> f32 {
        self.avg_fitness
    }
}

#[cfg(test)]
mod test {
    use crate::{Individual, Statistics, TestIndividual};

    fn individual(genes: &[f32]) -> TestIndividual {
        let chromosome = genes.iter().cloned().collect();
        TestIndividual::create(chromosome)
    }

    fn population() -> Vec<TestIndividual> {
        vec![
            individual(&[0.0, 0.0, 0.0]), // fitness = 0.0
            individual(&[1.0, 1.0, 1.0]), // fitness = 3.0
            individual(&[1.0, 2.0, 1.0]), // fitness = 4.0
            individual(&[1.0, 2.0, 4.0]), // fitness = 7.0
        ]
    }

    #[test]
    #[should_panic]
    fn test_populations_is_empty() {
        let population: Vec<TestIndividual> = Vec::new();
        let _statistics = Statistics::new(&population);
    }

    #[test]
    fn test_min_fitness() {
        let statistics = Statistics::new(&population());

        assert_eq!(statistics.min_fitness(), 0.0);
    }

    #[test]
    fn test_max_fitness() {
        let statistics = Statistics::new(&population());

        assert_eq!(statistics.max_fitness(), 7.0);
    }

    #[test]
    fn test_avg_fitness() {
        let statistics = Statistics::new(&population());

        assert_eq!(statistics.avg_fitness(), 3.5);
    }
}
