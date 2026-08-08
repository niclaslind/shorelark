use crate::*;

#[derive(Debug)]
pub struct Food {
    pub position: na::Point2<f32>,
}

impl Food {
    pub fn random(rng: &mut dyn Rng) -> Self {
        Self {
            position: rng.random(),
        }
    }

    pub fn position(&self) -> na::Point2<f32> {
        self.position
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;
    use rand_chacha::ChaCha8Rng;

    #[test]
    fn random_places_food_within_bounds() {
        let mut rng = ChaCha8Rng::from_seed(Default::default());
        let food = Food::random(&mut rng);

        assert!((0.0..=1.0).contains(&food.position.x));
        assert!((0.0..=1.0).contains(&food.position.y));
    }

    #[test]
    fn position_returns_the_stored_position() {
        let mut rng = ChaCha8Rng::from_seed(Default::default());
        let food = Food::random(&mut rng);

        assert_eq!(food.position(), food.position);
    }
}
