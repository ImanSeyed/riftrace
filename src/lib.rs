//! This crate provides a simple API for handling Ftrace files.
//!
//! # Examples
//!
//! ```
//! use riftrace::{Tracer, TracingStat, TracingControl, FilterOps, CommonController};
//! let trace_ctrl = TracingControl::new();
//! let filter_operator = FilterOps::new(&trace_ctrl);
//! // Change current tracer from nop to function_graph
//! trace_ctrl.set_current_tracer(Tracer::FunctionGraph).unwrap();
//! // Turn tracing on
//! trace_ctrl.set_tracing_on(TracingStat::On).unwrap();
//! // Limit the trace to only "net*"
//! filter_operator.set_ftrace_filter("net*", false).unwrap();
//! // Print out the output of the trace in a human readable format
//! println!("{}", trace_ctrl.trace().unwrap());
//! ```

mod commonctrl;
mod error;
mod filterops;
mod instancectrl;
mod markerops;
mod tracecontrol;
mod tracer;

pub use commonctrl::*;
pub use filterops::*;
pub use instancectrl::*;
pub use markerops::*;
pub use tracecontrol::*;
pub use tracer::{Tracer, TracingStat};
pub type RifError = error::Error;
pub type RifResult<T> = Result<T, RifError>;
