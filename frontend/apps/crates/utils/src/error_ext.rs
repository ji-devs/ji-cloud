use std::error::Error;

use crate::toasts::error_string;

pub trait ErrorExt<T> {
    fn into_anyhow(self) -> anyhow::Result<T>;

    fn toast_on_err(self) -> Self;
}

impl<T, E> ErrorExt<T> for Result<T, E>
where
    E: Error + Send + Sync + 'static + Into<anyhow::Error>,
{
    fn into_anyhow(self) -> anyhow::Result<T> {
        self.map_err(|e| e.into())
    }

    fn toast_on_err(self) -> Self {
        if let Err(e) = &self {
            error_string(e.to_string());
        }
        self
    }
}
