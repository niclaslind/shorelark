use crate::*;

#[derive(Debug)]
pub struct World {
    pub animals: Vec<Animal>,
    pub foods: Vec<Food>,
}

impl World {
    pub fn random(rng: &mut dyn Rng) -> Self {
        Self::random_with(rng, 40, 60)
    }

    pub fn random_with(rng: &mut dyn Rng, num_animals: usize, num_foods: usize) -> Self {
        let animals = (0..num_animals).map(|_| Animal::random(rng)).collect();

        let foods = (0..num_foods).map(|_| Food::random(rng)).collect();

        Self { animals, foods }
    }

    pub fn animals(&self) -> &[Animal] {
        &self.animals
    }

    pub fn foods(&self) -> &[Food] {
        &self.foods
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;
    use rand_chacha::ChaCha8Rng;

    #[test]
    fn random_creates_forty_animals_and_sixty_foods() {
        let mut rng = ChaCha8Rng::from_seed(Default::default());
        let world = World::random(&mut rng);

        assert_eq!(world.animals().len(), 40);
        assert_eq!(world.foods().len(), 60);
    }

    #[test]
    fn random_with_creates_the_requested_counts() {
        let mut rng = ChaCha8Rng::from_seed(Default::default());
        let world = World::random_with(&mut rng, 12, 34);

        assert_eq!(world.animals().len(), 12);
        assert_eq!(world.foods().len(), 34);
    }

    #[test]
    fn getters_expose_the_underlying_collections() {
        let mut rng = ChaCha8Rng::from_seed(Default::default());
        let world = World::random(&mut rng);

        assert_eq!(world.animals().len(), world.animals.len());
        assert_eq!(world.foods().len(), world.foods.len());
    }
}
