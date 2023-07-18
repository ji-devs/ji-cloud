use std::error::Error;

pub trait ErrorExt<T> {
    fn into_anyhow(self) -> anyhow::Result<T>;
}

impl<T, E> ErrorExt<T> for Result<T, E>
where
    E: Error + Send + Sync + 'static + Into<anyhow::Error>,
{
    fn into_anyhow(self) -> anyhow::Result<T> {
        self.map_err(|e| e.into())
    }
}
