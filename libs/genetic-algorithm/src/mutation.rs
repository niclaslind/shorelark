pub use guassian::*;

use crate::*;

mod guassian;

pub trait MutationMethod {
    fn mutate(&self, rng: &mut dyn Rng, child: &mut Chromosome);
}
