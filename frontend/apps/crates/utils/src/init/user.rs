use once_cell::sync::OnceCell;
use shared::domain::user::UserProfile;

static USER: OnceCell<UserProfile> = OnceCell::new();

pub(crate) async fn init() {
    // let (result, status) = Profile::api_with_auth_status(None).await;

    // match result {
    //     Ok(user) if status != 401 || status != 403 => {
    //         // `USER` is private and the only way to initialize it is through `init` - `set()`
    //         // should never fail at this point.
    //         let _ = USER.set(user);
    //     },
    //     _ => {},
    // }
}

pub fn get_user() -> Option<&'static UserProfile> {
    USER.get()
}
