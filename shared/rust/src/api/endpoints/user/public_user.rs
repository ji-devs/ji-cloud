use super::super::ApiEndpoint;
use crate::{
    api::Method,
    domain::{
        jig::JigBrowseResponse,
        playlist::PlaylistBrowseResponse,
        user::public_user::{
            BrowsePublicUserFollowersPath, BrowsePublicUserFollowersQuery,
            BrowsePublicUserFollowersResponse, BrowsePublicUserFollowingPath,
            BrowsePublicUserFollowingResponse, BrowsePublicUserFollowingsQuery,
            BrowsePublicUserJigsPath, BrowsePublicUserJigsQuery, BrowsePublicUserPlaylistsPath,
            BrowsePublicUserPlaylistsQuery, BrowsePublicUserResourcesPath,
            BrowsePublicUserResourcesQuery, BrowsePublicUserResourcesResponse,
            BrowsePublicUserResponse, PublicUser, PublicUserBrowsePath, PublicUserFollowPath,
            PublicUserGetPath, PublicUserSearchPath, PublicUserUnfollowPath, SearchPublicUserQuery,
            SearchPublicUserResponse, UserBrowseQuery,
        },
    },
    error::EmptyError,
};

/// Fetch user profile.
pub struct Get;
impl ApiEndpoint for Get {
    type Req = ();
    type Res = PublicUser;
    type Path = PublicUserGetPath;
    type Err = EmptyError;
    const METHOD: Method = Method::Get;
}

/// Browse public user
pub struct BrowsePublicUser;
impl ApiEndpoint for BrowsePublicUser {
    type Req = UserBrowseQuery;
    type Res = BrowsePublicUserResponse;
    type Path = PublicUserBrowsePath;
    type Err = EmptyError;
    const METHOD: Method = Method::Get;
}

/// Search user profile
pub struct SearchPublicUser;
impl ApiEndpoint for SearchPublicUser {
    type Req = SearchPublicUserQuery;
    type Res = SearchPublicUserResponse;
    type Path = PublicUserSearchPath;
    type Err = EmptyError;
    const METHOD: Method = Method::Get;
}

/// Browse public user
pub struct BrowseUserJigs;
impl ApiEndpoint for BrowseUserJigs {
    type Req = BrowsePublicUserJigsQuery;
    type Res = JigBrowseResponse;
    type Path = BrowsePublicUserJigsPath;
    type Err = EmptyError;
    const METHOD: Method = Method::Get;
}

/// Browse user's resources
pub struct BrowseUserResources;
impl ApiEndpoint for BrowseUserResources {
    type Req = BrowsePublicUserResourcesQuery;
    type Res = BrowsePublicUserResourcesResponse;
    type Path = BrowsePublicUserResourcesPath;
    type Err = EmptyError;
    const METHOD: Method = Method::Get;
}

/// Browse user's playlists
pub struct BrowseUserPlaylists;
impl ApiEndpoint for BrowseUserPlaylists {
    type Req = BrowsePublicUserPlaylistsQuery;
    type Res = PlaylistBrowseResponse;
    type Path = BrowsePublicUserPlaylistsPath;
    type Err = EmptyError;
    const METHOD: Method = Method::Get;
}

/// Browse user's followers
pub struct BrowseFollowers;
impl ApiEndpoint for BrowseFollowers {
    type Req = BrowsePublicUserFollowersQuery;
    type Res = BrowsePublicUserFollowersResponse;
    type Path = BrowsePublicUserFollowersPath;
    type Err = EmptyError;
    const METHOD: Method = Method::Get;
}

/// Browse user's followings
pub struct BrowseFollowing;
impl ApiEndpoint for BrowseFollowing {
    type Req = BrowsePublicUserFollowingsQuery;
    type Res = BrowsePublicUserFollowingResponse;
    type Path = BrowsePublicUserFollowingPath;
    type Err = EmptyError;
    const METHOD: Method = Method::Get;
}

/// Follow a user
pub struct Follow;
impl ApiEndpoint for Follow {
    type Req = ();
    type Res = ();
    type Path = PublicUserFollowPath;
    type Err = EmptyError;
    const METHOD: Method = Method::Post;
}

/// Unfollow a user
pub struct Unfollow;
impl ApiEndpoint for Unfollow {
    type Req = ();
    type Res = ();
    type Path = PublicUserUnfollowPath;
    type Err = EmptyError;
    const METHOD: Method = Method::Delete;
}
