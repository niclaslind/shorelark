pub use guassian::*;

use crate::*;

mod guassian;

pub trait MutationMethod {
    fn mutate(&self, rng: &mut dyn RngCore, child: &mut Chromosome);
}
