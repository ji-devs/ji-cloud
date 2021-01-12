use std::rc::Rc;
use std::future::Future;

pub trait StateLoader <RawData, State> {
    type FutureState: Future<Output = Option<Rc<State>>>;
    
    fn load_state(&self) -> Self::FutureState;
    fn derive_state(&self, data: RawData) -> Rc<State>;
}
