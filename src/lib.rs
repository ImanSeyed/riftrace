//! This crate provides a simple API for handling Ftrace files.
//!
//! # Examples
//!
//! ```
//! use riftrace::controller::{CommonController, MainController};
//! use riftrace::operations::FilterOps;
//! use riftrace::{Tracer, TracingStat};
//! let trace_ctrl = MainController::new();
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
mod mainctrl;
mod markerops;
mod tracer;

pub mod controller {
    pub use super::commonctrl::*;
    pub use super::instancectrl::*;
    pub use super::mainctrl::*;
}

pub mod operations {
    pub use super::filterops::*;
    pub use super::markerops::*;
}

pub use tracer::{Tracer, TracingStat};
pub type RifError = error::Error;
pub type RifResult<T> = Result<T, RifError>;
