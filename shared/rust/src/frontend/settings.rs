pub const MEDIA_UI_PATH:&'static str = "app/ui";

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum RemoteTarget {
    Local,
    Sandbox,
    Release,
}

impl RemoteTarget {
    pub fn api_url_base(&self) -> &'static str {
        match self {
            Self::Local => "http://localhost:8081",
            Self::Sandbox => "https://sandbox.api.jicloud.org",
            Self::Release => "https://api.jicloud.org",
        }
    }

    pub fn api_js_url_base(&self) -> &'static str {
        match self {
            Self::Local => "http://localhost:8082",
            Self::Sandbox => "https://sandbox.api-js.jicloud.org",
            Self::Release => "https://api-js.jicloud.org",
        }
    }

    pub fn media_url_base(&self) -> &'static str {
        match self {
            Self::Local => "http://localhost:4102",
            Self::Sandbox => "https://storage.googleapis.com/ji-cloud-eu",
            Self::Release => "https://storage.googleapis.com/ji-cloud-eu",
        }
    }


    pub fn host_url_base(&self) -> Option<&'static str> {
        None
    }
}