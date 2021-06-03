use super::ApiEndpoint;
use crate::{
    api::Method,
    domain::image::{
        CreateResponse, ImageBrowseQuery, ImageBrowseResponse, ImageCreateRequest, ImageResponse,
        ImageSearchQuery, ImageSearchResponse, ImageUpdateRequest,
    },
    error::{EmptyError, MetadataNotFound},
};

/// image routes for the user image library
pub mod user {
    use crate::{
        api::{ApiEndpoint, Method},
        domain::{
            image::{
                user::{UserImageListResponse, UserImageResponse},
                ImageId,
            },
            CreateResponse,
        },
        error::EmptyError,
    };

    /// List images.
    pub struct List;
    impl ApiEndpoint for List {
        type Req = ();
        type Res = UserImageListResponse;
        type Err = EmptyError;
        const PATH: &'static str = "/v1/user/me/image";
        const METHOD: Method = Method::Get;
    }

    // todo: list route
    /// Get an image by ID.
    pub struct Get;
    impl ApiEndpoint for Get {
        type Req = ();
        type Res = UserImageResponse;
        type Err = EmptyError;
        const PATH: &'static str = "/v1/user/me/image/{id}";
        const METHOD: Method = Method::Get;
    }
    /// Create an image.
    pub struct Create;
    impl ApiEndpoint for Create {
        type Req = ();
        type Res = CreateResponse<ImageId>;
        type Err = EmptyError;
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
        type Err = EmptyError;
        const PATH: &'static str = "/v1/user/me/image/{id}/raw";
        const METHOD: Method = Method::Put;
    }

    /// Delete an image.
    pub struct Delete;
    impl ApiEndpoint for Delete {
        type Req = ();
        type Res = ();
        type Err = EmptyError;
        const PATH: &'static str = "/v1/user/me/image/{id}";
        const METHOD: Method = Method::Delete;
    }
}

/// Routes to manage image tags
pub mod tag {
    use super::super::ApiEndpoint;
    use crate::{
        api::Method,
        domain::{
            image::tag::{
                ImageTagCreateRequest, ImageTagListResponse, ImageTagResponse,
                ImageTagUpdateRequest,
            },
        },
        error::EmptyError,
    };

    /// List all image tags.
    ///
    /// # Authorization
    /// Standard + [`UserScope::Admin`](crate::domain::user::UserScope)
    ///
    /// # Errors
    /// [`Unauthorized`](http::StatusCode::UNAUTHORIZED) if authorization is not valid.
    ///
    /// [`Forbidden`](http::StatusCode::FORBIDDEN) if the user does not have sufficient permission to perform the action.
    pub struct List;
    impl ApiEndpoint for List {
        type Req = ();
        type Res = ImageTagListResponse;
        type Err = EmptyError;
        const PATH: &'static str = "/v1/image/tag/all";
        const METHOD: Method = Method::Get;
    }

    /// Create an image tag.
    ///
    /// # Authorization
    /// Standard + [`UserScope::Admin`](crate::domain::user::UserScope)
    ///
    /// # Errors
    /// [`Unauthorized`](http::StatusCode::UNAUTHORIZED) if authorization is not valid.
    ///
    /// [`Forbidden`](http::StatusCode::FORBIDDEN) if the user does not have sufficient permission to perform the action.
    ///
    /// [`BadRequest`](http::StatusCode::BAD_REQUEST) if the request is missing/invalid.
    ///
    /// [`Conflict`](http::StatusCode::CONFLICT) if the requested `index` is already occupied.
    pub struct Create;
    impl ApiEndpoint for Create {
        type Req = ImageTagCreateRequest;
        type Res = ImageTagResponse;
        type Err = EmptyError;
        const PATH: &'static str = "/v1/image/tag/{index}";
        const METHOD: Method = Method::Post;
    }

    /// Update an image tag by index.
    ///
    /// # Authorization
    /// Standard + [`UserScope::Admin`](crate::domain::user::UserScope)
    ///
    /// # Errors
    /// [`Unauthorized`](http::StatusCode::UNAUTHORIZED) if authorization is not valid.
    ///
    /// [`Forbidden`](http::StatusCode::FORBIDDEN) if the user does not have sufficient permission to perform the action.
    ///
    /// [`InvalidRequest`](http::StatusCode::INVALID_REQUEST) if the request is missing/invalid.
    ///
    /// [`NotFound`](http::StatusCode::NOT_FOUND) if the image tag does not exist.
    ///
    /// [`Conflict`](http::StatusCode::CONFLICT) if the requested `index` is already occupied.
    pub struct Update;
    impl ApiEndpoint for Update {
        type Req = ImageTagUpdateRequest;
        type Res = ();
        type Err = EmptyError;
        const PATH: &'static str = "/v1/image/tag/{index}";
        const METHOD: Method = Method::Patch;
    }

    /// Delete an image tag by index.
    ///
    /// # Authorization
    /// Standard + [`UserScope::Admin`](crate::domain::user::UserScope)
    ///
    /// # Errors
    /// [`Unauthorized`](http::StatusCode::UNAUTHORIZED) if authorization is not valid.
    ///
    /// [`Forbidden`](http::StatusCode::FORBIDDEN) if the user does not have sufficient permission to perform the action.
    ///
    /// [`InvalidRequest`](http::StatusCode::INVALID_REQUEST) if the given `id` is not a [`Uuid`](uuid::Uuid) or the request is missing/invalid.
    ///
    /// [`NotFound`](http::StatusCode::NOT_FOUND) if the image tag does not exist.
    pub struct Delete;
    impl ApiEndpoint for Delete {
        type Req = ();
        type Res = ();
        type Err = EmptyError;
        const PATH: &'static str = "/v1/image/tag/{index}";
        const METHOD: Method = Method::Delete;
    }
}

