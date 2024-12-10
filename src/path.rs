use crate::traits;
use traits::Path;
use traits::GlobalPath;
use lazy_static::lazy_static;

impl GlobalPath {
    pub fn new() -> Self {
        Self { cwd: "/".to_string() }
    }
}

impl Path for GlobalPath {
    fn get_cwd(&self) -> String {
        self.cwd.clone()
    }

    fn set_cwd(&mut self, new_cwd: String) {
        self.cwd = new_cwd;
    }
}

lazy_static! {
    pub static ref GLOBAL_PATH: GlobalPath = GlobalPath::new();
}