# Description

(starting over, as i had terrible management of my project)

The idea is to simulate circuits using logical gates like "and", "or", ...
Once done, i will do a CLI circuit editor.

The library will be used by first creating a "CircuitMaker", wich would then be
used to make a reusable template, or a usable circuit.

The CircuitMaker will allow you to add and remove gates and templates, and
connect them together (including in circles).

It should also check the validity of your circuits before making them.

When executed, the Circuit will be executed using a BFS (like) algorithm
(I just don't know if it really is BFS).

This will mean you might have to worry about timings for your circuits, but it
will make things like a clock possible (a "not" gate going into itself).

## Ideas

Templates are going to be reusable sections of gates, and will have an optional
stats section with the number of gates, and the delay of the template.

This stats section will be purely for the user to remember, it will not impact
the circuits in any way.

## Tasks

- [x] Setup Tasks

- [ ] CircuitMaker
  - [ ] Fields
  - [ ] Impl
- [ ] Circuit
  - [ ] Fields
  - [ ] Impl
- [ ] Gate
  - [ ] Fields
  - [ ] Impl
    - [x] define
    - [ ] inputs
    - [ ] outputs
    - [ ] set_inputs
    - [ ] update
    - [ ] changes
