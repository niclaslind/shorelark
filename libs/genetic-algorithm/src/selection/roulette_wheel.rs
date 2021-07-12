use crate::*;

pub struct RouletteWheelSelection;

impl RouletteWheelSelection {
    pub fn new() -> Self {
        Self
    }
}

impl Default for RouletteWheelSelection {
    fn default() -> Self {
        Self::new()
    }
}

impl SelectionMethod for RouletteWheelSelection {
    fn select<'a, I>(&self, rng: &mut dyn RngCore, population: &'a [I]) -> &'a I
    where
        I: Individual,
    {
        population
            .choose_weighted(rng, |individual| individual.fitness())
            .expect("got an empty population")
    }
}
