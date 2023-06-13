use crate::{
    api::Method,
    domain::{
        playlist::{
            PlaylistBrowsePath, PlaylistBrowseQuery, PlaylistBrowseResponse, PlaylistClonePath,
            PlaylistCreatePath, PlaylistCreateRequest, PlaylistDeletePath, PlaylistGetDraftPath,
            PlaylistGetLivePath, PlaylistId, PlaylistLikePath, PlaylistLikedPath,
            PlaylistLikedResponse, PlaylistPublishPath, PlaylistResponse, PlaylistSearchPath,
            PlaylistSearchQuery, PlaylistSearchResponse, PlaylistUnlikePath,
            PlaylistUpdateDraftDataPath, PlaylistUpdateDraftDataRequest, PlaylistViewPath,
        },
        CreateResponse,
    },
    error::{EmptyError, MetadataNotFound},
};

use super::ApiEndpoint;

/// Create a Playlist and it's draft and live data copies.
///
/// * New Playlists are all set to `PrivacyLevel::Unlisted` by default
///
/// # Flow:
/// 1. Create a Playlist and its two data copies with [`Create`]
/// 2. Optionally update Playlist info such as privacy, author with [`Update`]
/// 3. Make updates to draft data:
///     a. Patch Playlist data through [`UpdateDraftData`]

/// 4. Finalize draft changes by calling [`Publish`]
///
/// # Authorization
/// * TokenUser
/// * One of `Admin`, `AdminAsset`, or `ManageSelfAsset`
pub struct Create;
impl ApiEndpoint for Create {
    type Req = PlaylistCreateRequest;
    type Res = CreateResponse<PlaylistId>;
    type Path = PlaylistCreatePath;
    type Err = MetadataNotFound;
    const METHOD: Method = Method::Post;
}

/// Get a Playlist's live data by ID.
///
/// # Authorization
/// * Creator ID of Playlist
/// * One of `Admin`, `AdminAsset`,, or `ManageSelfAsset` for owned Playlists
///
/// # Errors
///
pub struct GetLive;
impl ApiEndpoint for GetLive {
    type Req = ();
    type Res = PlaylistResponse;
    type Path = PlaylistGetLivePath;
    type Err = EmptyError;
    const METHOD: Method = Method::Get;
}

/// Get a Playlist's draft data by ID.
///
/// # Authorization
/// * Creator ID of Playlist
/// * One of `Admin`, `AdminAsset`,, or `ManageSelfAsset` for owned Playlists
///
/// # Errors
/// * [`Unauthorized`](http::StatusCode::UNAUTHORIZED) if authorization is not valid.
///
pub struct GetDraft;
impl ApiEndpoint for GetDraft {
    type Req = ();
    type Res = PlaylistResponse;
    type Path = PlaylistGetDraftPath;
    type Err = EmptyError;
    const METHOD: Method = Method::Get;
}

/// Update the draft data of a Playlist.
///
/// Note that a copy of the Playlist's draft or live data can not be fetched directly, but only as a part
/// of one of the following routes:
/// * [`GetLive`] fetches live copies
/// * [`Search`]
///
/// See [`Playlist Data`](crate::domain::playlist::PlaylistData) for the over-the-wire representation.
///
/// # Authorization
/// * One of `Admin`, `AdminAsset`, or `ManageSelfAsset` for owned Playlists
pub struct UpdateDraftData;
impl ApiEndpoint for UpdateDraftData {
    type Req = PlaylistUpdateDraftDataRequest;
    type Res = ();
    type Path = PlaylistUpdateDraftDataPath;
    type Err = MetadataNotFound;
    const METHOD: Method = Method::Patch;
}

/// Publish a Playlist draft to live by copying over the Playlistdata.
///
/// # Authorization
/// * Creator ID of Playlist
/// * One of `Admin`, `AdminAsset`, or `ManageSelfAsset`
pub struct Publish;
impl ApiEndpoint for Publish {
    type Req = ();
    type Res = ();
    type Path = PlaylistPublishPath;
    type Err = EmptyError;
    const METHOD: Method = Method::Put;
}

/// Browse Playlists. Returns the draft data copies in the response.
///
/// # Authorization
/// * None
pub struct Browse;
impl ApiEndpoint for Browse {
    type Req = PlaylistBrowseQuery;
    type Res = PlaylistBrowseResponse;
    type Path = PlaylistBrowsePath;
    type Err = EmptyError;
    const METHOD: Method = Method::Get;
}

/// Search for Playlists.
///
/// # Authorization
/// * None
pub struct Search;
impl ApiEndpoint for Search {
    type Req = PlaylistSearchQuery;
    type Res = PlaylistSearchResponse;
    type Path = PlaylistSearchPath;
    type Err = EmptyError;
    const METHOD: Method = Method::Get;
}

/// Delete a Playlist.
///
/// # Authorization
/// * Creator ID of Playlist
/// * One of `Admin`, `AdminAsset`, or `ManageSelfAsset` for owned Playlists
pub struct Delete;
impl ApiEndpoint for Delete {
    type Req = ();
    type Res = ();
    type Path = PlaylistDeletePath;
    type Err = EmptyError;
    const METHOD: Method = Method::Delete;
}

/// Clone a Playlist. This clones both the draft and live.
///
/// # Authorization
/// * One of `Admin`, `AdminAsset`, or `ManageSelfAsset`
///
/// # Errors
/// * [`Unauthorized`](http::StatusCode::UNAUTHORIZED) if authorization is not valid.
/// * [`Forbidden`](http::StatusCode::FORBIDDEN) if the user does not have sufficient permission to perform the action.
/// * ['NotFound'](http::StatusCode::NOT_FOUND) if the resource does not exist.
/// * ['BadRequest'](http::StatusCode::BAD_REQUEST) if the request is malformed or the Playlist is a draft.
pub struct Clone;
impl ApiEndpoint for Clone {
    type Path = PlaylistClonePath;
    type Req = ();
    type Res = CreateResponse<PlaylistId>;
    type Err = EmptyError;
    const METHOD: Method = Method::Post;
}

/// Like a Playlist
///
/// # Authorization
/// * Admin, BasicAuth
pub struct Like;
impl ApiEndpoint for Like {
    type Path = PlaylistLikePath;
    type Req = ();
    type Res = ();
    type Err = EmptyError;
    const METHOD: Method = Method::Put;
}

/// Unlike a Playlist
///
/// # Authorization
/// * Admin, BasicAuth
pub struct Unlike;
impl ApiEndpoint for Unlike {
    type Path = PlaylistUnlikePath;
    type Req = ();
    type Res = ();
    type Err = EmptyError;
    const METHOD: Method = Method::Delete;
}

/// Is a Playlist liked by a user
///
/// # Authorization
/// * Admin, BasicAuth
pub struct Liked;
impl ApiEndpoint for Liked {
    type Path = PlaylistLikedPath;
    type Req = ();
    type Res = PlaylistLikedResponse;
    type Err = EmptyError;
    const METHOD: Method = Method::Get;
}

/// View a Playlist
///
/// # Authorization
/// * None
pub struct View;
impl ApiEndpoint for View {
    type Path = PlaylistViewPath;
    type Req = ();
    type Res = ();
    type Err = EmptyError;
    const METHOD: Method = Method::Put;
}