/// Routes for a user's recently used images list.
/// Note: this assumes that the image referred to exists or is valid.
pub mod recent {
    use crate::{
        api::{ApiEndpoint, Method},
        domain::{
            image::{
                recent::{
                    UserRecentImageListRequest,
                    UserRecentImageListResponse,
                    UserRecentImageCreateRequest,
                    UserRecentImageResponse,
                },
            },
        },
        error::EmptyError,
    };

    /// List recent images for the user.
    /// Note: `limit` query is optional.
    ///
    /// # Errors
    /// * [`Unauthorized`](http::StatusCode::UNAUTHORIZED) if authorization is not valid.
    ///
    /// * ['BadRequest'](http::StatusCode::BAD_REQUEST) if the request is malformed.
    pub struct List;
    impl ApiEndpoint for List {
        type Req = UserRecentImageListRequest;
        type Res = UserRecentImageListResponse;
        type Err = EmptyError;
        const PATH: &'static str = "/v1/user/me/recent/image";
        const METHOD: Method = Method::Get;
    }

    /// Add an entry to the list of recent user images.
    ///
    /// # Errors
    /// * [`Unauthorized`](http::StatusCode::UNAUTHORIZED) if authorization is not valid.
    ///
    /// * ['BadRequest'](http::StatusCode::BAD_REQUEST) if the request is malformed (e.g. invalid uuid or ['MediaLibrary'](crate::media::MediaLibrary) enum given).
    pub struct Create;
    impl ApiEndpoint for Create {
        type Req = UserRecentImageCreateRequest;
        type Res = UserRecentImageResponse;
        type Err = EmptyError;
        const PATH: &'static str = "/v1/user/me/recent/image";
        const METHOD: Method = Method::Post;
    }

    /// Update an entry in the list of recent user images.
    /// Invoking this bumps the entry to the top of the recent images list.
    ///
    /// # Errors
    /// * [`Unauthorized`](http::StatusCode::UNAUTHORIZED) if authorization is not valid.
    ///
    /// * [`NotFound`](http::StatusCode::NOT_FOUND) if the image doesn't exist in the user's recent images list.
    ///
    /// * ['BadRequest'](http::StatusCode::BAD_REQUEST) if the request is malformed.
    pub struct Update;
    impl ApiEndpoint for Update {
        type Req = ();
        type Res = ();
        type Err = EmptyError;
        // uuid should be sufficient to identify an image, VERY unlikely to conflict across media libraries
        const PATH: &'static str = "/v1/user/me/recent/image/{id}";
        const METHOD: Method = Method::Patch;
    }

    /// Remove an entry from the list of recent user images.
    ///
    /// # Errors
    /// * [`Unauthorized`](http::StatusCode::UNAUTHORIZED) if authorization is not valid.
    ///
    /// * ['BadRequest'](http::StatusCode::BAD_REQUEST) if the request is malformed.
    pub struct Delete;
    impl ApiEndpoint for Delete {
        type Req = ();
        type Res = ();
        type Err = EmptyError;
        // uuid should be sufficient to identify an image, VERY unlikely to conflict across media libraries
        const PATH: &'static str = "/v1/user/me/recent/image/{id}";
        const METHOD: Method = Method::Delete;
    }
}

/// Get an image by ID.
pub struct Get;
impl ApiEndpoint for Get {
    type Req = ();
    type Res = ImageResponse;
    type Err = EmptyError;
    const PATH: &'static str = "/v1/image/{id}";
    const METHOD: Method = Method::Get;
}

/// Search for images.
pub struct Search;
impl ApiEndpoint for Search {
    type Req = ImageSearchQuery;
    type Res = ImageSearchResponse;
    type Err = EmptyError;
    const PATH: &'static str = "/v1/image";
    const METHOD: Method = Method::Get;
}

/// Browse images.
pub struct Browse;
impl ApiEndpoint for Browse {
    type Req = ImageBrowseQuery;
    type Res = ImageBrowseResponse;
    type Err = EmptyError;
    const PATH: &'static str = "/v1/image/browse";
    const METHOD: Method = Method::Get;
}

/// Create an image.
pub struct Create;
impl ApiEndpoint for Create {
    type Req = ImageCreateRequest;
    type Res = CreateResponse;
    type Err = MetadataNotFound;
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
    type Err = EmptyError;
    const PATH: &'static str = "/v1/image/{id}/raw";
    const METHOD: Method = Method::Patch;
}

/// Update an image's metadata.
pub struct UpdateMetadata;
impl ApiEndpoint for UpdateMetadata {
    type Req = ImageUpdateRequest;
    type Res = ();
    type Err = MetadataNotFound;
    const PATH: &'static str = "/v1/image/{id}";
    const METHOD: Method = Method::Patch;
}

/// Delete an image.
pub struct Delete;
impl ApiEndpoint for Delete {
    type Req = ();
    type Res = ();
    type Err = EmptyError;
    const PATH: &'static str = "/v1/image/{id}";
    const METHOD: Method = Method::Delete;
}
