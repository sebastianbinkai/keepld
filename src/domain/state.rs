#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum State {
    Active,
    Waiting,
    Paused,
    Done,
}
