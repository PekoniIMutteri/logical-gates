
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

// A gate to compute some output based on some boolean inputs
// (a glorified fn(Vec<bool>) -> bool)
pub struct Gate {
    // TODO: Add Gate fields
}

impl Gate {
    // Creates a new Gate using the given function and a size for inputs.
    // All inputs are initialized to `false`.
    // fn new: (fn(Vec<bool>) -> bool) x Number -> Gate
    // TODO: new

    // changes the gate's inputs
    // fn set_input: Gate x Number x bool -> Gate
    // TODO: set_input

    // Updates the gate's output based on the current inputs
    // And saves wether the output changed or not
    // fn update: Gate -> (Gate, (Number, Number, bool))
    // TODO: update

    // Connects a gate's output to another's input
    // the first number being the destination component, the second, its input index.
    // fn connect: Gate x Number x Number -> Gate
    // TODO: connect
}

