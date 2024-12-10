pub trait Command {
    fn execute(&self, input: &str) -> String;
    fn name(&self) -> &str;
}

pub trait Path {
    fn get_cwd(&self) -> String;
    fn set_cwd(&mut self, new_cwd: String);
}

pub struct GlobalPath {
    pub cwd: String,
}
