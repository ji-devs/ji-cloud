/// routes for the user pdf library
pub mod user {
    use crate::{
        api::{ApiEndpoint, Method},
        domain::{
            pdf::{
                user::{
                    UserPdfCreatePath, UserPdfDeletePath, UserPdfGetPath, UserPdfListPath,
                    UserPdfListResponse, UserPdfResponse, UserPdfUploadPath, UserPdfUploadRequest,
                    UserPdfUploadResponse,
                },
                PdfId,
            },
            CreateResponse,
        },
        error::EmptyError,
    };

    /// List pdf files.
    pub struct List;
    impl ApiEndpoint for List {
        type Path = UserPdfListPath;
        type Req = ();
        type Res = UserPdfListResponse;
        type Err = EmptyError;
        const METHOD: Method = Method::Get;
    }

    /// Get an pdf file by ID.
    pub struct Get;
    impl ApiEndpoint for Get {
        type Path = UserPdfGetPath;
        type Req = ();
        type Res = UserPdfResponse;
        type Err = EmptyError;
        const METHOD: Method = Method::Get;
    }

    /// Create a pdf file.
    pub struct Create;
    impl ApiEndpoint for Create {
        type Path = UserPdfCreatePath;
        type Req = ();
        type Res = CreateResponse<PdfId>;
        type Err = EmptyError;
        const METHOD: Method = Method::Post;
    }

    /// Upload a pdf file. Returns a pre-signed URL for upload to Google Cloud Storage.
    ///
    /// Notes:
    /// * can be used to update the raw data associated with the pdf file.
    pub struct Upload;
    impl ApiEndpoint for Upload {
        type Path = UserPdfUploadPath;
        // raw bytes
        type Req = UserPdfUploadRequest;
        type Res = UserPdfUploadResponse;
        type Err = EmptyError;
        const METHOD: Method = Method::Put;
    }

    /// Delete a pdf file.
    pub struct Delete;
    impl ApiEndpoint for Delete {
        type Path = UserPdfDeletePath;
        type Req = ();
        type Res = ();
        type Err = EmptyError;
        const METHOD: Method = Method::Delete;
    }
}
