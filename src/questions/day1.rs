use crate::solutions::{Solution, SolutionResult};

#[derive(Default)]
pub struct Question;

impl Question {
    fn solve(&self, input: Option<&str>) -> SolutionResult {
        Ok(("".to_string()))
    }
}

impl Solution for Question {
    fn solve_input_1(&self, input: Option<&str>) -> SolutionResult {
        Ok(self.solve(input)?)
    }

    fn solve_input_2(&self, input: Option<&str>) -> SolutionResult {
        Ok(self.solve(input)?)
    }
}