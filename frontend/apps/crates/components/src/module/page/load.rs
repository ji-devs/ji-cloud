use std::rc::Rc;
use std::future::Future;

pub trait StateLoader <RawData, State> {
    type FutureState: Future<Output = Option<Rc<State>>>;

    fn load_state(&self) -> Self::FutureState;
    
    //This is generally used in "preview" mode
    //So that the current state can be directly passed
    //And avoids potential cache/race conditions on the server
    fn derive_state(&self, data: RawData) -> Rc<State>;
}
