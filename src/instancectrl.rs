use crate::commonctrl::CommonController;
use std::path::PathBuf;

pub struct InstanceController {
    instance_path: Option<PathBuf>,
}

impl InstanceController {
    pub fn new(instance: Option<&str>) -> Self {
        let mut instance_path = PathBuf::from("instances");
        let instance_path_opt = match instance {
            Some(inst) => {
                instance_path.push(inst);
                InstanceController::find_tracefs_dirs()
                    .and_then(|vec| vec.into_iter().nth(0).map(|path| path.join(&instance_path)))
            }
            None => None,
        };

        InstanceController {
            instance_path: instance_path_opt,
        }
    }
}

impl CommonController for InstanceController {
    fn get_path(&self) -> Option<PathBuf> {
        self.instance_path.clone()
    }

    fn set_path(&mut self, path: Option<PathBuf>) {
        self.instance_path = path;
    }
}
