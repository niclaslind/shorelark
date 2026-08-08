use crate::*;

#[derive(Debug, Clone, Serialize)]
pub struct Animal {
    pub x: f32,
    pub y: f32,
    pub rotation: f32,
}

impl From<&sim::Animal> for Animal {
    fn from(animal: &sim::Animal) -> Self {
        Self {
            x: animal.position().x,
            y: animal.position().y,
            rotation: animal.rotation().angle(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;
    use rand_chacha::ChaCha8Rng;

    #[test]
    fn from_sim_animal_copies_position_and_rotation() {
        let mut rng = ChaCha8Rng::from_seed(Default::default());
        let sim_animal = sim::Animal::random(&mut rng);

        let animal = Animal::from(&sim_animal);

        assert_eq!(animal.x, sim_animal.position().x);
        assert_eq!(animal.y, sim_animal.position().y);
        assert_eq!(animal.rotation, sim_animal.rotation().angle());
    }
}
