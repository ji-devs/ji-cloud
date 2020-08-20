use crate::env::{env_bool, req_env};

pub struct S3Settings {
    pub endpoint: String,
    pub bucket: String,
    pub use_client: bool,
}

impl S3Settings {
    pub(crate) fn new(is_local: bool) -> anyhow::Result<Self> {
        let endpoint = req_env("S3_ENDPOINT")?;
        let bucket = req_env("S3_BUCKET")?;
        let disable_local = env_bool("S3_LOCAL_DISABLE_CLIENT");

        Ok(Self {
            endpoint,
            bucket,
            use_client: !is_local || !disable_local,
        })
    }
}
