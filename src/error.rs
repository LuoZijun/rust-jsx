

#[derive(Debug, Clone, Copy)]
pub enum Error {
    EndOfProgram,
    UnexpectedEndOfProgram,
    UnexpectedToken,
}
