//////////////
// External APIs
/// JWK issuer
pub const JWK_ISSUER_URL: &str = "https://accounts.google.com";
/// URL to JWK verifier
pub const JWK_URL: &str = "https://www.googleapis.com/oauth2/v3/certs";
/// Service name for handling Auditlog push events
pub const EVENTARC_AUDITLOG_SERVICE_NAME: &str = "cloudaudit.googleapis.com";

//////////////
// Database
/// Sandbox target
pub const DB_INSTANCE_SANDBOX: &str =
    "ji-cloud-developer-sandbox:europe-west1:ji-cloud-003-sandbox";
/// Release target
pub const DB_INSTANCE_RELEASE: &str = "ji-cloud:europe-west1:ji-cloud-002";
/// Maximum # of DB pools
pub const DB_POOL_CONNECTIONS: u32 = 20;
/// User and DB owner
pub const REMOTE_DB_USER: &str = "postgres";
/// Name of the DB
pub const REMOTE_DB_NAME: &str = "jicloud";
/// SQL proxy port.
/// Must match the port number in build-utils/package.json where cloud-sql-proxy is launched
pub const SQL_PROXY_PORT: u16 = 6432;

//////////////
// Requests
/// Timeout interval for media uploading
pub const MEDIA_UPLOAD_TIMEOUT_SECS: u64 = 300;
/// Image file size limit. 30 MB
pub const IMAGE_BODY_SIZE_LIMIT: usize = 1024 * 1024 * 30;
/// Animation file size limit. 40 MB
/// Animations are reasonably expected to be larger than normal images?
pub const ANIMATION_BODY_SIZE_LIMIT: usize = 1024 * 1024 * 40;
/// Audio file size limit. 30 MB
pub const AUDIO_BODY_SIZE_LIMIT: usize = 1024 * 1024 * 30;
/// PDF file size limit. 10 MB
pub const PDF_BODY_SIZE_LIMIT: usize = 1024 * 1024 * 10;
/// JSON body size limit for both requests and responses. 256 KB
pub const JSON_BODY_LIMIT: u64 = 1024 * 256;
/// Allowed CORS origins
pub const CORS_ORIGINS: &[&str] = &[
    "https://jigzi.org",
    "https://sandbox.jigzi.org",
    "https://api.jigzi.org",
    "https://api.sandbox.jigzi.org",
    "http://localhost:4104",
    "http://localhost:4105",
];

//////////////
// Other
/// `MAX_SIGNIN_COOKIE_DURATION` but as seconds,
/// as there's no way to get the string number of seconds from it `const`ly
#[deprecated = "use `MAX_SIGNIN_COOKIE_DURATION.whole_seconds()` instead"]
pub const MAX_SIGNIN_COOKIE: &str = "1209600";

//////////////
// Media and uploads
/// Runs cleanup on failed up loads. Should be less than the `UPLOAD_EXPIRY_TIME`.
pub const EXPIRED_UPLOAD_CLEANUP_PERIOD: u64 = 60 * 60 * 6;
/// Any media created at least this old will be deleted from the database if it has not successfully
/// completed processing by the time the cleaning task has
pub const UPLOAD_EXPIRY_TIME: u64 = 60 * 60 * 24 * 3;
