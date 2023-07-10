use crate::error::Error;
use crate::{RifError, RifResult, Tracer, TracingStat};
use std::fs;
use std::io::prelude::*;

macro_rules! get_path {
    ($sub_path:expr) => {
        if cfg!(feature = "old-linux") {
            concat!("/sys/kernel/debug/tracing/", $sub_path)
        } else {
            concat!("/sys/kernel/tracing/", $sub_path)
        }
    };
}

macro_rules! open_ftrace_file {
    ($sub_path:expr, $append:expr) => {
        fs::OpenOptions::new()
            .write(true)
            .append($append)
            .open(get_path!($sub_path))
    };
}

/// Returns the output of the trace in a human
/// readable format
pub fn trace() -> RifResult<String> {
    match is_tracing_on()? {
        true => Ok(fs::read_to_string(get_path!("trace"))?),
        false => Err(Error::TracingIsOff),
    }
}

/// Get the current tracer that is configured.
pub fn get_current_tracer() -> RifResult<Tracer> {
    let tracer: Tracer = fs::read_to_string(get_path!("current_tracer"))?.parse()?;
    Ok(tracer)
}

/// Enable or disable writing to the trace
/// ring buffer.
pub fn set_tracing_on(stat: TracingStat) -> RifResult<()> {
    match stat {
        TracingStat::On => fs::write(get_path!("tracing_on"), "1")?,
        TracingStat::Off => fs::write(get_path!("tracing_on"), "0")?,
    }
    Ok(())
}

/// Returns a boolean whether writing to the trace
/// ring buffer is enabled.
pub fn is_tracing_on() -> RifResult<bool> {
    let is_on = {
        let is_on = fs::read_to_string(get_path!("tracing_on"))?;
        matches!(is_on.trim(), "1")
    };

    Ok(is_on)
}

/// Returns the different types of tracers that
/// have been compiled into the kernel.
pub fn get_available_tracers() -> RifResult<Vec<String>> {
    let available_tracers: Vec<String> = {
        fs::read_to_string(get_path!("available_tracers"))?
            .split_whitespace()
            .map(str::to_string)
            .collect()
    };

    Ok(available_tracers)
}

/// Set the current tracer.
pub fn set_current_tracer(tracer: Tracer) -> RifResult<()> {
    if get_available_tracers()
        .unwrap()
        .contains(&tracer.to_string())
    {
        fs::write(get_path!("current_tracer"), tracer.to_string())?;
        return Ok(());
    }
    Err(RifError::InvalidTracer)
}

/// Limit the trace to only `filter`ed functions.
pub fn set_ftrace_filter(filter: &str, append: bool) -> RifResult<()> {
    let mut file = open_ftrace_file!("set_ftrace_filter", append)?;
    writeln!(file, "{}", filter)?;
    Ok(())
}

/// Function passed to this function will cause the function graph
/// tracer to only trace these functions and the functions that
/// they call.
pub fn set_graph_function(filter: &str, append: bool) -> RifResult<()> {
    let mut file = open_ftrace_file!("set_graph_function", append)?;
    writeln!(file, "{}", filter)?;
    Ok(())
}

/// `mark` will be written into the ftrace buffer.
pub fn trace_marker(mark: &str) -> RifResult<()> {
    fs::write(get_path!("trace_marker"), mark)?;
    Ok(())
}

/// Any function that is added here will not
/// be traced.
pub fn set_ftrace_notrace(filter: &str, append: bool) -> RifResult<()> {
    let mut file: fs::File = open_ftrace_file!("set_ftrace_notrace", append)?;
    writeln!(file, "{}", filter)?;
    Ok(())
}

/// Have the function tracer only trace the threads whose PID are
/// in the `pids`.
pub fn set_ftrace_pid(pids: &[u32], append: bool) -> RifResult<()> {
    let pid_max = fs::read_to_string("/proc/sys/kernel/pid_max")?.parse()?;

    let pids_string = pids
        .iter()
        .map(|pid| {
            if *pid > pid_max {
                return Err(RifError::InvalidPID);
            }
            Ok(pid.to_string())
        })
        .collect::<RifResult<Vec<_>>>()?
        .join(" ");

    let mut file: fs::File = open_ftrace_file!("set_ftrace_pid", append)?;
    writeln!(file, "{}", pids_string)?;

    Ok(())
}

/// Set tracer back to `nop` and disable tracing.
pub fn cleanup_tracing() -> RifResult<()> {
    set_current_tracer(Tracer::Nop)?;
    set_tracing_on(TracingStat::Off)?;
    Ok(())
}
