use std::error::Error;

use crate::toasts::error_string;

pub trait ErrorExt {
    fn toast_on_err(self) -> Self;
}

impl<T, E> ErrorExt for Result<T, E>
where
    E: Error + Send + Sync + 'static + Into<anyhow::Error>,
{
    fn toast_on_err(self) -> Self {
        if let Err(e) = &self {
            error_string(e.to_string());
        }
        self
    }
}
