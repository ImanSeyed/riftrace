use crate::commonctrl::CommonController;
use crate::instancectrl::InstanceController;
use crate::{RifError, RifResult};
use nix::{errno::Errno, sys::stat::Mode, unistd};
use std::path::PathBuf;

static TRACEFS_PATH: &str = "/sys/kernel/tracing";

/// TracingControl providing methods to manage core functionalities of ftrace.
pub struct TracingControl {
    tracefs_path: Option<PathBuf>,
}

impl TracingControl {
    /// Create a new `TracingControl`.
    pub fn new() -> Self {
        TracingControl {
            tracefs_path: TracingControl::find_tracefs_dirs()
                .and_then(|vec| vec.into_iter().nth(0)),
        }
    }

    pub fn obtain_instance(instance: &str) -> RifResult<InstanceController> {
        let instance_ctrl = InstanceController::new(instance);
        let instance_path = instance_ctrl.get_path();

        match &instance_path {
            Some(instance_path) => {
                match unistd::mkdir(instance_path, Mode::from_bits_truncate(0o750)) {
                    Ok(()) => Ok(instance_ctrl),
                    Err(errno) if errno == Errno::from_i32(libc::EEXIST) => Ok(instance_ctrl),
                    Err(errno) => Err(RifError::InstanceMkdirFailed(Box::new(errno))),
                }
            }
            None => Err(RifError::InstanceMkdirFailed(Box::new(Errno::from_i32(
                libc::ENOENT,
            )))),
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
}

impl Default for TracingControl {
    fn default() -> Self {
        Self::new()
    }
}

impl CommonController for TracingControl {
    fn get_path(&self) -> Option<PathBuf> {
        self.tracefs_path.clone()
    }

    fn set_path(&mut self, path: Option<PathBuf>) {
        self.tracefs_path = path;
    }
}
