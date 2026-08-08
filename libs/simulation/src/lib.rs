pub use self::{animal::*, animal_individual::*, brain::*, eye::*, food::*, world::*};
use lib_genetic_algorithm as ga;
pub use lib_genetic_algorithm::Statistics;
use lib_neural_network as nn;
use nalgebra as na;
use rand::{Rng, RngExt};

mod animal;
mod animal_individual;
mod brain;
mod eye;
mod food;
mod world;

// FRAC_PI_2 = PI / 2.0; a convenient shortcut
use std::f32::consts::FRAC_PI_2;

/// Minimum speed of a bird.
///
/// Keeping it above zero prevents birds from getting stuck in one place.
const SPEED_MIN: f32 = 0.001;

/// Maximum speed of a bird.
///
/// Keeping it "sane" prevents birds from accelerating up to infinity,
/// which makes the simulation... unrealistic :-)
const SPEED_MAX: f32 = 0.005;

/// Speed acceleration; determines how much the brain can affect bird's
/// speed during one step.
///
/// Assuming our bird is currently flying with speed=0.5, when the brain
/// yells "stop flying!", a SPEED_ACCEL of:
///
/// - 0.1 = makes it take 5 steps ("5 seconds") for the bird to actually slow down to SPEED_MIN,
///
/// - 0.5 = makes it take 1 step for the bird to slow down to SPEED_MIN.
///
/// This improves simulation faithfulness, because - as in real life -
/// it's not possible to increase speed from 1km/h to 50km/h in one
/// instant, even if your brain very much wants to.
const SPEED_ACCEL: f32 = 0.2;

/// Ditto, but for rotation:
///
/// - 2 * PI = it takes one step for the bird to do a 360° rotation,
/// - PI = it takes two steps for the bird to do a 360° rotation,
///
/// I've chosen PI/2, because - as our motto goes - this value seems
/// to play nice.
const ROTATION_ACCEL: f32 = FRAC_PI_2;

/// How much `.step()`-s have to occur before we push data into the
/// genetic algorithm.
///
/// Value that's too low might prevent the birds from learning, while
/// a value that's too high will make the evolution unnecessarily
/// slower.
///
/// You can treat this number as "for how many steps each bird gets
/// to live"; 2500 was chosen with a fair dice roll.
const GENERATION_LENGTH: usize = 2500;

/// Tunable knobs affecting how a [`Simulation`] is set up.
///
/// All fields have sane defaults (see [`Config::default`]) matching the
/// values this simulation originally shipped with.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Config {
    /// How many animals (birds) populate the world.
    pub num_animals: usize,

    /// How many foods populate the world.
    pub num_foods: usize,

    /// Probability of a single gene mutating during evolution.
    ///
    /// - 0.0 = no genes will be touched
    /// - 1.0 = all genes will be touched
    pub mutation_chance: f32,

    /// Magnitude of a mutation, when it happens.
    ///
    /// - 0.0 = touched genes will not be modified
    /// - 1.0 = touched genes will be += or -= by at most 3.0
    pub mutation_coeff: f32,

    /// Maximum speed a bird can reach.
    pub max_speed: f32,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            num_animals: 40,
            num_foods: 60,
            mutation_chance: 0.01,
            mutation_coeff: 0.3,
            max_speed: SPEED_MAX,
        }
    }
}

pub struct Simulation {
    world: World,
    ga: ga::GeneticAlgorithm<ga::RouletteWheelSelection>,
    age: usize,
    generation: usize,
    max_speed: f32,
}

impl Simulation {
    pub fn random(rng: &mut dyn Rng) -> Self {
        Self::random_with_config(rng, Config::default())
    }

    pub fn random_with_config(rng: &mut dyn Rng, config: Config) -> Self {
        let world = World::random_with(rng, config.num_animals, config.num_foods);

        let ga = ga::GeneticAlgorithm::new(
            ga::RouletteWheelSelection::new(),
            ga::UniformCrossover,
            ga::GaussianMutation::new(config.mutation_chance, config.mutation_coeff),
        );

        Self {
            world,
            ga,
            age: 0,
            generation: 0,
            max_speed: config.max_speed,
        }
    }

    pub fn world(&self) -> &World {
        &self.world
    }

    /// Returns how many full generations have been completed so far.
    pub fn generation(&self) -> usize {
        self.generation
    }

    /// Performs a single step - a single second, so to say - of our simulation
    pub fn step(&mut self, rng: &mut dyn Rng) -> Option<ga::Statistics> {
        self.process_collisions(rng);
        self.process_brains();
        self.process_movements();

        self.age += 1;

        if self.age > GENERATION_LENGTH {
            Some(self.evolve(rng))
        } else {
            None
        }
    }

    /// Fast-forwards 'till the end if the current generation.
    pub fn train(&mut self, rng: &mut dyn Rng) -> ga::Statistics {
        loop {
            if let Some(summary) = self.step(rng) {
                return summary;
            }
        }
    }

    fn process_collisions(&mut self, rng: &mut dyn Rng) {
        for animal in &mut self.world.animals {
            for food in &mut self.world.foods {
                let distance = na::distance(&animal.position, &food.position);

                if distance <= 0.01 {
                    animal.satiation += 1;
                    food.position = rng.random();
                }
            }
        }
    }

