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
pub const DB_POOL_CONNECTIONS: u32 = 5;
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
/// Image file size limit. 10 MB
pub const IMAGE_BODY_SIZE_LIMIT: usize = 1024 * 1024 * 10;
/// Animation file size limit. 15 MB
/// Animations are reasonably expected to be larger than normal images?
pub const ANIMATION_BODY_SIZE_LIMIT: usize = 1024 * 1024 * 15;
/// JSON body size limit for both requests and responses
pub const JSON_BODY_LIMIT: u64 = 1024 * 16; // 16

//////////////
// Other
/// Defines the range of possible values for Jig player session sharing codes
/// /// means 0-9999 are possible. If this is changed then the DB's check constraint must also be updated.
pub const JIG_PLAYER_SESSION_CODE_MAX: i16 = 9999;
/// `MAX_SIGNIN_COOKIE_DURATION` but as seconds,
/// as there's no way to get the string number of seconds from it `const`ly
#[deprecated = "use `MAX_SIGNIN_COOKIE_DURATION.whole_seconds()` instead"]
pub const MAX_SIGNIN_COOKIE: &str = "1209600";
