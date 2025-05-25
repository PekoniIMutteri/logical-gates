
/// A logical gate, with some inputs, some ouptuts, and a way of computing those outputs.
pub struct Gate<const IN: usize, const OUT: usize> {
    inputs: [bool; IN],
    outputs: [bool; OUT],
    /// A function to compute the output based on some inputs.
    rule: fn (inputs: &[bool; IN]) -> [bool; OUT],
    /// Each vector in the array represent the corresponding output's connections.
    /// The first usize being the index of the connected component.
    /// The second usize being the index of the input to change on that component.
    connections: [Vec<(usize, usize)>; OUT],
}

impl<const IN: usize, const OUT: usize> Gate<IN, OUT> {
    /// All the inputs and outputs are set to false.
    /// The Gate also has no connections when created.
    pub fn new(rule: fn (inputs: &[bool; IN]) -> [bool; OUT]) -> Gate<IN, OUT> {
        Gate {
            inputs: [false; IN],
            outputs: [false; OUT],
            rule,
            connections: [const {Vec::new()}; OUT],
        }
    }
}

/// A trait to let some implementations be valid on all Gate.
pub trait GateTrait {
    /// Switches one of the inputs of the gate, without updating the output.
    fn stage_change(&mut self, index: usize);
    /// Updates the gate's outputs, based on the current inputs.
    fn update(&mut self) -> Vec<(usize, usize)>;
}

impl<const IN: usize, const OUT: usize> GateTrait for Gate<IN, OUT> {
    fn stage_change(&mut self, index: usize) {
        self.inputs[index] = !self.inputs[index];
    }
    fn update(&mut self) -> Vec<(usize, usize)> {
        let new_outputs = (self.rule)(&self.inputs);
        let mut changes = Vec::new();
        for (index, value) in new_outputs.into_iter().enumerate() {
            if self.outputs[index] == value {
                continue;
            }
            self.outputs[index] = value;
            changes.push(index);
        }
        changes
            .into_iter()
            .flat_map(|index| self.connections[index].clone())
            .collect()
    }
}

