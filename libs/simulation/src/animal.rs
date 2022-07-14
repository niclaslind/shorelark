use crate::*;

#[derive(Debug)]
pub struct Animal {
    pub position: na::Point2<f32>,
    pub rotation: na::Rotation2<f32>,
    pub speed: f32,
    pub eye: Eye,
    pub brain: Brain,

    /// Number of foods eaten by this animal
    pub satiation: usize,
}

impl Animal {
    pub fn random(rng: &mut dyn RngCore) -> Self {
        let eye = Eye::default();

        let brain = Brain::random(rng, &eye);

        Self::new(eye, brain, rng)
    }

    pub fn from_chromosome(chromosome: ga::Chromosome, rng: &mut dyn RngCore) -> Self {
        let eye = Eye::default();
        let brain = Brain::from_chromosome(chromosome, &eye);

        Self::new(eye, brain, rng)
    }

    pub fn as_chromosome(&self) -> ga::Chromosome {
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
