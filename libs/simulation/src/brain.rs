use crate::*;

#[derive(Debug)]
pub struct Brain {
    pub nn: nn::Network,
}

impl Brain {
    pub fn random(rng: &mut dyn Rng, eye: &Eye) -> Self {
        Self {
            nn: nn::Network::random(rng, &Self::topology(eye)),
        }
    }

    pub fn from_chromosome(chromosome: ga::Chromosome, eye: &Eye) -> Self {
        Self {
            nn: nn::Network::from_weights(&Self::topology(eye), chromosome),
        }
    }

    pub fn as_chromosome(&self) -> ga::Chromosome {
        self.nn.weights().collect()
    }

    fn topology(eye: &Eye) -> [nn::LayerTopology; 3] {
        [
            nn::LayerTopology {
                neurons: eye.cells(),
            },
            nn::LayerTopology {
                neurons: eye.cells(),
            },
            nn::LayerTopology { neurons: 2 },
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;
    use rand_chacha::ChaCha8Rng;

    #[test]
    fn random_produces_a_network_matching_the_eyes_topology() {
        let mut rng = ChaCha8Rng::from_seed(Default::default());
        let eye = Eye::default();
        let brain = Brain::random(&mut rng, &eye);

        let output = brain.nn.propagate(vec![0.0; eye.cells()]);

        // Output layer always has exactly 2 neurons: speed & rotation.
        assert_eq!(output.len(), 2);
    }

    #[test]
    fn chromosome_roundtrip_preserves_the_weights() {
        let mut rng = ChaCha8Rng::from_seed(Default::default());
        let eye = Eye::default();
        let brain = Brain::random(&mut rng, &eye);

        let chromosome = brain.as_chromosome();
        let rebuilt = Brain::from_chromosome(chromosome.clone(), &eye);

        assert_eq!(
            rebuilt.as_chromosome().into_iter().collect::<Vec<_>>(),
            chromosome.into_iter().collect::<Vec<_>>(),
        );
    }
}
