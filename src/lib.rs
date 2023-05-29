//! This crate provides a simple API for handling Ftrace files.
//!
//! # Examples
//!
//! ```
//! use riftrace::{self, Tracer, TracingStat};
//! // Change current tracer from nop to function_graph
//! riftrace::set_current_tracer(Tracer::FunctionGraph).unwrap();
//! // Turn tracing on
//! riftrace::set_tracing_on(TracingStat::On).unwrap();
//! // Limit the trace to only "net*"
//! riftrace::set_ftrace_filter("net*", false).unwrap();
//! // Print out the output of the trace in a human readable format
//! println!("{}", riftrace::trace().unwrap());
//! ```

mod controllers;
mod error;
mod tracer;

pub use controllers::*;
pub use tracer::{Tracer, TracingStat};
pub type RifError = error::Error;
pub type RifResult<T> = Result<T, RifError>;