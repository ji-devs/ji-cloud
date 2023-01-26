use std::rc::Rc;

use dominator::clone;
use shared::{
    api::endpoints,
    domain::user::{
        UserBrowsePath, UserBrowseQuery, UserResponse, UserSearchPath, UserSearchQuery,
    },
};
use utils::{
    prelude::ApiEndpointExt,
    routes::{AdminRoute, AdminUsersRoute, Route},
};

use super::{FetchMode, Users};

impl Users {
    pub fn load_data(self: &Rc<Self>) {
        let state = Rc::clone(self);
        state.loader.load(clone!(state => async move {
                state.load_users().await
        }));
    }

    pub async fn load_users(self: &Rc<Self>) {
        // clone right away to free the lock
        let fetch_mode = self.fetch_mode.borrow().clone();
        let res = match fetch_mode {
            FetchMode::Browse => self.load_users_browse().await,
            FetchMode::Search(query) => self.load_users_search(query.clone()).await,
        };

        self.users.lock_mut().replace_cloned(
            res.users
                .into_iter()
                .map(|user| Rc::new(user.into()))
                .collect(),
        );
        // self.set_total_page(res.total_page);

        self.total_pages.set_neq(Some(res.total_pages));
    }

    async fn load_users_browse(&self) -> UserListResponse {
        let req = UserBrowseQuery {
            page: Some(self.active_page.get()),
            ..Default::default()
        };

        match endpoints::user::Browse::api_with_auth(UserBrowsePath(), Some(req)).await {
            Err(_) => todo!(),
            Ok(res) => UserListResponse {
                users: res.users,
                total_pages: res.pages,
            },
        }
    }

    async fn load_users_search(&self, query: String) -> UserListResponse {
        let req = UserSearchQuery {
            q: query,
            page: Some(self.active_page.get()),
            ..Default::default()
        };

        match endpoints::user::SearchUser::api_with_auth(UserSearchPath(), Some(req)).await {
            Err(_) => todo!(),
            Ok(res) => UserListResponse {
                users: res.users,
                total_pages: res.pages,
            },
        }
    }

    pub fn go_to_page(self: &Rc<Self>, page: u32) {
        let state = self;
        state.loader.load(clone!(state => async move {
            state.active_page.set(page);
            state.load_users().await;
        }));
    }

    pub fn navigate_to(self: &Rc<Self>, route: AdminUsersRoute) {
        self.route.set(route.clone());
        Route::Admin(AdminRoute::Users(route)).push_state();
    }

    // pub async fn get_user(self: Rc<Self>, user_id: UserId) -> Rc<EditableUser> {
    //     let user = self
    //         .users
    //         .lock_ref()
    //         .iter()
    //         .find(|user| user.id == user_id)
    //         .cloned();
    //     match user {
    //         Some(user) => user,
    //         None => Rc::new(self.load_user(&user_id).await),
    //     }
    // }

    // async fn load_user(self: &Rc<Self>, user_id: &UserId) -> EditableUser {
    //     match endpoints::user::api_with_auth(GetProfilePath(user_id.clone()), None).await {
    //         Ok(user) => user.into(),
    //         Err(_) => {
    //             todo!()
    //         }
    //     }
    // }
}

#[derive(Clone, Debug)]
struct UserListResponse {
    users: Vec<UserResponse>,
    total_pages: u32,
}
