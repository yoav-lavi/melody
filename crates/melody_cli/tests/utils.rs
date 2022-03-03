use std::error::Error;

#[derive(Debug)]
pub struct BoxedError {
    #[allow(unused)]
    error: Box<dyn Error>,
}

impl<T: 'static> From<T> for BoxedError
where
    T: Error,
{
    fn from(error: T) -> Self {
        BoxedError {
            error: Box::new(error),
        }
    }
}

pub type TestResult = Result<(), BoxedError>;
