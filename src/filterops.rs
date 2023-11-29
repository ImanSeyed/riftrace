use crate::ctrltrait::ControllerTrait;
use crate::mainctrl::MainController;
use crate::{RifError, RifResult};
use std::fs;
use std::path::PathBuf;

/// Facilitates operations involving filtering of tracing events and functions.
pub struct FilterOps<'a> {
    trace_ctrl: &'a MainController,
}

impl<'a> FilterOps<'a> {
    /// Create a new `FilterOps`.
    pub fn new(trace_ctrl: &'a MainController) -> Self {
        FilterOps { trace_ctrl }
    }

    /// Limit the trace to only `filter`ed functions.
    pub fn set_ftrace_filter(&self, filter: &str, with_append: bool) -> RifResult<()> {
        self.trace_ctrl
            .write_to_subpath(PathBuf::from("set_ftrace_filter"), with_append, filter)
    }

    /// Any function that is added here will not
    /// be traced.
    pub fn set_ftrace_notrace(&self, filter: &str, with_append: bool) -> RifResult<()> {
        self.trace_ctrl
            .write_to_subpath(PathBuf::from("set_ftrace_notrace"), with_append, filter)
    }

    /// Function passed to this function will cause the function graph
    /// tracer to only trace these functions and the functions that
    /// they call.
    pub fn set_graph_function(&self, filter: &str, with_append: bool) -> RifResult<()> {
        self.trace_ctrl
            .write_to_subpath(PathBuf::from("set_graph_function"), with_append, filter)
    }

    /// Check and merge PIDs into a string.
    fn pids_as_string(&self, pids: &[u32]) -> RifResult<String> {
        let pid_max = fs::read_to_string("/proc/sys/kernel/pid_max")?.parse()?;

        let pids_string = pids
            .iter()
            .map(|pid| {
                if *pid > pid_max {
                    return Err(RifError::InvalidProcessID);
                }
                Ok(pid.to_string())
            })
            .collect::<RifResult<Vec<_>>>()?
            .join(" ");

        Ok(pids_string)
    }

    /// Modify the tracer function to exclusively trace threads with PIDs present
    /// in the pids list.
    pub fn set_ftrace_pid(&self, pids: &[u32], with_append: bool) -> RifResult<()> {
        let pids_string = self.pids_as_string(pids)?;
        self.trace_ctrl
            .write_to_subpath(PathBuf::from("set_ftrace_pid"), with_append, &pids_string)
    }

    /// The tracer function should exclude tracing threads with PIDs listed in pids.
    pub fn set_ftrace_notrace_pid(&self, pids: &[u32], with_append: bool) -> RifResult<()> {
        let pids_string = self.pids_as_string(pids)?;
        self.trace_ctrl.write_to_subpath(
            PathBuf::from("set_ftrace_notrace_pid"),
            with_append,
            &pids_string,
        )
    }
}
