use serde::{Deserialize, Serialize, de::DeserializeOwned};

#[derive(Serialize, Deserialize, Debug)]
pub struct HttpStatus {
    pub code: u16,
    pub message: String,
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type", content = "content", rename_all = "lowercase")]
pub enum ResultResponse<T: Serialize + DeserializeOwned, E: Serialize + DeserializeOwned>{
    #[serde(deserialize_with = "T::deserialize")]
    Ok(T),
    #[serde(deserialize_with = "E::deserialize")]
    Err(E)
}

impl <T: Serialize + DeserializeOwned, E: Serialize + DeserializeOwned> ResultResponse<T, E> {
    pub fn unwrap(self) -> T {
        match self {
            ResultResponse::Ok(x) => x,
            ResultResponse::Err(x) => panic!("Could not unwrap ResultResponse. Error: [{}]", serde_json::to_string(&x).unwrap()),
        }
    }
}

impl <T: Serialize + DeserializeOwned,E: Serialize + DeserializeOwned> From<ResultResponse<T,E>> for Result<T,E> {
    fn from(resp:ResultResponse<T, E>) -> Result<T,E> {
        match resp {
            ResultResponse::Ok(x) => Result::Ok(x),
            ResultResponse::Err(x) => Result::Err(x),
        }
    }
}

impl <T: Serialize + DeserializeOwned,E: Serialize + DeserializeOwned> From<Result<T,E>> for ResultResponse<T,E> {
    fn from(res:Result<T, E>) -> ResultResponse<T,E> {
        match res {
            Result::Ok(x) => ResultResponse::Ok(x),
            Result::Err(x) => ResultResponse::Err(x),
        }
    }
}
