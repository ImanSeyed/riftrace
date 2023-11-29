use crate::ctrltrait::ControllerTrait;
use crate::mainctrl::MainController;
use crate::RifResult;
use std::fs;

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
        fs::write(self.trace_ctrl.get_joined_path("trace_marker"), mark)?;
        Ok(())
    }
}
