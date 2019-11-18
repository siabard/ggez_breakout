use crate::states;
pub struct Reg {
    pub states: Vec<Box<dyn states::States>>,
}

impl Reg {
    pub fn new() -> Reg {
        Reg { states: vec![] }
    }

    pub fn add_state(&mut self, state: Box<dyn states::States>) {
        self.states.push(state);
    }

    pub fn pop_state(&mut self) -> Box<dyn states::States> {
        self.states.pop().unwrap()
    }
}
