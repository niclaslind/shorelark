use ga::Chromosome;

use crate::*;

pub struct AnimalIndividual {
    fitness: f32,
    chromosome: Chromosome,
}

impl AnimalIndividual {
    pub fn from_animal(animal: &Animal) -> Self {
        Self {
            fitness: animal.satiation as f32,
            chromosome: animal.as_chromosome(),
        }
    }

    pub fn into_animal(self, rng: &mut dyn Rng) -> Animal {
        Animal::from_chromosome(self.chromosome, rng)
    }
}

impl ga::Individual for AnimalIndividual {
    fn create(chromosome: ga::Chromosome) -> Self {
        Self {
            fitness: 0.0,
            chromosome,
        }
    }

    fn chromosome(&self) -> &ga::Chromosome {
        &self.chromosome
    }

    fn fitness(&self) -> f32 {
        self.fitness
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ga::Individual;
    use rand::SeedableRng;
    use rand_chacha::ChaCha8Rng;

    #[test]
    fn from_animal_uses_satiation_as_fitness() {
        let mut rng = ChaCha8Rng::from_seed(Default::default());
        let mut animal = Animal::random(&mut rng);
        animal.satiation = 7;

        let individual = AnimalIndividual::from_animal(&animal);

        assert_eq!(individual.fitness(), 7.0);
    }

    #[test]
    fn from_animal_carries_over_the_chromosome() {
        let mut rng = ChaCha8Rng::from_seed(Default::default());
        let animal = Animal::random(&mut rng);

        let individual = AnimalIndividual::from_animal(&animal);

        assert_eq!(
            individual.chromosome().iter().collect::<Vec<_>>(),
            animal.as_chromosome().iter().collect::<Vec<_>>(),
        );
    }

    #[test]
    fn into_animal_rebuilds_the_brain_from_the_chromosome() {
        let mut rng = ChaCha8Rng::from_seed(Default::default());
        let animal = Animal::random(&mut rng);
        let chromosome = animal.as_chromosome();

        let individual = AnimalIndividual::from_animal(&animal);
        let rebuilt = individual.into_animal(&mut rng);

        assert_eq!(
            rebuilt.as_chromosome().into_iter().collect::<Vec<_>>(),
            chromosome.into_iter().collect::<Vec<_>>(),
        );
    }

    #[test]
    fn create_starts_with_zero_fitness() {
        let mut rng = ChaCha8Rng::from_seed(Default::default());
        let animal = Animal::random(&mut rng);
        let chromosome = animal.as_chromosome();

        let individual = AnimalIndividual::create(chromosome);

        assert_eq!(individual.fitness(), 0.0);
    }
}
