mod control_flow;
mod datatypes;
mod functions;
mod variables;

fn main() {
    // Each module in the playground will include a run function and run everything
    // related to said module
    variables::main();
    datatypes::main();
    functions::main();
    control_flow::main();
}
