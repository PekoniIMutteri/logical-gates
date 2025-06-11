
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
// (a glorified fn(Vec<bool>) -> Vec<bool>)
pub struct Gate {
    // TODO: Add Gate fields
}

impl Gate {
    // gets all the gate's inputs
    // fn inputs: Gate -> Vec<bool>
    // TODO: inputs

    // gets all the gate's outputs
    // fn outputs: Gate -> Vec<bool>
    // TODO: outputs

    // changes the gate's inputs
    // fn set_input: Gate x Number x bool -> Gate
    // TODO: set_input

    // Updates the gate's outputs based on the current inputs
    // And saves wich outputs changed
    // fn update: Gate -> Gate
    // TODO: update

    // gets a list of all the components to update, with wich input changed, and to what.
    // fn changes: Gate -> Vec<(Number, Number, bool)>
    // TODO: changes
}

