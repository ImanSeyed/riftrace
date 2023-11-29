use crate::ctrltrait::ControllerTrait;
use crate::mainctrl::MainController;
use crate::{RifError, RifResult};
use std::fs;
use std::io::Write;
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
        let mut file = self
            .trace_ctrl
            .open_to_write(PathBuf::from("set_ftrace_filter"), with_append)?;
        writeln!(file, "{}", filter)?;
        Ok(())
    }

    /// Any function that is added here will not
    /// be traced.
    pub fn set_ftrace_notrace(&self, filter: &str, with_append: bool) -> RifResult<()> {
        let mut file = self
            .trace_ctrl
            .open_to_write(PathBuf::from("set_ftrace_notrace"), with_append)?;
        writeln!(file, "{}", filter)?;
        Ok(())
    }

    /// Function passed to this function will cause the function graph
    /// tracer to only trace these functions and the functions that
    /// they call.
    pub fn set_graph_function(&self, filter: &str, with_append: bool) -> RifResult<()> {
        let mut file = self
            .trace_ctrl
            .open_to_write(PathBuf::from("set_graph_function"), with_append)?;
        writeln!(file, "{}", filter)?;
        Ok(())
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
        let mut file = self
            .trace_ctrl
            .open_to_write(PathBuf::from("set_ftrace_pid"), with_append)?;
        writeln!(file, "{}", pids_string)?;

        Ok(())
    }

    /// The tracer function should exclude tracing threads with PIDs listed in pids.
    pub fn set_ftrace_notrace_pid(&self, pids: &[u32], with_append: bool) -> RifResult<()> {
        let pids_string = self.pids_as_string(pids)?;
        let mut file = self
            .trace_ctrl
            .open_to_write(PathBuf::from("set_ftrace_notrace_pid"), with_append)?;
        writeln!(file, "{}", pids_string)?;

        Ok(())
    }
}
