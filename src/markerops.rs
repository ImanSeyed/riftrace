use crate::ctrltrait::ControllerTrait;
use crate::mainctrl::MainController;
use crate::RifResult;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

/// Provides functionality to manage tracing markers.
pub struct MarkerOps<'a> {
    trace_ctrl: &'a MainController,
}

impl<'a> MarkerOps<'a> {
    /// Create a new `MarkerOps`.
    pub fn new(trace_ctrl: &'a MainController) -> Self {
        MarkerOps { trace_ctrl }
    }

    /// `mark` will be written into the ftrace buffer.
    pub fn marker_write(&self, mark: &str) -> RifResult<()> {
        self.trace_ctrl
            .write_to_subpath(PathBuf::from("trace_marker"), false, mark)?;
        Ok(())
    }

    pub fn marker_write_raw(&self, mark: &[u8]) -> RifResult<()> {
        let mut raw_marker_file = File::open(
            self.trace_ctrl
                .get_joined_path(PathBuf::from("trace_marker_raw")),
        )?;
        raw_marker_file.write_all(mark)?;
        Ok(())
    }
}