    fn process_brains(&mut self) {
        for animal in &mut self.world.animals {
            let vision =
                animal
                    .eye
                    .process_vision(animal.position, animal.rotation, &self.world.foods);

            let response = animal.brain.nn.propagate(vision);

            let speed = response[0].clamp(-SPEED_ACCEL, SPEED_ACCEL);

            let rotation = response[1].clamp(-ROTATION_ACCEL, ROTATION_ACCEL);

            animal.speed = (animal.speed + speed).clamp(SPEED_MIN, self.max_speed);

            animal.rotation = na::Rotation2::new(animal.rotation.angle() + rotation);
        }
    }

    fn process_movements(&mut self) {
        for animal in &mut self.world.animals {
            animal.position += animal.rotation * na::Vector2::new(animal.speed, 0.0);

            animal.position.x = na::wrap(animal.position.x, 0.0, 1.0);
            animal.position.y = na::wrap(animal.position.y, 0.0, 1.0);
        }
    }

    fn evolve(&mut self, rng: &mut dyn Rng) -> ga::Statistics {
        self.age = 0;
        self.generation += 1;

        // Step 1: Prepeare birdies to be sent into genetic algorithm
        let current_population: Vec<_> = self
            .world
            .animals
            .iter()
            .map(AnimalIndividual::from_animal)
            .collect();

        // Step 2: Evolve birdies
        let (evoloved_population, stats) = self.ga.evolve(rng, &current_population);

        // Step 3: Bring birdies back from the genetic algorithm
        self.world.animals = evoloved_population
            .into_iter()
            .map(|individual| individual.into_animal(rng))
            .collect();

        // Step 4: Restart foods
        //
        // (this is not strictly necessary, but it allows to easily spot
        // when the evolution happens - so it's more of a UI thing. )
        for food in &mut self.world.foods {
            food.position = rng.random();
        }
        stats
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;
    use rand_chacha::ChaCha8Rng;

    #[test]
    fn random_creates_a_world_with_forty_animals_and_sixty_foods() {
        let mut rng = ChaCha8Rng::from_seed(Default::default());
        let simulation = Simulation::random(&mut rng);

        assert_eq!(simulation.world().animals().len(), 40);
        assert_eq!(simulation.world().foods().len(), 60);
    }

    #[test]
    fn generation_starts_at_zero() {
        let mut rng = ChaCha8Rng::from_seed(Default::default());
        let simulation = Simulation::random(&mut rng);

        assert_eq!(simulation.generation(), 0);
        assert_eq!(simulation.age, 0);
    }

    #[test]
    fn step_returns_none_until_a_generation_completes() {
        let mut rng = ChaCha8Rng::from_seed(Default::default());
        let mut simulation = Simulation::random(&mut rng);

        for _ in 0..GENERATION_LENGTH {
            assert!(simulation.step(&mut rng).is_none());
        }

        assert!(simulation.step(&mut rng).is_some());
        assert_eq!(simulation.generation(), 1);
    }

    #[test]
    fn train_advances_a_full_generation_and_returns_sane_statistics() {
        let mut rng = ChaCha8Rng::from_seed(Default::default());
        let mut simulation = Simulation::random(&mut rng);

        let stats = simulation.train(&mut rng);

        assert_eq!(simulation.generation(), 1);
        assert_eq!(simulation.age, 0);
        assert!(stats.min_fitness() <= stats.avg_fitness());
        assert!(stats.avg_fitness() <= stats.max_fitness());
    }

    #[test]
    fn train_resets_the_world_population_size() {
        let mut rng = ChaCha8Rng::from_seed(Default::default());
        let mut simulation = Simulation::random(&mut rng);

        simulation.train(&mut rng);

        assert_eq!(simulation.world().animals().len(), 40);
        assert_eq!(simulation.world().foods().len(), 60);
    }

    #[test]
    fn config_default_matches_the_historical_hard_coded_values() {
        let config = Config::default();

        assert_eq!(config.num_animals, 40);
        assert_eq!(config.num_foods, 60);
        assert_eq!(config.mutation_chance, 0.01);
        assert_eq!(config.mutation_coeff, 0.3);
        assert_eq!(config.max_speed, SPEED_MAX);
    }

    #[test]
    fn random_with_config_honors_the_requested_population_size() {
        let mut rng = ChaCha8Rng::from_seed(Default::default());
        let config = Config {
            num_animals: 5,
            num_foods: 7,
            ..Config::default()
        };

        let simulation = Simulation::random_with_config(&mut rng, config);

        assert_eq!(simulation.world().animals().len(), 5);
        assert_eq!(simulation.world().foods().len(), 7);
    }

    #[test]
    fn random_with_config_honors_the_requested_max_speed() {
        let mut rng = ChaCha8Rng::from_seed(Default::default());
        let config = Config {
            // Force every bird to accelerate to (and stay clamped at) a
            // tiny max speed within a single step.
            max_speed: 0.0011,
            ..Config::default()
        };

        let mut simulation = Simulation::random_with_config(&mut rng, config);

        simulation.step(&mut rng);

        for animal in simulation.world().animals() {
            assert!(animal.speed <= config.max_speed);
        }
    }
}
