pub enum SolutionError {
    FileNotFound
}

pub type SolutionResult = Result<String, SolutionError>;

pub trait Solution {
    fn solve_input_1(&self, input: Option<&str>) -> SolutionResult;
    fn solve_input_2(&self, input: Option<&str>) -> SolutionResult;
}
