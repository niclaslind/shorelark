#[derive(Clone, Copy, Debug)]
pub struct LayerTopology {
    pub neurons: usize,
}

#[cfg(test)]
mod tests {
    use crate::LayerTopology;

    #[test]
    fn test() {
        let layer_topology = LayerTopology { neurons: 3 };

        assert_eq!(3, layer_topology.neurons);
    }
}
