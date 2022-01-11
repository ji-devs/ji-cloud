use uuid::{Uuid, v1::{Timestamp, Context}};
use std::time::{Duration, SystemTime};

const NODE_ID:&[u8] = &[0,1,2,3,4,5];

static CONTEXT:Context = Context::new(0);

pub fn new_id() -> Uuid {
    let now = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
    let ts = Timestamp::from_unix(&CONTEXT, now.as_secs(), now.subsec_nanos());
    // log::info!("{:?} {} {}", ts, now.as_secs(), now.subsec_nanos());
    Uuid::new_v1(ts, NODE_ID).unwrap()
}