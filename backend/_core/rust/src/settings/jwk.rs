#[derive(Debug)]
pub struct JwkSettings {
    pub audience: String,
    pub issuer: String,
}

impl JwkSettings {
    pub(crate) fn new(project_id: String) -> anyhow::Result<Self> {
        let issuer = format!("{}/{}", config::JWK_ISSUER_URL, project_id);

        Ok(JwkSettings {
            audience: project_id,
            issuer,
        })
    }
}
