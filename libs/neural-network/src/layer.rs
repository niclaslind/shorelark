use crate::*;

#[derive(Clone, Debug)]
pub struct Layer {
    pub neurons: Vec<Neuron>,
}

impl Layer {
    pub fn new(neurons: Vec<Neuron>) -> Self {
        assert!(!neurons.is_empty());

        assert!(neurons
            .iter()
            .all(|neuron| neuron.weights.len() == neurons[0].weights.len()));

        Self { neurons }
    }

    pub fn random(rng: &mut dyn RngCore, input_neurons: usize, output_neurons: usize) -> Self {
        let neurons = (0..output_neurons)
            .map(|_| Neuron::random(rng, input_neurons))
            .collect();

        Self::new(neurons)
    }

    pub fn propagate(&self, inputs: Vec<f32>) -> Vec<f32> {
        self.neurons
            .iter()
            .map(|neuron| neuron.propagate(&inputs))
            .collect()
    }

    pub fn from_weights(
        input_size: usize,
        output_size: usize,
        weights: &mut dyn Iterator<Item = f32>,
    ) -> Self {
        let neurons = (0..output_size)
            .map(|_| Neuron::from_weights(input_size, weights))
            .collect();

        Self::new(neurons)
    }
}

#[cfg(test)]
mod tests {
    use core::f32;

    use super::*;

    mod new_layer {
        use super::*;

        #[test]
        #[should_panic]
        fn with_empty_neurons() {
            Layer::new(vec![]);
        }

        #[test]
        fn with_no_empty_neurons() {
            let layer = Layer::new(vec![Neuron::new(3.13, vec![3.15])]);
            assert_eq!(1, layer.neurons.len());
        }
    }

    #[test]
    fn test_propogate() {
        let layer = Layer::new(vec![
            Neuron::new(f32::consts::PI, vec![3.15, 6.15, 1.4, 8.4, 1.1]),
            Neuron::new(1.2, vec![3.23, 1.23, 5.23, 9.0, 1.2]),
        ]);
        assert_eq!(
            vec![794.58997, 821.2],
            layer.propagate(vec![32.1, 13.5, 12.4, 53.3, 129.3])
        );
    }
}
