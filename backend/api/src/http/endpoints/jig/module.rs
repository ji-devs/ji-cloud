// use actix_web::{
//     web::{self, Data, Json, Path, ServiceConfig},
//     HttpResponse,
// };
// use shared::{
//     api::{endpoints::jig::module, ApiEndpoint},
//     domain::{
//         jig::{
//             module::{ModuleId, ModuleIdOrIndex, ModuleResponse},
//             JigId,
//         },
//         CreateResponse,
//     },
// };
// use sqlx::PgPool;
//
// use crate::{db, error, extractor::TokenUser};
//
// /// Create a new module on a draft JIG.
// async fn create(
//     db: Data<PgPool>,
//     auth: TokenUser,
//     parent: Path<JigId>,
//     req: Json<<module::Create as ApiEndpoint>::Req>,
// ) -> Result<HttpResponse, error::Auth> {
//     let parent_id = parent.into_inner();
//
//     db::jig::authz(&*db, auth.0.user_id, Some(parent_id)).await?;
//
//     let req = req.into_inner();
//
//     let (id, _index) = db::jig::module::create(&*db, parent_id, req.body).await?;
//
//     Ok(HttpResponse::Created().json(CreateResponse { id }))
// }
//
// /// Delete a module from a draft JIG.
// async fn delete(
//     db: Data<PgPool>,
//     auth: TokenUser,
//     path: web::Path<(JigId, ModuleId)>,
// ) -> Result<HttpResponse, error::Delete> {
//     let (parent_id, module) = path.into_inner();
//
//     db::jig::authz(&*db, auth.0.user_id, Some(parent_id)).await?;
//
//     db::jig::module::delete(&*db, parent_id, ModuleIdOrIndex::Id(module)).await?;
//
//     Ok(HttpResponse::NoContent().finish())
// }
//
// /// Update a module in a draft JIG.
// async fn update(
//     db: Data<PgPool>,
//     auth: TokenUser,
//     req: Option<Json<<module::Update as ApiEndpoint>::Req>>,
//     path: web::Path<(JigId, ModuleId)>,
// ) -> Result<HttpResponse, error::NotFound> {
//     let (parent_id, module) = path.into_inner();
//
//     db::jig::authz(&*db, auth.0.user_id, Some(parent_id)).await?;
//
//     let req = req.map_or_else(Default::default, Json::into_inner);
//     let exists = db::jig::module::update(
//         &*db,
//         parent_id,
//         ModuleIdOrIndex::Id(module),
//         req.body.as_ref(),
//         req.index,
//         req.is_complete,
//     )
//     .await?;
//
//     if !exists {
//         return Err(error::NotFound::ResourceNotFound);
//     }
//
//     Ok(HttpResponse::NoContent().finish())
// }
//
// /// Get a module from a live JIG.
// async fn get_live(
//     db: Data<PgPool>,
//     path: web::Path<(JigId, ModuleId)>,
// ) -> Result<Json<<module::Get as ApiEndpoint>::Res>, error::NotFound> {
//     let (parent_id, module) = path.into_inner();
//
//     let module = db::jig::module::get(&db, parent_id, ModuleIdOrIndex::Id(module))
//         .await?
//         .ok_or(error::NotFound::ResourceNotFound)?;
//
//     Ok(Json(ModuleResponse { module }))
// }
// /// Get a module from a draft JIG.
// async fn get_draft(
//     db: Data<PgPool>,
//     path: web::Path<(JigId, ModuleId)>,
// ) -> Result<Json<<module::Get as ApiEndpoint>::Res>, error::NotFound> {
//     let (parent_id, module) = path.into_inner();
//
//     let module = db::jig::module::get(&db, parent_id, ModuleIdOrIndex::Id(module))
//         .await?
//         .ok_or(error::NotFound::ResourceNotFound)?;
//
//     Ok(Json(ModuleResponse { module }))
// }
//
// pub fn configure(cfg: &mut ServiceConfig) {
//     cfg.route(
//         module::GetLive::PATH,
//         module::GetLive::METHOD.route().to(get_live),
//     )
//     .route(
//         module::GetDraft::PATH,
//         module::GetDraft::METHOD.route().to(get_draft),
//     )
//     .route(
//         module::Create::PATH,
//         module::Create::METHOD.route().to(create),
//     )
//     .route(
//         module::Update::PATH,
//         module::Update::METHOD.route().to(update),
//     )
//     .route(
//         module::Delete::PATH,
//         module::Delete::METHOD.route().to(delete),
//     );
// }

use actix_web::web::ServiceConfig;
pub fn configure(cfg: &mut ServiceConfig) {}
