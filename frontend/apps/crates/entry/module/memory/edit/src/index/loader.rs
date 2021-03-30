use components::module::page::ModulePageKind;
use futures_signals::{
    map_ref,
    signal::{Mutable, Signal, SignalExt},
    signal_vec::{MutableVec, SignalVec, SignalVecExt}
};
use std::rc::Rc;
use crate::{
    debug,
    data::{raw, state::{Step, State}, raw::GameData as RawData},
};
use std::future::Future;
use components::module::page::StateLoader;
use shared::{
    api::endpoints::{ApiEndpoint, self, module::*},
    error::{EmptyError, MetadataNotFound},
    domain::jig::{*, module::*},
};
use utils::prelude::*;

pub struct PageLoader { 
    pub jig_id: JigId,
    pub module_id: ModuleId 
}
impl StateLoader<RawData, State> for PageLoader {
    type FutureState = impl Future<Output = Option<Rc<State>>>;

    fn load_state(&self) -> Self::FutureState { 
        let jig_id = self.jig_id.clone();
        let module_id = self.module_id.clone();
        async move {
            let game_data = match debug::settings().data {
                None => {
                    let path = Get::PATH.replace("{id}",&module_id.0.to_string());

                    match api_with_auth::<ModuleResponse, EmptyError, ()>(&path, Get::METHOD, None).await {
                        Ok(resp) => {
                            resp.module.body.map(|value| {
                                serde_json::from_value(value).unwrap_ji()
                            })
                        },
                        Err(_) => {
                            panic!("error loading module!")
                        }
                    }
                },
                Some(game_data) => game_data
            };

            let state = Rc::new(State::new(jig_id, module_id, game_data));
            Some(state)
        }
    }

    fn derive_state(&self, data:RawData) -> Rc<State> { 
        Rc::new(State::new(self.jig_id.clone(), self.module_id.clone(), Some(data)))
    }
}
