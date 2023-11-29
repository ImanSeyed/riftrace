use crate::ctrltrait::ControllerTrait;
use crate::instancectrl::InstanceController;
use crate::{RifError, RifResult};
use nix::{errno::Errno, sys::stat::Mode, unistd};
use std::path::PathBuf;

static TRACEFS_PATH: &str = "/sys/kernel/tracing";

/// Providing methods to manage core functionalities of ftrace.
pub struct MainController {
    tracefs_path: Option<PathBuf>,
}

impl MainController {
    /// Creates a new `MainController`.
    pub fn new() -> Self {
        MainController {
            tracefs_path: MainController::find_tracefs_dirs()
                .and_then(|vec| vec.into_iter().nth(0)),
        }
    }

    pub fn obtain_instance(&self, instance: &str) -> RifResult<InstanceController> {
        let mut instance_ctrl = InstanceController::new(None);
        let instance_path = self.get_path().and_then(|tracefs_path| {
            instance_ctrl.set_path(Some(tracefs_path.join("instances").join(instance)));
            instance_ctrl.get_path()
        });

        match &instance_path {
            Some(instance_path) => {
                match unistd::mkdir(instance_path, Mode::from_bits_truncate(0o750)) {
                    Ok(()) => Ok(instance_ctrl),
                    Err(errno) if errno == Errno::from_i32(libc::EEXIST) => Ok(instance_ctrl),
                    Err(errno) => Err(RifError::InstanceCreationFailed(Box::new(errno))),
                }
            }
            None => Err(RifError::InstanceCreationFailed(Box::new(Errno::from_i32(
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
            Err(errno) => Err(RifError::TracefsMountFailed(Box::new(errno))),
        }
    }
}

impl Default for MainController {
    fn default() -> Self {
        Self::new()
    }
}

impl ControllerTrait for MainController {
    fn get_path(&self) -> Option<PathBuf> {
        self.tracefs_path.clone()
    }

    fn set_path(&mut self, path: Option<PathBuf>) {
        self.tracefs_path = path;
    }
}
