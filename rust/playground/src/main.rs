mod control_flow;
mod datatypes;
mod enums;
mod functions;
mod ownership;
mod references_borrowing;
mod structures;
mod slices;
mod variables;
mod datastructs;
mod errors;

fn main() {
    // Each module in the playground will include a run function and run everything
    // related to said module
    variables::main();
    datatypes::main();
    functions::main();
    control_flow::main();
    ownership::main();
    references_borrowing::main();
    slices::main();
    structures::main();
    enums::main();
    datastructs::main();
    errors::main();
}
