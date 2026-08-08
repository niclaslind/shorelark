pub use self::{animal::*, food::*, world::*};
use lib_simulation as sim;
use rand::rngs::ThreadRng;
use serde::Serialize;
use wasm_bindgen::prelude::*;

mod animal;
mod food;
mod world;

#[derive(Debug, Clone, Serialize)]
pub struct GenerationStats {
    pub generation: usize,
    pub min_fitness: f32,
    pub max_fitness: f32,
    pub avg_fitness: f32,
}

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
    /// Returns structured statistics whenever a generation has just
    /// finished (i.e. the population evolved), or `None` otherwise.
    pub fn step(&mut self) -> JsValue {
        match self.sim.step(&mut self.rng) {
            Some(stats) => JsValue::from_serde(&self.generation_stats(&stats)).unwrap(),
            None => JsValue::NULL,
        }
    }

    pub fn train(&mut self) -> JsValue {
        let stats = self.sim.train(&mut self.rng);

        JsValue::from_serde(&self.generation_stats(&stats)).unwrap()
    }

    /// Current generation number (how many times the population has
    /// evolved so far).
    pub fn generation(&self) -> usize {
        self.sim.generation()
    }

    fn generation_stats(&self, stats: &sim::Statistics) -> GenerationStats {
        GenerationStats {
            generation: self.sim.generation(),
            min_fitness: stats.min_fitness(),
            max_fitness: stats.max_fitness(),
            avg_fitness: stats.avg_fitness(),
        }
    }
}

impl Default for Simulation {
    fn default() -> Self {
        Self::new()
    }
}
