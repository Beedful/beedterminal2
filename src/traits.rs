pub trait Command {
    fn execute(&self, input: &str) -> String;
    fn name(&self) -> &str;
}