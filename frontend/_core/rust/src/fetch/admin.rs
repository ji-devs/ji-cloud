pub mod category {
    use shared::{
        api::endpoints::{ApiEndpoint, category::*},
        domain::category::*
    };
    use crate::{
        path::api_url,
        fetch::{api_with_auth, api_with_auth_no_result, FetchResult}
    };
    use uuid::Uuid;
    use wasm_bindgen::UnwrapThrowExt;

    //needs to be a function due to orphan rule
    fn category_id_from_str(id:&str) -> CategoryId {
        CategoryId(uuid_from_str(id))
    }
    //needs to be a function due to orphan rule
    fn uuid_from_str(id:&str) -> Uuid {
        Uuid::parse_str(id).unwrap_throw()
    }

    pub async fn get_all() -> FetchResult < <Get as ApiEndpoint>::Res, <Get as ApiEndpoint>::Err> {
        let req:<Get as ApiEndpoint>::Req = GetCategoryRequest {
            ids: Vec::new(), 
            scope: Some(CategoryTreeScope::Decendants)
        };
        
        let query = serde_qs::to_string(&req).unwrap_throw();

        let path = api_url(&format!("{}?{}", Get::PATH, query)); 

        api_with_auth::<_,_,()>(&path, Get::METHOD, None).await
    }

    pub async fn create(name:String, parent_id: Option<&str>) -> FetchResult < <Create as ApiEndpoint>::Res, <Create as ApiEndpoint>::Err> {

        let req:<Create as ApiEndpoint>::Req = CreateCategoryRequest {
            name,
            parent_id: parent_id.map(category_id_from_str)
        };
        api_with_auth(&api_url(Create::PATH), Create::METHOD, Some(req)).await
    }

    pub async fn rename(id:&str, name:String) -> FetchResult < <Update as ApiEndpoint>::Res, <Update as ApiEndpoint>::Err> {
        let path = Update::PATH.replace("{id}",id);
        
        let req:<Update as ApiEndpoint>::Req = UpdateCategoryRequest {
            name: Some(name),
            parent_id: None,
            index: None
        };
        api_with_auth(&api_url(&path), Update::METHOD, Some(req)).await
    }

    pub async fn move_before_sibling(id:&str, before_sibling_index:u16) -> FetchResult < <Update as ApiEndpoint>::Res, <Update as ApiEndpoint>::Err> {
        let path = Update::PATH.replace("{id}",id);
        
        let req:<Update as ApiEndpoint>::Req = UpdateCategoryRequest {
            name: None,
            parent_id: None, 
            index: Some(before_sibling_index) 
        };
        api_with_auth(&api_url(&path), Update::METHOD, Some(req)).await
    }
    pub async fn move_before_sibling_new_parent(id:&str, parent_id: &str, before_sibling_index:u16) -> FetchResult < <Update as ApiEndpoint>::Res, <Update as ApiEndpoint>::Err> {
        let path = Update::PATH.replace("{id}",id);
        
        let req:<Update as ApiEndpoint>::Req = UpdateCategoryRequest {
            name: None,
            parent_id: Some(Some(category_id_from_str(parent_id))),
            index: Some(before_sibling_index) 
        };
        api_with_auth(&api_url(&path), Update::METHOD, Some(req)).await
    }

    pub async fn move_before_sibling_root(id:&str, before_sibling_index:u16) -> FetchResult < <Update as ApiEndpoint>::Res, <Update as ApiEndpoint>::Err> {
        let path = Update::PATH.replace("{id}",id);
        
        let req:<Update as ApiEndpoint>::Req = UpdateCategoryRequest {
            name: None,
            parent_id: Some(None),
            index: Some(before_sibling_index) 
        };
        api_with_auth(&api_url(&path), Update::METHOD, Some(req)).await
    }

    pub async fn move_end(id:&str, parent_id:&str) -> FetchResult < <Update as ApiEndpoint>::Res, <Update as ApiEndpoint>::Err> {
        let path = Update::PATH.replace("{id}",id);
        
        let req:<Update as ApiEndpoint>::Req = UpdateCategoryRequest {
            name: None,
            parent_id: Some(Some(category_id_from_str(parent_id))),
            index: None 
        };
        api_with_auth(&api_url(&path), Update::METHOD, Some(req)).await
    }

    pub async fn move_end_root(id:&str) -> FetchResult < <Update as ApiEndpoint>::Res, <Update as ApiEndpoint>::Err> {
        let path = Update::PATH.replace("{id}",id);
        
        let req:<Update as ApiEndpoint>::Req = UpdateCategoryRequest {
            name: None,
            parent_id: Some(None),
            index: None 
        };
        api_with_auth(&api_url(&path), Update::METHOD, Some(req)).await
    }

    pub async fn delete(id:&str) -> FetchResult < <Delete as ApiEndpoint>::Res, <Delete as ApiEndpoint>::Err> {
        let path = Delete::PATH.replace("{id}",id);

        api_with_auth_no_result::<_,()>(&api_url(&path), Delete::METHOD, None).await
    }
}

/*
 *pub struct UpdateCategoryRequest {
    pub name: Option<String>,
    /// If None, don't change parents. If Some, change parent to the given CategoryId (or null).
    #[serde(deserialize_with = "deserialize_optional_field")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub parent_id: Option<Option<CategoryId>>,
    /// If None, don't change index. If Some move to _before_ whatever has the given index (ie, 0 moves to the start).
    /// Will cause an error if you try to move to past the end of the parent.
    ///
    /// If None and parent_id is Some(...) it will append to the end of the new parent.
    pub index: Option<u16>,
}
*/
