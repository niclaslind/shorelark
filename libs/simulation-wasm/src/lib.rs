pub use self::{animal::*, food::*, world::*};
use lib_simulation as sim;
use rand::rngs::ThreadRng;
use serde::Serialize;
use wasm_bindgen::prelude::*;

mod animal;
mod food;
mod world;

#[wasm_bindgen]
pub struct Simulation {
    rng: ThreadRng,
    sim: sim::Simulation,
}

#[wasm_bindgen]
impl Simulation {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        let mut rng = rand::rng();
        let sim = sim::Simulation::random(&mut rng);

        Self { rng, sim }
    }

    pub fn world(&self) -> JsValue {
        let world = World::from(self.sim.world());
        JsValue::from_serde(&world).unwrap()
    }

    /// Advances the simulation by a single step.
    ///
    /// Returns a formatted statistics string whenever a generation has just
    /// finished (i.e. the population evolved), or `None` otherwise.
    pub fn step(&mut self) -> Option<String> {
        self.sim.step(&mut self.rng).map(|stats| Self::format_stats(&stats))
    }

    pub fn train(&mut self) -> String {
        let stats = self.sim.train(&mut self.rng);

        Self::format_stats(&stats)
    }

    /// Current generation number (how many times the population has
    /// evolved so far).
    pub fn generation(&self) -> usize {
        self.sim.generation()
    }

    fn format_stats(stats: &sim::Statistics) -> String {
        format!(
            "min={:.2}, max={:.2}, avg={:.2}",
            stats.min_fitness(),
            stats.max_fitness(),
            stats.avg_fitness(),
        )
    }
}

impl Default for Simulation {
    fn default() -> Self {
        Self::new()
    }
}
