mod control_flow;
mod datatypes;
mod functions;
mod ownership;
mod references_borrowing;
mod variables;

fn main() {
    // Each module in the playground will include a run function and run everything
    // related to said module
    variables::main();
    datatypes::main();
    functions::main();
    control_flow::main();
    ownership::main();
    references_borrowing::main();
}
