use crate::*;

#[derive(Debug, Clone, Serialize)]
pub struct World {
    pub animals: Vec<Animal>,
    pub foods: Vec<Food>,
}

impl From<&sim::World> for World {
    fn from(world: &sim::World) -> Self {
        let animals = world.animals().iter().map(Animal::from).collect();

        let foods = world.foods().iter().map(Food::from).collect();

        Self { animals, foods }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;
    use rand_chacha::ChaCha8Rng;

    #[test]
    fn from_sim_world_converts_all_animals_and_foods() {
        let mut rng = ChaCha8Rng::from_seed(Default::default());
        let sim_world = sim::World::random(&mut rng);

        let world = World::from(&sim_world);

        assert_eq!(world.animals.len(), sim_world.animals().len());
        assert_eq!(world.foods.len(), sim_world.foods().len());
    }
}
