use crate::*;

#[derive(Debug)]
pub struct World {
    crate animals: Vec<Animal>,
    crate foods: Vec<Food>,
}

impl World {
    pub fn random(rng: &mut dyn RngCore) -> Self {
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