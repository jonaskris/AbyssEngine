#[derive(Debug)]
pub enum EngineError {
    InitializeError(String),
    RuntimeError(String),
}