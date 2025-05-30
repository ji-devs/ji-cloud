use std::{collections::HashMap, rc::Rc};

use components::qr_dialog::{QrDialog, QrDialogCallbacks};
use dominator::clone;
use futures::{future::try_join_all, join};
use shared::{
    api::endpoints,
    domain::{
        asset::AssetType,
        jig::{
            codes::{
                JigCode, JigCodePath, JigCodeSessionsPath, JigPlaySessionModule,
                JigPlaySessionModuleGetPointsEarned,
            },
            JigGetLivePath,
        },
        module::{ModuleGetLivePath, ModuleResponse, StableModuleId},
    },
};
use utils::{
    bail_on_err, date_formatters, error_ext::ErrorExt, js_wrappers::download_url,
    prelude::ApiEndpointExt, unwrap::UnwrapJiExt,
};
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::spawn_local;

use super::{CodeSessions, JigWithModules};

impl CodeSessions {
    pub fn load_data(self: &Rc<Self>) {
        let state = self;
        spawn_local(clone!(state => async move {
            join!(
                state.load_code(),
                state.load_jig(),
                state.load_report(),
            );
        }));
    }

    async fn load_code(self: &Rc<Self>) {
        let code_response =
            endpoints::jig::codes::GetJigCode::api_with_auth(JigCodePath(self.code), None)
                .await
                .toast_on_err();
        let code_response = bail_on_err!(code_response);
        self.code_response.set(Some(code_response));
    }

    async fn load_jig(self: &Rc<Self>) {
        let jig = endpoints::jig::GetLive::api_with_auth(JigGetLivePath(self.jig_id.clone()), None)
            .await
            .toast_on_err();
        let jig = bail_on_err!(jig);
        let modules = try_join_all(jig.jig_data.modules.iter().map(|module| {
            endpoints::module::GetLive::api_with_auth(
                ModuleGetLivePath(AssetType::Jig, module.id.clone()),
                None,
            )
        }))
        .await
        .toast_on_err();
        let modules: Vec<ModuleResponse> = bail_on_err!(modules);
        let modules = modules
            .into_iter()
            .map(|module| (module.module.stable_id, module))
            .collect();

        self.jig.set(Some(JigWithModules { jig, modules }));
    }

    async fn load_report(self: &Rc<Self>) {
        let res = endpoints::jig::codes::JigCodeSessions::api_with_auth(
            JigCodeSessionsPath(self.code),
            None,
        )
        .await
        .toast_on_err();
        let res = bail_on_err!(res);
        self.infos.lock_mut().extend(res.sessions);
    }

    pub fn show_qr_code(self: &Rc<Self>) {
        let state = self;
        if let Some(code_response) = state.code_response.lock_ref().as_ref() {
            let qr_dialog = QrDialog::new_jig_code(
                code_response.index,
                state
                    .jig
                    .lock_ref()
                    .as_ref()
                    .unwrap()
                    .jig
                    .jig_data
                    .display_name
                    .clone(),
                code_response.name.clone(),
                QrDialogCallbacks::new(clone!(state => move || {
                    state.qr_dialog.set(None);
                })),
            );
            self.qr_dialog.set(Some(qr_dialog));
        }
    }

    fn generate_csv_string(&self) -> String {
        let mut wtr = csv::WriterBuilder::new().from_writer(vec![]);
        if let Some(jig) = self.jig.lock_ref().as_ref() {
            let mut headers = vec!["Student's Name", "Started", "Ended"];
            headers.extend(
                jig.jig
                    .jig_data
                    .modules
                    .iter()
                    .map(|module| module.kind.display_name()),
            );
            headers.extend(&["Total Percent", "Total Points"]);
            wtr.write_record(&headers).unwrap_ji();

            for session in self.infos.get_cloned() {
                let total_points_earned = session.info.as_ref().map(|i| i.get_points_earned());
                let sessions = session
                    .info
                    .unwrap()
                    .modules
                    .into_iter()
                    .map(|module| {
                        let stable_module_id = module.stable_module_id();
                        (stable_module_id, module)
                    })
                    .collect::<HashMap<StableModuleId, JigPlaySessionModule>>();

                let mut row = vec![
                    session.players_name.clone().unwrap_or_default(),
                    date_formatters::year_month_day_hour_minute(&session.started_at),
                    session
                        .finished_at
                        .map(|t| date_formatters::year_month_day_hour_minute(&t))
                        .unwrap_or_default(),
                ];

                // add modules
                row.extend(jig.jig.jig_data.modules.iter().map(|module| {
                    let stable_module_id = module.stable_id;
                    sessions
                        .get(&stable_module_id)
                        .map(|session| session.get_points_earned().to_string())
                        .unwrap_or_default()
                }));

                row.extend([
                    total_points_earned
                        .as_ref()
                        .map(|p| format!("{}%", p.percent()))
                        .unwrap_or_default(),
                    total_points_earned
                        .map(|p| p.to_string())
                        .unwrap_or_default(),
                ]);

                wtr.write_record(row).unwrap();
            }
        }

        let data = String::from_utf8(wtr.into_inner().unwrap_ji()).unwrap_ji();
        data
    }

    pub fn export_sessions(&self) {
        let data = self.generate_csv_string();
        let data = JsValue::from_str(&data);
        let blob_property_bag = web_sys::BlobPropertyBag::new();
        blob_property_bag.set_type("text/csv");
        let blob = web_sys::Blob::new_with_str_sequence_and_options(
            &js_sys::Array::from_iter(vec![data]),
            &blob_property_bag,
        )
        .unwrap_ji();
        let url = web_sys::Url::create_object_url_with_blob(&blob).unwrap_ji();
        let filename = filename(
            self.code,
            self.code_response.lock_ref().as_ref().unwrap().name.clone(),
            self.jig
                .lock_ref()
                .as_ref()
                .unwrap()
                .jig
                .jig_data
                .display_name
                .clone(),
        );
        download_url(&filename, &url);
    }
}

fn filename(code: JigCode, code_name: Option<String>, jig_name: String) -> String {
    let jig_name = jig_name.replace(" ", "-");
    let code_name = code_name.map(|n| n.replace(" ", "-"));
    let mut file_label = format!("{}_{}", code.to_string(), jig_name);
    if let Some(code_name) = code_name {
        file_label = format!("{}_{}", code_name, file_label);
    }
    file_label
}
