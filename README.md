# Description
This library provides a simple API for handling Ftrace files.

# Examples

```rust
use riftrace::{self, Tracer, TracingStat};
// Change current tracer from nop to function_graph 
riftrace::set_current_tracer(Tracer::FunctionGraph).unwrap();
// Turn tracing on
riftrace::set_tracing_on(TracingStat::On).unwrap();
// Limit the trace to only "net*"
riftrace::set_ftrace_filter("net*", false).unwrap();
// Print out the output of the trace in a human readable format
println!("{}", riftrace::trace().unwrap());
```