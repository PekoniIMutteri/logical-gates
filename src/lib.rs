
pub struct CircuitMaker {
    // TODO: Add CircuitMaker fields
}

impl CircuitMaker {
    // TODO: Add CircuitMaker implementation
}

pub struct Circuit {
    // TODO: Add Circuit fields
}

impl Circuit {
    // TODO: Add circuit implementation
}

/// A gate to compute some output based on some boolean inputs
/// (a glorified fn(Vec<bool>) -> bool)
pub struct Gate {
    inputs: Vec<bool>,
    output: bool,
    function: fn(&Vec<bool>) -> bool,
    connections: Vec<(usize, usize)>,
}

impl Gate {
    /// Creates a new Gate using the given function and a size for inputs.
    /// All inputs and the output are initialized to `false`.
    pub fn new(function: fn(&Vec<bool>) -> bool, in_size: usize) -> Gate {
        Gate {
            inputs: vec![false; in_size],
            output: false,
            function,
            connections: Vec::new(),
        }
    }

    /// changes the gate's inputs
    pub fn set_input(&mut self, index: usize, new_value: bool) {
        self.inputs[index] = new_value;
    }

    /// Updates the gate's output based on the current inputs
    /// Returns wich component to update, with wich input was updated, and to what.
    pub fn update(&mut self) -> Vec<(usize, usize, bool)> {
        if self.output != (self.function)(&self.inputs) {
            self.output = !self.output;
            self.connections
                .iter()
                .map(|(comp, ind)| (*comp, *ind, self.output))
                .collect()
        } else {
            Vec::new()
        }
    }

    /// Connects a gate's output to another's input
    /// the first number being the destination component, the second, its input index.
    pub fn connect(&mut self, comp: usize, ind: usize) {
        self.connections.push((comp, ind));
    }
}

