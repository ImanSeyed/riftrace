use crate::{RifError, RifResult, Tracer, TracingStat};
use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};

static TRACEFS_PATH: &str = "/sys/kernel/tracing";

/// Controller providing methods to manage core functionalities of ftrace.
pub struct Controller {
    tracefs_path: Option<PathBuf>,
}

impl Controller {
    /// Create a new `Controller`.
    pub fn new() -> Self {
        Controller {
            tracefs_path: Controller::find_tracefs_dirs().and_then(|vec| vec.into_iter().nth(0)),
        }
    }

    /// Find tracefs directories from /proc/mounts.
    pub fn find_tracefs_dirs() -> Option<Vec<PathBuf>> {
        let mut tracefs_dirs = Vec::<PathBuf>::new();
        let mounts_content = fs::read_to_string("/proc/mounts").unwrap();
        for line in mounts_content.lines() {
            let words: Vec<&str> = line.split_whitespace().collect();
            if words.get(2) == Some(&"tracefs") {
                if let Some(path) = words.get(1) {
                    tracefs_dirs.push(PathBuf::from(path));
                }
            }
        }

        if tracefs_dirs.is_empty() {
            None
        } else {
            Some(tracefs_dirs)
        }
    }

    /// Tries to mount tracefs to TRACEFS_PATH.
    pub fn mount_tracefs() -> RifResult<()> {
        match nix::mount::mount::<str, str, str, str>(
            Some("nodev"),
            TRACEFS_PATH,
            Some("tracefs"),
            nix::mount::MsFlags::MS_NOSUID
                | nix::mount::MsFlags::MS_NOEXEC
                | nix::mount::MsFlags::MS_NODEV,
            None,
        ) {
            Ok(()) => Ok(()),
            Err(errno) => Err(RifError::MountTracefsFailed(Box::new(errno))),
        }
    }

    /// Generates the full path by combining `subpath`
    /// with `tracefs_path`.
    fn get_fullpath<P: AsRef<Path>>(&self, subpath: P) -> PathBuf {
        match &self.tracefs_path {
            Some(tracefs_path) => tracefs_path.join(subpath),
            None => panic!("There is no tracefs available to work on."),
        }
    }

    //// Open a file located at `subpath` within the tracefs.
    fn open_to_write(&self, subpath: PathBuf, with_append: bool) -> RifResult<File> {
        fs::OpenOptions::new()
            .write(true)
            .append(with_append)
            .open(self.get_fullpath(subpath))
            .map_err(|_| RifError::FailedOpenToWrite)
    }

    /// Returns the output of the trace in a human
    /// readable format.
    pub fn trace(&self) -> RifResult<String> {
        match self.is_tracing_on()? {
            true => Ok(fs::read_to_string(self.get_fullpath("trace"))?),
            false => Err(RifError::TracingIsOff),
        }
    }

    /// Get the current tracer that is configured.
    pub fn get_current_tracer(&self) -> RifResult<Tracer> {
        let tracer: Tracer = fs::read_to_string(self.get_fullpath("current_tracer"))?.parse()?;
        Ok(tracer)
    }

    /// Enable or disable writing to the trace
    /// ring buffer.
    pub fn set_tracing_on(&self, stat: TracingStat) -> RifResult<()> {
        match stat {
            TracingStat::On => fs::write(self.get_fullpath("tracing_on"), "1")?,
            TracingStat::Off => fs::write(self.get_fullpath("tracing_on"), "0")?,
        }
        Ok(())
    }

    /// Returns a boolean whether writing to the trace
    /// ring buffer is enabled.
    pub fn is_tracing_on(&self) -> RifResult<bool> {
        Ok(matches!(
            fs::read_to_string(self.get_fullpath("tracing_on"))?.trim(),
            "1"
        ))
    }

    /// Returns the different types of tracers that
    /// have been compiled into the kernel.
    pub fn get_available_tracers(&self) -> RifResult<Vec<String>> {
        let available_tracers: Vec<String> = {
            fs::read_to_string(self.get_fullpath("available_tracers"))?
                .split_whitespace()
                .map(str::to_string)
                .collect()
        };

        Ok(available_tracers)
    }

    /// Set the current tracer.
    pub fn set_current_tracer(&self, tracer: Tracer) -> RifResult<()> {
        if self.get_available_tracers()?.contains(&tracer.to_string()) {
            fs::write(self.get_fullpath("current_tracer"), tracer.to_string())?;
            return Ok(());
        }
        Err(RifError::InvalidTracer)
    }

    /// Limit the trace to only `filter`ed functions.
    pub fn set_ftrace_filter(&self, filter: &str, with_append: bool) -> RifResult<()> {
        let mut file = self.open_to_write(PathBuf::from("set_ftrace_filter"), with_append)?;
        writeln!(file, "{}", filter)?;
        Ok(())
    }

    /// Function passed to this function will cause the function graph
    /// tracer to only trace these functions and the functions that
    /// they call.
    pub fn set_graph_function(&self, filter: &str, with_append: bool) -> RifResult<()> {
        let mut file = self.open_to_write(PathBuf::from("set_graph_function"), with_append)?;
        writeln!(file, "{}", filter)?;
        Ok(())
    }

    /// `mark` will be written into the ftrace buffer.
    pub fn trace_marker(&self, mark: &str) -> RifResult<()> {
        fs::write(self.get_fullpath("trace_marker"), mark)?;
        Ok(())
    }

    /// Any function that is added here will not
    /// be traced.
    pub fn set_ftrace_notrace(&self, filter: &str, with_append: bool) -> RifResult<()> {
        let mut file = self.open_to_write(PathBuf::from("set_ftrace_notrace"), with_append)?;
        writeln!(file, "{}", filter)?;
        Ok(())
    }

    /// Have the function tracer only trace the threads whose PID are
    /// in the `pids`.
    pub fn set_ftrace_pid(&self, pids: &[u32], with_append: bool) -> RifResult<()> {
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

        let mut file = self.open_to_write(PathBuf::from("set_ftrace_pid"), with_append)?;
        writeln!(file, "{}", pids_string)?;

        Ok(())
    }

    /// Set tracer back to `nop` and disable tracing.
    pub fn cleanup_tracing(&self) -> RifResult<()> {
        self.set_current_tracer(Tracer::Nop)?;
        self.set_tracing_on(TracingStat::Off)?;
        Ok(())
    }
}

impl Default for Controller {
    fn default() -> Self {
        Self::new()
    }
}
