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
    pub fn random(rng: &mut dyn Rng) -> Self {
        let eye = Eye::default();

        let brain = Brain::random(rng, &eye);

        Self::new(eye, brain, rng)
    }

    pub fn from_chromosome(chromosome: ga::Chromosome, rng: &mut dyn Rng) -> Self {
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

    fn new(eye: Eye, brain: Brain, rng: &mut dyn Rng) -> Self {
        Self {
            position: rng.random(),
            rotation: rng.random(),
            speed: 0.002,
            eye,
            brain,
            satiation: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;
    use rand_chacha::ChaCha8Rng;

    #[test]
    fn random_creates_an_animal_with_default_speed_and_no_satiation() {
        let mut rng = ChaCha8Rng::from_seed(Default::default());
        let animal = Animal::random(&mut rng);

        assert_eq!(animal.speed, 0.002);
        assert_eq!(animal.satiation, 0);
    }

    #[test]
    fn position_and_rotation_getters_match_the_fields() {
        let mut rng = ChaCha8Rng::from_seed(Default::default());
        let animal = Animal::random(&mut rng);

        assert_eq!(animal.position(), animal.position);
        assert_eq!(animal.rotation(), animal.rotation);
    }

    #[test]
    fn chromosome_roundtrip_preserves_the_brain() {
        let mut rng = ChaCha8Rng::from_seed(Default::default());
        let animal = Animal::random(&mut rng);

        let chromosome = animal.as_chromosome();
        let reconstructed = Animal::from_chromosome(chromosome.clone(), &mut rng);

        assert_eq!(
            reconstructed
                .as_chromosome()
                .into_iter()
                .collect::<Vec<_>>(),
            chromosome.into_iter().collect::<Vec<_>>(),
        );
    }
}
