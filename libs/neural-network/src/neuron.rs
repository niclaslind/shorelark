use rand::Rng;

#[derive(Clone, Debug)]
pub struct Neuron {
    pub bias: f32,
    pub weights: Vec<f32>,
}

impl Neuron {
    pub fn new(bias: f32, weights: Vec<f32>) -> Self {
        assert!(!weights.is_empty());

        Self { bias, weights }
    }

    pub fn random(rng: &mut dyn rand::RngCore, output_size: usize) -> Self {
        let bias = rng.gen_range(-1.0..=1.0);

        let weights = (0..output_size)
            .map(|_| rng.gen_range(-1.0..=1.0))
            .collect();

        Self::new(bias, weights)
    }

    pub fn propagate(&self, inputs: &[f32]) -> f32 {
        let output = inputs
            .iter()
            .zip(&self.weights)
            .map(|(input, weight)| input * weight)
            .sum::<f32>();

        (self.bias + output).max(0.0)
    }

    pub fn from_weights(output_neurons: usize, weights: &mut dyn Iterator<Item = f32>) -> Self {
        let bias = weights.next().expect("got not enough weights");

        let weights = (0..output_neurons)
            .map(|_| weights.next().expect("got not enough weights"))
            .collect();

        Self::new(bias, weights)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod new_neuron {
        use std::f32::consts::PI;

        use super::*;

        #[test]
        #[should_panic]
        fn with_empty_weights() {
            Neuron::new(3.3, vec![]);
        }

        #[test]
        fn with_no_empty_weights() {
            Neuron::new(3.3, vec![3.12, PI, 3.17]);
        }
    }
}
