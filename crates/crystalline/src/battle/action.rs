#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Action {
    UseMove { move_index: usize },
    Switch { target_index: usize },
    Pass,
}
