pub use self::{animal::*, food::*, world::*};
use lib_simulation as sim;
use rand::{prelude::ThreadRng, thread_rng};
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
        let mut rng = thread_rng();
        let sim = sim::Simulation::random(&mut rng);

        Self { rng, sim }
    }

    pub fn world(&self) -> JsValue {
        let world = World::from(self.sim.world());
        JsValue::from_serde(&world).unwrap()
    }

    pub fn step(&mut self) {
        self.sim.step(&mut self.rng);
    }

    pub fn train(&mut self) -> String {
        let stats = self.sim.train(&mut self.rng);

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
