use crate::*;


pub trait Individual {
    fn create(chromosome: Chromosome) -> Self;
    fn chromosome(&self) -> &Chromosome;
    fn fitness(&self) -> f32;
}


#[cfg(test)]
#[derive(Clone, Debug, PartialEq)]
pub enum TestIndividual {
    /// For tests that require ccess to chromosome
    WithChromosome { chromosome: Chromosome },

    /// For tests that don't require access to chromosome
    WithFitness { fitness: f32 },
}

#[cfg(test)]
impl TestIndividual {
    pub fn new(fitness: f32) -> Self {
        Self::WithFitness { fitness }
    }
}

#[cfg(test)]
impl Individual for TestIndividual {
    fn chromosome(&self) -> &Chromosome {
        match self {
            Self::WithChromosome { chromosome } => chromosome,

            Self::WithFitness { .. } => {
                panic!("not supported for TestIndividual::WithFitness")
            }
        }
    }

    fn fitness(&self) -> f32 {
        match self {
            Self::WithChromosome { chromosome } => {
                // summing all the genes together give us the fitness
                chromosome.iter().sum()
            }
            Self::WithFitness { fitness } => *fitness,
        }
    }

    fn create(chromosome: Chromosome) -> Self {
        Self::WithChromosome { chromosome }
    }
}