use futures_util::future::TryFutureExt;
use serde::{Deserialize, Serialize};
use std::{
    env,
    time::{Duration, SystemTime, UNIX_EPOCH},
    fs::File,
    io::BufReader,
};
use jsonwebtoken as jwt;

#[derive(Deserialize)]
pub struct GoogleCredentials {
    pub private_key: String,
    pub project_id: String,
    pub client_email: String,
}

#[derive(Deserialize)]
pub struct GoogleAccessTokenResponse {
    pub access_token: String,
}

#[derive(Deserialize)]
pub struct GoogleSecretResponse {
    pub payload: GoogleSecretResponsePayload
}
#[derive(Deserialize)]
pub struct GoogleSecretResponsePayload {
    pub data: String 
}

#[derive(Serialize, Debug)]
pub struct GoogleApiClaims<'a> {
    iss: &'a str,
    scope: &'static str,
    aud: &'static str,
    exp: u64,
    iat: u64,
}

impl <'a> GoogleApiClaims <'a> {
    pub fn new(credentials:&'a GoogleCredentials) -> Self {
        Self {
            iss: &credentials.client_email,
            scope: "https://www.googleapis.com/auth/cloud-platform",
            aud: "https://oauth2.googleapis.com/token",
            exp: (SystemTime::now() + Duration::from_secs(3600)).duration_since(UNIX_EPOCH).expect("get duration").as_secs(),
            iat: SystemTime::now().duration_since(UNIX_EPOCH).expect("get duration").as_secs()
        }
    }
}

pub async fn get_google_credentials() -> GoogleCredentials {
    let credentials_path = env::var("GOOGLE_APPLICATION_CREDENTIALS").expect("must have GOOGLE_APPLICATION_CREDENTIALS set");
    let credentials_file = File::open(credentials_path.clone()).expect(&format!("couldn't read {}", credentials_path));
    let reader = BufReader::new(credentials_file);
    //let credentials_str = std::fs::read_to_string(credentials_file.clone()).expect(&format!("couldn't read {}", credentials_file));

    serde_json::from_reader(reader).expect("couldn't read google credentials, even though file exists")
}

pub async fn get_google_token(credentials:&GoogleCredentials) -> String {

    let claims = GoogleApiClaims::new(&credentials);
    let token_assertion = jwt::encode(&jwt::Header::new(jwt::Algorithm::RS256), &claims, &jwt::EncodingKey::from_rsa_pem(credentials.private_key.as_bytes()).expect("couldn't get encoding key")).expect("couldn't encode jwt for google api request");

    let form = reqwest::multipart::Form::new()
        .text("grant_type", "urn:ietf:params:oauth:grant-type:jwt-bearer")
        .text("assertion", token_assertion);

    let token_response:GoogleAccessTokenResponse = reqwest::Client::new()
        .post("https://oauth2.googleapis.com/token")
        .multipart(form)
        .send()
        .and_then(|res| res.json())
        .await
        .expect("couldn't get google access token");
    
    token_response.access_token
}

pub async fn get_secret(token:&str, project_id:&str, secret_name:&str) -> String {
    let api_name = format!("projects/{}/secrets/{}/versions/latest:access", project_id, secret_name);

    let path = format!("https://secretmanager.googleapis.com/v1beta1/{}", api_name);
    let response:GoogleSecretResponse = reqwest::Client::new()
        .get(&path)
        .header("Authorization",&format!("Bearer {}", token))
        .send()
        .and_then(|res| res.json())
        .await
        .expect(&format!("couldn't get secret: {}", secret_name));

    let bytes:Vec<u8> = base64::decode(response.payload.data).unwrap();
    std::str::from_utf8(&bytes).unwrap().to_string()
}

#[cfg(all(test, feature = "has_google_auth"))]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_secrets() {

        dotenv::dotenv().ok();

        let credentials = get_google_credentials().await;
        let token = get_google_token(&credentials).await;
        let secret = get_secret(&token, &credentials.project_id, "SANITY_TEST").await;

        assert_eq!(secret, "hello_world");
    }
}
