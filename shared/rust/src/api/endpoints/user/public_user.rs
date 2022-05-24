use crate::{
    api::{ApiEndpoint, Method},
    domain::user::public_user::{
        BrowsePublicUserCoursesQuery, BrowsePublicUserCoursesResponse,
        BrowsePublicUserFollowersQuery, BrowsePublicUserFollowersResponse,
        BrowsePublicUserFollowingResponse, BrowsePublicUserFollowingsQuery,
        BrowsePublicUserJigsQuery, BrowsePublicUserJigsResponse, BrowsePublicUserResourcesQuery,
        BrowsePublicUserResourcesResponse, BrowsePublicUserResponse, PublicUser, UserBrowseQuery,
    },
    error::EmptyError,
};

/// Fetch user profile.
pub struct Get;
impl ApiEndpoint for Get {
    type Req = ();
    type Res = PublicUser;
    type Err = EmptyError;
    const PATH: &'static str = "/v1/user/{user_id}/public";
    const METHOD: Method = Method::Get;
}

/// Browse public user
pub struct BrowsePublicUser;
impl ApiEndpoint for BrowsePublicUser {
    type Req = UserBrowseQuery;
    type Res = BrowsePublicUserResponse;
    type Err = EmptyError;
    const PATH: &'static str = "/v1/user/public/browse";
    const METHOD: Method = Method::Get;
}

/// Search user profile
pub struct SearchPublicUser;
impl ApiEndpoint for SearchPublicUser {
    type Req = ();
    type Res = BrowsePublicUserResponse;
    type Err = EmptyError;
    const PATH: &'static str = "/v1/user/public";
    const METHOD: Method = Method::Get;
}

/// Browse public user
pub struct BrowseUserJigs;
impl ApiEndpoint for BrowseUserJigs {
    type Req = BrowsePublicUserJigsQuery;
    type Res = BrowsePublicUserJigsResponse;
    type Err = EmptyError;
    const PATH: &'static str = "/v1/user/{user_id}/public/jig/browse";
    const METHOD: Method = Method::Get;
}

/// Browse user's resources
pub struct BrowseUserResources;
impl ApiEndpoint for BrowseUserResources {
    type Req = BrowsePublicUserResourcesQuery;
    type Res = BrowsePublicUserResourcesResponse;
    type Err = EmptyError;
    const PATH: &'static str = "/v1/user/{user_id}/public/resource/browse";
    const METHOD: Method = Method::Get;
}

/// Browse user's courses
pub struct BrowseUserCourses;
impl ApiEndpoint for BrowseUserCourses {
    type Req = BrowsePublicUserCoursesQuery;
    type Res = BrowsePublicUserCoursesResponse;
    type Err = EmptyError;
    const PATH: &'static str = "/v1/user/{user_id}/public/course/browse";
    const METHOD: Method = Method::Get;
}

/// Browse user's followers
pub struct BrowseFollowers;
impl ApiEndpoint for BrowseFollowers {
    type Req = BrowsePublicUserFollowersQuery;
    type Res = BrowsePublicUserFollowersResponse;
    type Err = EmptyError;
    const PATH: &'static str = "/v1/user/{user_id}/public/follower/browse";
    const METHOD: Method = Method::Get;
}

/// Browse user's followings
pub struct BrowseFollowing;
impl ApiEndpoint for BrowseFollowing {
    type Req = BrowsePublicUserFollowingsQuery;
    type Res = BrowsePublicUserFollowingResponse;
    type Err = EmptyError;
    const PATH: &'static str = "/v1/user/{user_id}/public/following/browse";
    const METHOD: Method = Method::Get;
}

/// Follow a user
pub struct Follow;
impl ApiEndpoint for Follow {
    type Req = ();
    type Res = ();
    type Err = EmptyError;
    const PATH: &'static str = "/v1/user/{user_id}/follow";
    const METHOD: Method = Method::Post;
}

/// Unfollow a user
pub struct Unfollow;
impl ApiEndpoint for Unfollow {
    type Req = ();
    type Res = ();
    type Err = EmptyError;
    const PATH: &'static str = "/v1/user/{user_id}/unfollow";
    const METHOD: Method = Method::Post;
}
