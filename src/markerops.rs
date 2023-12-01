use crate::ctrltrait::ControllerTrait;
use crate::mainctrl::MainController;
use crate::RifResult;
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
    pub fn trace_marker(&self, mark: &str) -> RifResult<()> {
        self.trace_ctrl
            .write_to_subpath(PathBuf::from("trace_marker"), false, mark)?;
        Ok(())
    }
}
