use super::ApiEndpoint;
use crate::{
    api::Method,
    domain::image::{
        ImageCreateRequest, CreateResponse, ImageResponse, ImageSearchQuery, ImageSearchResponse, ImageUpdateRequest,
    },
    error::{
        image::{CreateError, SearchError, UpdateError, UploadError},
        DeleteError, GetError,
    },
};

/// image routes for the user image library
pub mod user {
    use crate::{
        api::{ApiEndpoint, Method},
        domain::{
            image::{
                user::{UserImageResponse, UserImageListResponse},
                ImageId,
            },
            CreateResponse,
        },
        error::{image::UploadError, CreateError, DeleteError, GetError},
    };

    /// List images.
    pub struct List;
    impl ApiEndpoint for List {
        type Req = ();
        type Res = UserImageListResponse;
        type Err = GetError;
        const PATH: &'static str = "/v1/user/me/image";
        const METHOD: Method = Method::Get;
    }

    // todo: list route
    /// Get an image by ID.
    pub struct Get;
    impl ApiEndpoint for Get {
        type Req = ();
        type Res = UserImageResponse;
        type Err = GetError;
        const PATH: &'static str = "/v1/user/me/image/{id}";
        const METHOD: Method = Method::Get;
    }
    /// Create an image.
    pub struct Create;
    impl ApiEndpoint for Create {
        type Req = ();
        type Res = CreateResponse<ImageId>;
        type Err = CreateError;
        const PATH: &'static str = "/v1/user/me/image";
        const METHOD: Method = Method::Post;
    }

    /// Upload an image
    /// Note: can be used to update the raw data associated with the image.
    pub struct Upload;
    impl ApiEndpoint for Upload {
        // raw bytes
        type Req = ();
        type Res = ();
        type Err = UploadError;
        const PATH: &'static str = "/v1/user/me/image/{id}/raw";
        const METHOD: Method = Method::Put;
    }

    /// Delete an image.
    pub struct Delete;
    impl ApiEndpoint for Delete {
        type Req = ();
        type Res = ();
        type Err = DeleteError;
        const PATH: &'static str = "/v1/user/me/image/{id}";
        const METHOD: Method = Method::Delete;
    }
}

/// Get an image by ID.
pub struct Get;
impl ApiEndpoint for Get {
    type Req = ();
    type Res = ImageResponse;
    type Err = GetError;
    const PATH: &'static str = "/v1/image/{id}";
    const METHOD: Method = Method::Get;
}

/// Search for images.
pub struct Search;
impl ApiEndpoint for Search {
    type Req = ImageSearchQuery;
    type Res = ImageSearchResponse;
    type Err = SearchError;
    const PATH: &'static str = "/v1/image";
    const METHOD: Method = Method::Get;
}

/// Create an image.
pub struct Create;
impl ApiEndpoint for Create {
    type Req = ImageCreateRequest;
    type Res = CreateResponse;
    type Err = CreateError;
    const PATH: &'static str = "/v1/image";
    const METHOD: Method = Method::Post;
}

/// Upload an image
/// Note: can be used to update the raw data associated with the image.
pub struct Upload;
impl ApiEndpoint for Upload {
    // raw bytes
    type Req = ();
    type Res = ();
    type Err = UploadError;
    const PATH: &'static str = "/v1/image/{id}/raw";
    const METHOD: Method = Method::Patch;
}

/// Update an image's metadata.
pub struct UpdateMetadata;
impl ApiEndpoint for UpdateMetadata {
    type Req = ImageUpdateRequest;
    type Res = ();
    type Err = UpdateError;
    const PATH: &'static str = "/v1/image/{id}";
    const METHOD: Method = Method::Patch;
}

/// Delete an image.
pub struct Delete;
impl ApiEndpoint for Delete {
    type Req = ();
    type Res = ();
    type Err = DeleteError;
    const PATH: &'static str = "/v1/image/{id}";
    const METHOD: Method = Method::Delete;
}
