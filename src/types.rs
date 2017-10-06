#[derive(Debug)]
pub enum BrainCommand {
    Decrement,
    Increment,
    Input,
    JumpBackward(Option<usize>),
    JumpForward(Option<usize>),
    MoveLeft,
    MoveRight,
    Noop,
    Output,
}
