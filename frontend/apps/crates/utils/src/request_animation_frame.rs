use futures::channel::oneshot;

use crate::unwrap::UnwrapJiExt;

// should this be moved to gloo::render? Maybe with a stream version as well?

pub async fn before_next_frame() -> f64 {
    let (sender, receiver) = oneshot::channel();

    let _handle = gloo::render::request_animation_frame(|res| {
        sender.send(res).unwrap_ji();
    });

    receiver.await.unwrap_ji()
}
