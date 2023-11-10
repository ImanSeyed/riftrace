# Description
This library provides a simple API for handling Ftrace files.

# Examples
```rust
use riftrace::{Tracer, TracingStat, Controller};
let controller = Controller::new();
// Change current tracer from nop to function_graph
controller.set_current_tracer(Tracer::FunctionGraph).unwrap();
// Turn tracing on
controller.set_tracing_on(TracingStat::On).unwrap();
// Limit the trace to only "net*"
controller.set_ftrace_filter("net*", false).unwrap();
// Print out the output of the trace in a human readable format
println!("{}", controller.trace().unwrap());
```