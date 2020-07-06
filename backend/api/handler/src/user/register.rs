use ji_cloud_shared::{
    auth::{RegisterRequest, RegisterError, SigninSuccess},
    api::result::ResultResponse,
};
use super::queries::{get_by_email, get_by_id};
use sqlx::postgres::PgPool;

//the user_id is already validated in terms of firebase auth

//register handler doesn't use the usual wrapper since it needs to set the header
pub async fn handle_register(user_id:String, req:RegisterRequest, db:PgPool) -> Result<(), RegisterError> { 


    if get_by_id(&db, &user_id).is_some() {
        Err(RegisterError::TakenId)
    } else if req.display_name.is_empty() {
        Err(RegisterError::EmptyDisplayname)
    } else if req.first_name.is_empty() {
        Err(RegisterError::EmptyFirstname)
    } else if req.last_name.is_empty() {
        Err(RegisterError::EmptyLastname)
    } else if get_by_email(&db, &req.email).is_some() {
        Err(RegisterError::TakenEmail)
    } else {
        Ok(())
    }

    /*
    insert_into(users)
        .values((
            id.eq(&user_id),
            display_name.eq(req.display_name),
            first_name.eq(req.first_name),
            last_name.eq(req.last_name),
            email.eq(req.email),
        ))
        .execute(&db)
        .map_err(|_| DbQueryError::rejection())?;
        */
}
