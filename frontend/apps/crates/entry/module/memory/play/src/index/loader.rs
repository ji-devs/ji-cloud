use components::module::page::ModulePageKind;
use futures_signals::{
    map_ref,
    signal::{Mutable, Signal, SignalExt},
    signal_vec::{MutableVec, SignalVec, SignalVecExt}
};
use std::rc::Rc;
use crate::{
    debug,
    data::{raw, state::{State}, raw::ModuleData as RawData},
};

use components::font_loader::{FontLoader, Font};
use std::future::Future;
use components::module::page::StateLoader;
use shared::{
    api::endpoints::{ApiEndpoint, self, jig::module::*},
    error::{EmptyError, MetadataNotFound},
    domain::jig::{*, module::{*, body::Body}},
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

        debug::init(jig_id.clone(), module_id.clone());

        async move {
            let game_data = match debug::settings().data.as_ref() {
                None => {

                    let path = Get::PATH
                        .replace("{id}",&jig_id.0.to_string())
                        .replace("{module_id}",&module_id.0.to_string());

                    match api_with_auth::<ModuleResponse, EmptyError, ()>(&path, Get::METHOD, None).await {

                        Ok(resp) => {
                            resp.module.body.map(|resp| {
                                match resp {
                                    Body::MemoryGame(body) => body,
                                    _ => panic!("wrong module body kind!!")
                                }
                            })
                            .expect_ji("module data must exist for player!")
                        },
                        Err(_) => {
                            panic!("error loading module!")
                        }
                    }
                },
                Some(game_data) => game_data.clone()
            };
            
            //TODO - just load the fonts for this theme
            FontLoader::new().load_all().await;

            let state = Rc::new(State::new(jig_id, module_id, game_data));
            Some(state)
        }
    }

    fn derive_state(&self, data:RawData) -> Rc<State> { 
        Rc::new(State::new(self.jig_id.clone(), self.module_id.clone(), data))
    }
}
