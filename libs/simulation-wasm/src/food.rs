use crate::*;

#[derive(Clone, Debug, Serialize)]
pub struct Food {
    pub x: f32,
    pub y: f32,
}

impl From<&sim::Food> for Food {
    fn from(food: &sim::Food) -> Self {
        Self {
            x: food.position().x,
            y: food.position().y,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;
    use rand_chacha::ChaCha8Rng;

    #[test]
    fn from_sim_food_copies_the_position() {
        let mut rng = ChaCha8Rng::from_seed(Default::default());
        let sim_food = sim::Food::random(&mut rng);

        let food = Food::from(&sim_food);

        assert_eq!(food.x, sim_food.position().x);
        assert_eq!(food.y, sim_food.position().y);
    }
}
