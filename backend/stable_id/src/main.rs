// This was only needed once, after added the `stable_id`. Only exists for records purposes.

use ji_cloud_api::db;
use shared::domain::jig::JigId;
use shared::domain::module::LiteModule;
use shared::domain::module::ModuleId;
use shared::domain::module::ModuleKind;
use shared::domain::module::StableModuleId;

#[derive(Clone, Debug)]
pub struct Jig {
    pub jig_id: JigId,
    pub draft_id: JigId,
    pub live_id: JigId,
    pub draft_display_name: String,
    pub live_display_name: String,
    pub draft_modules: Vec<LiteModule>,
    pub live_modules: Vec<LiteModule>,
}
impl Jig {
    pub fn should_be_updated(&self) -> bool {
        if self.draft_modules.is_empty() {
            return false;
        }
        if self.draft_modules.len() != self.live_modules.len() {
            return false;
        }
        self.draft_modules.iter().enumerate().all(|(i, module)| {
            self.live_modules[i].kind == module.kind
                && self.live_modules[i].stable_id != module.stable_id
        })
    }
}

async fn get_jigs(db_pool: sqlx::PgPool) -> Vec<Jig> {
    let rows = sqlx::query!(
        // language=SQL
        r#"
            select
                jig.id                              as "jig_id!: JigId",
                draft_id                            as "draft_id!: JigId",
                live_id                             as "live_id!: JigId",
                jig_data_draft.display_name         as "draft_display_name!",
                jig_data_live.display_name          as "live_display_name!",
                array(
                    select row (jig_data_module.id, jig_data_module.stable_id, kind)
                    from jig_data_module
                    where jig_data_id = jig_data_draft.id
                    order by "index"
                ) as "draft_modules!: Vec<(ModuleId, StableModuleId, ModuleKind)>",
                array(
                    select row (jig_data_module.id, jig_data_module.stable_id, kind)
                    from jig_data_module
                    where jig_data_id = jig_data_live.id
                    order by "index"
                ) as "live_modules!: Vec<(ModuleId, StableModuleId, ModuleKind)>"
            from jig
            LEFT JOIN jig_data as jig_data_draft
            ON jig.draft_id = jig_data_draft.id
            LEFT JOIN jig_data as jig_data_live
            ON jig.live_id = jig_data_live.id;
        "#,
    )
    .fetch_all(&db_pool)
    .await
    .unwrap();

    rows.into_iter()
        .map(|row| Jig {
            jig_id: row.jig_id,
            draft_id: row.draft_id,
            live_id: row.live_id,
            draft_display_name: row.draft_display_name,
            live_display_name: row.live_display_name,
            draft_modules: row
                .draft_modules
                .into_iter()
                .map(|(id, stable_id, kind)| LiteModule {
                    id,
                    stable_id,
                    kind,
                    is_complete: false,
                })
                .collect(),
            live_modules: row
                .live_modules
                .into_iter()
                .map(|(id, stable_id, kind)| LiteModule {
                    id,
                    stable_id,
                    kind,
                    is_complete: false,
                })
                .collect(),
        })
        .collect()
}

async fn update_modules(db_pool: sqlx::PgPool, module_id: ModuleId, new_stable_id: StableModuleId) {
    sqlx::query!(
        "update jig_data_module set stable_id = $2 where id = $1",
        module_id.0,
        new_stable_id.0
    )
    .execute(&db_pool)
    .await
    .unwrap();
}

#[tokio::main]
async fn main() {
    let db_pool = db::get_pool(
        sqlx::postgres::PgConnectOptions::new()
            .host("")
            .port(0)
            .username("")
            .password("")
            .database(""),
    )
    .await
    .unwrap();

    let jigs = get_jigs(db_pool.clone()).await;

    let mut tasks = jigs.into_iter().filter_map(|jig| {
        let db_pool = db_pool.clone();
        if jig.should_be_updated() {
            Some(async move {
                println!("Updating jig: {:#?}", jig);
                for (draft_module, live_module) in
                    jig.draft_modules.iter().zip(jig.live_modules.iter())
                {
                    update_modules(db_pool.clone(), draft_module.id, live_module.stable_id).await;
                }
            })
        } else {
            None
        }
    });

    loop {
        let mut inner_tasks = vec![];
        for _ in 0..1000 {
            match tasks.next() {
                Some(t) => inner_tasks.push(t),
                None => {}
            }
        }
        if inner_tasks.is_empty() {
            break;
        }
        futures::future::join_all(inner_tasks).await;
    }
}
