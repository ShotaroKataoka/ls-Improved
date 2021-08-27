use thiserror::Error;

#[derive(Error, Debug)]
pub enum LsiError {
    #[error("test error")]
    TestError,
}
