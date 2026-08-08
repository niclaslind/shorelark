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
    pub fn new(
        num_animals: Option<usize>,
        num_foods: Option<usize>,
        mutation_chance: Option<f32>,
        mutation_coeff: Option<f32>,
        max_speed: Option<f32>,
    ) -> Self {
        let defaults = sim::Config::default();

        let config = sim::Config {
            num_animals: num_animals.unwrap_or(defaults.num_animals),
            num_foods: num_foods.unwrap_or(defaults.num_foods),
            mutation_chance: mutation_chance
                .unwrap_or(defaults.mutation_chance)
                .clamp(0.0, 1.0),
            mutation_coeff: mutation_coeff
                .unwrap_or(defaults.mutation_coeff)
                .clamp(0.0, 1.0),
            max_speed: max_speed.unwrap_or(defaults.max_speed).max(0.0),
        };

        let mut rng = rand::rng();
        let sim = sim::Simulation::random_with_config(&mut rng, config);

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
        Self::new(None, None, None, None, None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_creates_a_simulation_starting_at_generation_zero() {
        let simulation = Simulation::new(None, None, None, None, None);

        assert_eq!(simulation.generation(), 0);
    }

    #[test]
    fn step_does_not_evolve_before_generation_length_is_reached() {
        let mut simulation = Simulation::new(None, None, None, None, None);

        // GENERATION_LENGTH (in lib-simulation) is 2500; stepping fewer
        // times than that should never trigger evolution, and therefore
        // never touch the (wasm-only) JsValue::from_serde path.
        for _ in 0..100 {
            let _ = simulation.step();
        }

        assert_eq!(simulation.generation(), 0);
    }

    #[test]
    fn generation_stats_reflects_the_current_generation_and_fitness_values() {
        let simulation = Simulation::new(None, None, None, None, None);
        let stats = lib_simulation::Statistics::new(&[
            AnimalIndividualStub(1.0),
            AnimalIndividualStub(3.0),
            AnimalIndividualStub(5.0),
        ]);

        let generation_stats = simulation.generation_stats(&stats);

        assert_eq!(generation_stats.generation, simulation.generation());
        assert_eq!(generation_stats.min_fitness, 1.0);
        assert_eq!(generation_stats.max_fitness, 5.0);
        assert_eq!(generation_stats.avg_fitness, 3.0);
    }

    /// Minimal stand-in for `AnimalIndividual` used only to exercise
    /// `Statistics::new` without depending on a full `Animal`.
    struct AnimalIndividualStub(f32);

    impl lib_genetic_algorithm::Individual for AnimalIndividualStub {
        fn create(_chromosome: lib_genetic_algorithm::Chromosome) -> Self {
            unimplemented!()
        }

        fn chromosome(&self) -> &lib_genetic_algorithm::Chromosome {
            unimplemented!()
        }

        fn fitness(&self) -> f32 {
            self.0
        }
    }
}
