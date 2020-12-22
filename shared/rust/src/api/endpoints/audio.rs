/// routes for the user audio library
pub mod user {
    use crate::{
        api::{ApiEndpoint, Method},
        domain::{
            audio::{
                user::{UserAudioListResponse, UserAudioResponse},
                AudioId,
            },
            CreateResponse,
        },
        error::EmptyError,
    };

    /// List audio files.
    pub struct List;
    impl ApiEndpoint for List {
        type Req = ();
        type Res = UserAudioListResponse;
        type Err = EmptyError;
        const PATH: &'static str = "/v1/user/me/audio";
        const METHOD: Method = Method::Get;
    }

    /// Get an audio file by ID.
    pub struct Get;
    impl ApiEndpoint for Get {
        type Req = ();
        type Res = UserAudioResponse;
        type Err = EmptyError;
        const PATH: &'static str = "/v1/user/me/audio/{id}";
        const METHOD: Method = Method::Get;
    }
    /// Create an audio file.
    pub struct Create;
    impl ApiEndpoint for Create {
        type Req = ();
        type Res = CreateResponse<AudioId>;
        type Err = EmptyError;
        const PATH: &'static str = "/v1/user/me/audio";
        const METHOD: Method = Method::Post;
    }

    /// Upload an audio file
    /// Note: can be used to update the raw data associated with the audio file.
    pub struct Upload;
    impl ApiEndpoint for Upload {
        // raw bytes
        type Req = ();
        type Res = ();
        type Err = EmptyError;
        const PATH: &'static str = "/v1/user/me/audio/{id}/raw";
        const METHOD: Method = Method::Put;
    }

    /// Delete an audio file.
    pub struct Delete;
    impl ApiEndpoint for Delete {
        type Req = ();
        type Res = ();
        type Err = EmptyError;
        const PATH: &'static str = "/v1/user/me/audio/{id}";
        const METHOD: Method = Method::Delete;
    }
}
