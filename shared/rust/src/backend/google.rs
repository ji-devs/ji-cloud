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

pub async fn get_google_credentials() -> Result<GoogleCredentials, String> {
    env::var("GOOGLE_APPLICATION_CREDENTIALS")
        .map_err(|_| "no GOOGLE_APPLICATION_CREDENTIALS set".to_string())
        .and_then(|credentials_path| {
            File::open(credentials_path.clone()).map_err(|_| format!("couldn't open {}", credentials_path))
        })
        .and_then(|credentials_file| {
            let reader = BufReader::new(credentials_file);
            serde_json::from_reader(reader).map_err(|err| format!("{:?}", err))
        })
}

pub async fn get_google_token_from_credentials(credentials:&GoogleCredentials) -> Result<String, String> {

    let claims = GoogleApiClaims::new(&credentials);
    let token_assertion = jwt::encode(&jwt::Header::new(jwt::Algorithm::RS256), &claims, &jwt::EncodingKey::from_rsa_pem(credentials.private_key.as_bytes())
        .map_err(|_| "couldn't get encoding key".to_string())?
    )
    .map_err(|_| "couldn't encode jwt for google api request".to_string())?;

    let form = reqwest::multipart::Form::new()
        .text("grant_type", "urn:ietf:params:oauth:grant-type:jwt-bearer")
        .text("assertion", token_assertion);

    let token_response:GoogleAccessTokenResponse = reqwest::Client::new()
        .post("https://oauth2.googleapis.com/token")
        .multipart(form)
        .send()
        .and_then(|res| res.json())
        .await
        .map_err(|err| format!("couldn't get google access token: {:?}", err))?;
    
    Ok(token_response.access_token)
}

pub async fn get_google_token_from_metaserver() -> Result<String, String> {
    
    let url = "http://metadata.google.internal/computeMetadata/v1/instance/service-accounts/default/token";

    let token_response:GoogleAccessTokenResponse = reqwest::Client::new().get(url)
        .header("Metadata-Flavor","Google")
        .send()
        .and_then(|res| async move {
            //res.json()
            
            let text = res.text().await.expect("couldn't get response text to log");
            eprintln!("raw: {}", text); 
            let json = serde_json::from_str(&text).unwrap();
            Ok(json)

        })
        .await
        .map_err(|err| format!("couldn't get google access token from metaserver: {:?}", err))?;
    
    Ok(token_response.access_token)
}

pub async fn get_access_token_and_project_id() -> Result<(String, String), String> {
    let credentials = get_google_credentials().await;
    match credentials {
        Ok(credentials) => {
            let token = get_google_token_from_credentials(&credentials).await?;
            Ok((token, credentials.project_id))
        },
        Err(_) => {
            let project_id = env::var("PROJECT_ID").map_err(|_| "You must set PROJECT_ID as an env var since there's no GOOGLE_APPLICATION_CREDENTIALS".to_string())?;
            let token = get_google_token_from_metaserver().await?;
            Ok((token, project_id))
        }
    }
}

pub async fn get_secret(token:&str, project_id:&str, secret_name:&str) -> String {
    let api_name = format!("projects/{}/secrets/{}/versions/latest:access", project_id, secret_name);

    let path = format!("https://secretmanager.googleapis.com/v1beta1/{}", api_name);

    let request = reqwest::Client::new().get(&path)
        .header("Authorization",&format!("Bearer {}", token));

    let response:GoogleSecretResponse = request
        .send()
        .and_then(|res| res.json())
        /*
        .and_then(|res| async move {
            //res.json()
            
            let text = res.text().await.expect("couldn't get response text to log");
            eprintln!("raw: {}", text); 
            let json = serde_json::from_str(&text).unwrap();
            Ok(json)

        })
        */
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

        let credentials = get_google_credentials().await.unwrap();
        let token = get_google_token_from_credentials(&credentials).await.unwrap();
        let secret = get_secret(&token, &credentials.project_id, "SANITY_TEST").await;

        assert_eq!(secret, "hello_world");
    }
}
