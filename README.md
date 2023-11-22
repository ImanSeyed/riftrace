# Description
This library provides a simple API for handling Ftrace files.

# Examples
```rust
use riftrace::{Tracer, TracingStat, TracingControl, FilterOps, CommonController};
let trace_ctrl = TracingControl::new();
let filter_operator = FilterOps::new(&trace_ctrl);
// Change current tracer from nop to function_graph
trace_ctrl.set_current_tracer(Tracer::FunctionGraph).unwrap();
// Turn tracing on
trace_ctrl.set_tracing_on(TracingStat::On).unwrap();
// Limit the trace to only "net*"
filter_operator.set_ftrace_filter("net*", false).unwrap();
// Print out the output of the trace in a human readable format
println!("{}", trace_ctrl.trace().unwrap());
```