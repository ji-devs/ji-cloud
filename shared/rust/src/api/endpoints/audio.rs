/// routes for the user audio library
pub mod user {
    use crate::{
        api::{endpoints::ApiEndpoint, Method},
        domain::{
            audio::{
                user::{
                    UserAudioCreatePath, UserAudioDeletePath, UserAudioGetPath, UserAudioListPath,
                    UserAudioListResponse, UserAudioResponse, UserAudioUploadPath,
                    UserAudioUploadRequest, UserAudioUploadResponse,
                },
                AudioId,
            },
            CreateResponse,
        },
        error::EmptyError,
    };

    /// List audio files.
    pub struct List;
    impl ApiEndpoint for List {
        type Path = UserAudioListPath;
        type Req = ();
        type Res = UserAudioListResponse;
        type Err = EmptyError;
        const METHOD: Method = Method::Get;
    }

    /// Get an audio file by ID.
    pub struct Get;
    impl ApiEndpoint for Get {
        type Path = UserAudioGetPath;
        type Req = ();
        type Res = UserAudioResponse;
        type Err = EmptyError;
        const METHOD: Method = Method::Get;
    }
    /// Create an audio file.
    pub struct Create;
    impl ApiEndpoint for Create {
        type Path = UserAudioCreatePath;
        type Req = ();
        type Res = CreateResponse<AudioId>;
        type Err = EmptyError;
        const METHOD: Method = Method::Post;
    }

    /// Upload an audio file. Returns a pre-signed URL for upload to Google Cloud Storage.
    ///
    /// Notes:
    /// * can be used to update the raw data associated with the audio file.
    pub struct Upload;
    impl ApiEndpoint for Upload {
        // raw bytes
        type Path = UserAudioUploadPath;
        type Req = UserAudioUploadRequest;
        type Res = UserAudioUploadResponse;
        type Err = EmptyError;
        const METHOD: Method = Method::Put;
    }

    /// Delete an audio file.
    pub struct Delete;
    impl ApiEndpoint for Delete {
        type Path = UserAudioDeletePath;
        type Req = ();
        type Res = ();
        type Err = EmptyError;
        const METHOD: Method = Method::Delete;
    }
}
