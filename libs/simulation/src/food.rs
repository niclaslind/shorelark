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
