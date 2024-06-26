use crate::tracer::{Tracer, TracingStat};
use crate::{RifError, RifResult};
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

pub trait ControllerTrait {
    fn get_path(&self) -> Option<PathBuf>;
    fn set_path(&mut self, path: Option<PathBuf>);

    /// Generates the full path by combining `subpath`
    /// with `tracefs_path`.
    fn get_joined_path<P: AsRef<Path>>(&self, subpath: P) -> PathBuf {
        match &self.get_path() {
            Some(tracefs_path) => tracefs_path.join(subpath),
            None => panic!("There is no tracefs available to work on."),
        }
    }

    /// Write the `content` to a file in the tracefs
    /// located at `subpath`.
    fn write_to_subpath(
        &self,
        subpath: PathBuf,
        with_append: bool,
        content: &str,
    ) -> RifResult<()> {
        let mut file = fs::OpenOptions::new()
            .write(true)
            .append(with_append)
            .open(self.get_joined_path(subpath))?;

        write!(file, "{}", content)?;

        Ok(())
    }

    /// Enable or disable writing to the trace
    /// ring buffer.
    fn set_tracing_on(&self, stat: TracingStat) -> RifResult<()> {
        match stat {
            TracingStat::On => self.write_to_subpath(PathBuf::from("tracing_on"), false, "1")?,
            TracingStat::Off => self.write_to_subpath(PathBuf::from("tracing_on"), false, "0")?,
        }
        Ok(())
    }

    /// Find tracefs directories from /proc/mounts.
    fn find_tracefs_dirs() -> Option<Vec<PathBuf>> {
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

    /// Returns a boolean whether writing to the trace
    /// ring buffer is enabled.
    fn is_tracing_on(&self) -> RifResult<bool> {
        Ok(matches!(
            fs::read_to_string(self.get_joined_path("tracing_on"))?.trim(),
            "1"
        ))
    }

    /// Returns the output of the trace in a human
    /// readable format.
    fn trace(&self) -> RifResult<String> {
        match self.is_tracing_on()? {
            true => Ok(fs::read_to_string(self.get_joined_path("trace"))?),
            false => Err(RifError::TracingDisabled),
        }
    }

    /// Returns the different types of tracers that
    /// have been compiled into the kernel.
    fn get_available_tracers(&self) -> RifResult<Vec<String>> {
        let available_tracers: Vec<String> = {
            fs::read_to_string(self.get_joined_path("available_tracers"))?
                .split_whitespace()
                .map(str::to_string)
                .collect()
        };

        Ok(available_tracers)
    }

    /// Get the current tracer that is configured.
    fn get_current_tracer(&self) -> RifResult<Tracer> {
        let tracer: Tracer = fs::read_to_string(self.get_joined_path("current_tracer"))?.parse()?;
        Ok(tracer)
    }

    /// Set the current tracer.
    fn set_current_tracer(&self, tracer: Tracer) -> RifResult<()> {
        if self.get_available_tracers()?.contains(&tracer.to_string()) {
            self.write_to_subpath(PathBuf::from("current_tracer"), false, &tracer.to_string())?;
            return Ok(());
        }
        Err(RifError::InvalidTracer)
    }

    /// Set tracer back to `nop` and disable tracing.
    fn cleanup_tracing(&self) -> RifResult<()> {
        self.set_current_tracer(Tracer::Nop)?;
        self.set_tracing_on(TracingStat::Off)?;
        Ok(())
    }
}
