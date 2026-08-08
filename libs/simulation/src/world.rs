use crate::*;

#[derive(Debug)]
pub struct World {
    pub animals: Vec<Animal>,
    pub foods: Vec<Food>,
}

impl World {
    pub fn random(rng: &mut dyn Rng) -> Self {
        let animals = (0..40).map(|_| Animal::random(rng)).collect();

        let foods = (0..60).map(|_| Food::random(rng)).collect();

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
    fn getters_expose_the_underlying_collections() {
        let mut rng = ChaCha8Rng::from_seed(Default::default());
        let world = World::random(&mut rng);

        assert_eq!(world.animals().len(), world.animals.len());
        assert_eq!(world.foods().len(), world.foods.len());
    }
}
