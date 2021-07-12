use crate::*;

#[derive(Debug)]
pub struct Animal {
    crate position: na::Point2<f32>,
    crate rotation: na::Rotation2<f32>,
    crate speed: f32,
    crate eye: Eye,
    crate brain: Brain,

    /// Number of foods eaten by this animal
    crate satiation: usize,
}

impl Animal {
    pub fn random(rng: &mut dyn RngCore) -> Self {
        let eye = Eye::default();

        let brain = Brain::random(rng, &eye);

        Self::new(eye, brain, rng)
    }

    crate fn from_chromosome(chromosome: ga::Chromosome, rng: &mut dyn RngCore) -> Self {
        let eye = Eye::default();
        let brain = Brain::from_chromosome(chromosome, &eye);

        Self::new(eye, brain, rng)
    }

    crate fn as_chromosome(&self) -> ga::Chromosome {
        self.brain.as_chromosome()
    }

    pub fn position(&self) -> na::Point2<f32> {
        self.position
    }

    pub fn rotation(&self) -> na::Rotation2<f32> {
        self.rotation
    }

    fn new(eye: Eye, brain: Brain, rng: &mut dyn RngCore) -> Self {
        Self {
            position: rng.gen(),
            rotation: rng.gen(),
            speed: 0.002,
            eye,
            brain,
            satiation: 0,
        }
    }
}
